use super::*;
use ::redis::*;

#[derive(Clone)]
pub struct RedisSuber {
    pub client: Client,
    pub id: usize,
    pub con: Arc<Mutex<Connection>>,
    pub queue_name: String,
}

impl RedisSuber {
    pub fn new(url: impl AsRef<str>, queue_name: String) -> Result<Self> {
        let url = url.as_ref();
        log::info!("[RedisSuber] connect to: {}", url);

        let client = Client::open(url)?;
        let mut con = client.get_connection()?;
        let id: usize = redis::cmd("CLIENT").arg(&["ID"]).query(&mut con)?;
        log::info!("[RedisSuber] connected client id: {}", id);

        Ok(Self {
            client,
            queue_name,
            con: Arc::new(Mutex::new(con)),
            id,
        })
    }

    pub fn connection(&self) -> Result<Connection> {
        Ok(self.client.get_connection()?)
    }

    pub fn subscribe(env: NapiEnv) -> NapiResult<Function<JsUndefined>> {
        JsFunction::new(
            env,
            Some("subscribe"),
            move |this, worker: Function<JsUndefined>| {
                let env = this.env();
                let inner = if let Some(inner) = this.unwrap::<SuberInner>()? {
                    inner.clone()
                } else {
                    log::error!("unwrap nothing!");
                    return env.undefined();
                };

                let tsfn: NapiTsfn<(Task, SuberInner)> = env.tsfn(
                    "RedisSuber.subscribe",
                    worker,
                    move |_| Ok(()),
                    move |worker, (task, inner): (Task, SuberInner)| {
                        let env = worker.env();
                        let mut job = env.object()?;
                        log::debug!("[TSFN] worker fired: {:?}", task);

                        job.set("params", env.string(task.params)?)?;

                        job.set(
                            "publish",
                            env.func(move |this, result: JsString| {
                                let env = this.env();
                                let result = result.get()?;

                                let suber = match &*inner {
                                    DodoQueueSuber::Redis(suber) => suber,
                                    _ => {
                                        todo!()
                                    }
                                };

                                let subscribtion =
                                    format!("{}:{}:{}", suber.queue_name, task.client, task.serial);

                                loop {
                                    // FIXME: 这里暂时使用简单的重试逻辑, 确保 publish 一定成功
                                    let mut try_count = 0;
                                    if let Ok(mut con) = suber.connection() {
                                        if let Err(e) = redis::cmd("PUBLISH")
                                            .arg(&subscribtion)
                                            .arg(&result)
                                            .query::<()>(&mut con)
                                        {
                                            log::error!("[PUBLISH#{}] failed: {}", try_count, e);
                                            try_count += 1;

                                            if try_count > 20 {
                                                std::thread::sleep(
                                                    std::time::Duration::from_millis(500),
                                                );
                                            }
                                        } else {
                                            log::debug!("[PUBLISH] {} published.", subscribtion);
                                            break;
                                        }
                                    }
                                }

                                env.undefined()
                            })?,
                        )?;

                        worker.call(job, job)?;
                        Ok(())
                    },
                )?;

                std::thread::spawn(move || loop {
                    let suber = match &*inner {
                        DodoQueueSuber::Redis(suber) => suber,
                        _ => {
                            unreachable!()
                        }
                    };

                    let mut con = suber.con.lock();

                    let raw_task = match redis::cmd("BLPOP")
                        .arg(&suber.queue_name)
                        .arg(0usize)
                        .query(&mut *con)
                    {
                        RedisResult::<(String, String)>::Ok(task) => {
                            log::info!("[BLPOP] {:?}", task);
                            task.1
                        }
                        Err(e) => {
                            log::error!("[BLPOP] query error: {}", e);

                            'reconnect: loop {
                                match suber.connection() {
                                    Ok(new) => {
                                        *con = new;
                                        log::debug!("suber reconnected");
                                        break 'reconnect;
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "suber reconnect error: {}, reconnect in 1s",
                                            e
                                        );
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                    }
                                }
                            }

                            continue;
                        }
                    };

                    drop(con);

                    match serde_json::from_str::<Task>(&raw_task) {
                        Ok(task) => {
                            log::debug!("[TSFN] fire worker: {:?}", task);
                            tsfn.non_blocking((task, inner.clone())).unwrap();
                        }
                        Err(e) => {
                            log::error!("[BLOP] task parse error: {}, raw: {}", e, &raw_task);
                        }
                    }
                });

                env.undefined()
            },
        )
    }
}
