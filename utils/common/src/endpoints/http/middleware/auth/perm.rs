use crate::config::PERMConfig;
use crate::endpoints::http::*;
use crate::error::Result;
use casbin::prelude::*;
use std::{collections::VecDeque, marker::PhantomData};

use std::iter::FromIterator;

use tide::{Middleware, Next, Request, Response};

pub struct CasbinMiddleware<User> {
    enforcer: Enforcer,
    _user_t: PhantomData<User>,
}

impl<User: HttpUserT> CasbinMiddleware<User> {
    pub async fn new(perm: &PERMConfig) -> Result<Self> {
        let m = DefaultModel::from_str(&perm.model).await?;

        let enforcer = if let Some(ref path) = perm.policy_path {
            let adapter = FileAdapter::new(path.clone());
            Enforcer::new(m, adapter).await?
        } else {
            let adapter = MemoryAdapter::default();
            let policy = perm
                .policy
                .as_ref()
                .expect("[http/perm] policy not provided.");
            let mut e = Enforcer::new(m, adapter).await?;
            for line in policy.split('\n') {
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                // NB: 格式如下:
                // p, admin , /*             , *
                let mut rule = line
                    .split(',')
                    .map(|s| s.trim())
                    .map(|s| s.to_string())
                    .collect::<VecDeque<_>>();
                let sec = rule.pop_front();
                let rule = Vec::from_iter(rule.into_iter());
                tracing::info!("[http/perm] {:?}, rule: {}", sec, rule.join(","));
                e.add_policy(rule).await?;
            }
            e
        };

        // for line in perm.policy.split('\n') {
        //     let line = line.trim();
        //     if line.is_empty() { continue; }
        //
        //     if let Some(tokens) = parse_csv_line(line) {
        //         let key = &tokens[0];
        //
        //         if let Some(ref sec) = key.chars().next().map(|x| x.to_string()) {
        //             if let Some(ast_map) = m.get_mut_model().get_mut(sec) {
        //                 if let Some(ast) = ast_map.get_mut(key) {
        //                     ast.policy.insert(tokens[1..].to_vec());
        //                 }
        //             }
        //         }
        //     }
        // }

        Ok(Self {
            enforcer,
            _user_t: Default::default(),
        })
    }
}

#[async_trait::async_trait]
impl<State: HttpStateT, User: HttpUserT> Middleware<State> for CasbinMiddleware<User> {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        if let Some(user) = req.ext::<User>() {
            tracing::debug!(role=?user.role(), "CasbinMiddleware<User>::handle()");
            let path = req.url().path();
            let method = req.method();
            for role in user.role().iter() {
                match self.enforcer.enforce((role, path, method)) {
                    Ok(pass) => {
                        tracing::debug!(
                            role=role,
                            path=path,
                            method=?method,
                            allow=pass,
                            "CasbinMiddleware<User>::handle()",
                        );

                        if pass {
                            let res = next.run(req).await;
                            return Ok(res);
                        }
                    }
                    Err(error) => {
                        tracing::error!(?error, "CasbinMiddleware<User>::handle()");
                    }
                }
            }
        }

        Ok(Response::builder(401).build())
    }
}
