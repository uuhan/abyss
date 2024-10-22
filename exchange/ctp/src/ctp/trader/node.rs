use super::*;
use nodex::prelude::*;

impl<'a> CTPTrader<'a> {
    pub fn class(env: NapiEnv) -> NapiResult<JsClass> {
        JsClass::new(
            env,
            "CTPTraderClass",
            move |mut this, params: JsObject| {
                let env = this.env();
                let front: JsString = params.get("front")?;
                let broker: JsString = params.get("broker")?;
                let app: JsString = params.get("app")?;
                let user: JsString = params.get("user")?;
                let password: JsString = params.get("password")?;
                let auth: JsString = params.get("auth")?;
                let log: JsValue = params.get("log")?;

                let trader = if log.is_string()? {
                    CTPTrader::builder()
                        .with_log_path(log.as_string()?.get()?)
                        .with_front(front.get()?)
                        .with_broker_id(broker.get()?)
                        .with_app_id(app.get()?)
                        .with_user_id(user.get()?)
                        .with_user_password(password.get()?)
                        .with_auth_token(auth.get()?)
                        .build()
                } else {
                    CTPTrader::builder()
                        .with_front(front.get()?)
                        .with_broker_id(broker.get()?)
                        .with_app_id(app.get()?)
                        .with_user_id(user.get()?)
                        .with_user_password(password.get()?)
                        .with_auth_token(auth.get()?)
                        .build()
                };

                this.wrap(trader, move |_, _| Ok(()))?;

                this.set(
                    "init",
                    env.func(move |this, cb: Function<JsUndefined>| {
                        let env = this.env();

                        let tsfn: NapiTsfn<()> = env.tsfn(
                            "ctp-trader-init",
                            cb,
                            move |_| Ok(()),
                            move |cb, _| {
                                cb.call(this, this)?;
                                Ok(())
                            },
                        )?;

                        if let Some(trader) = this.unwrap::<CTPTrader>()? {
                            if let Err(e) = trader.init_by_env(move |_| {
                                if let Err(e) = tsfn.non_blocking(()) {
                                    tracing::error!("trader init cb: {}", e);
                                }
                                tsfn.release().unwrap();
                                Ok(())
                            }) {
                                env.throw_error(format!("{}", e))?;
                            }
                        }

                        env.undefined()
                    })?,
                )?;

                // 获取交易日
                this.set(
                    "get_trading_day",
                    env.func(move |this, ()| {
                        let env = this.env();

                        if let Some(market) = this.unwrap::<CTPMarket>()? {
                            env.string(market.get_trading_day())
                        } else {
                            env.string("")
                        }
                    })?,
                )?;

                env.undefined()
            },
            [],
        )
    }
}
