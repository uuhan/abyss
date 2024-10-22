use super::*;
use abyss_promise::Promise;
use crossbeam_channel::unbounded;
use nodex::prelude::*;

impl<'a> CTPMarket<'a> {
    pub fn class(env: NapiEnv) -> NapiResult<JsClass> {
        let (tx, rx) = unbounded::<CThostFtdcDepthMarketDataField>();
        JsClass::new(
            env,
            "CTPMarketClass",
            move |mut this, params: JsObject| {
                let env = this.env();
                let front: JsString = params.get("front")?;
                let broker: JsString = params.get("broker")?;
                let user: JsString = params.get("user")?;
                let password: JsString = params.get("password")?;
                let log: JsValue = params.get("log")?;
                let tx = tx.clone();

                let mut builder = CTPMarket::builder();
                if log.is_string()? {
                    builder = builder.with_log_path(log.as_string()?.get()?);
                }

                let market = builder
                    .with_front(front.get()?)
                    .with_broker_id(broker.get()?)
                    .with_user_id(user.get()?)
                    .with_user_password(password.get()?)
                    .build();

                market
                    .on_rtn_depth_market_data(move |_, data| {
                        tx.send(*data).unwrap();
                        Ok(())
                    })
                    .on_rsp_sub_market_data(move |_, field, _, _, _| {
                        tracing::info!("subscribe: {}", ctp::s(&field.InstrumentID));
                        Ok(())
                    });

                this.wrap(market, move |_, _| Ok(()))?;

                let mut inited = false;
                this.set(
                    "init",
                    env.func(move |this, cb: JsFunction| {
                        let env = this.env();

                        if inited {
                            env.throw_error("CTPMarket instance is already inited.")?;
                            return env.undefined();
                        }

                        let tsfn: NapiTsfn<_> = env.tsfn(
                            "ctp-async-task",
                            cb,
                            move |_| Ok(()),
                            move |cb, data: ()| {
                                let env = cb.env();
                                let mut obj = env.object()?;
                                cb.call(env.object()?, ())?;
                                Ok(())
                            },
                        )?;

                        if let Some(market) = this.unwrap::<CTPMarket>()? {
                            if let Err(e) = market.init_by_env(move |api| {
                                tsfn.non_blocking(()).unwrap();
                                Ok(())
                            }) {
                                env.throw_error(format!("{}", e))?;
                            }
                        }

                        inited = true;
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
            [DescriptorMethodBuilder::new()
                .with_utf8name("subscribe")
                .with_method(move |this, (id, cb): (JsString, JsFunction)| {
                    let rx = rx.clone();
                    let env = this.env();
                    let id = id.get()?;

                    if let Some(market) = this.unwrap::<CTPMarket>()? {
                        if let Err(e) = market.subscribe_market_data(&[id]) {
                            env.throw_error(format!("{}", e))?;
                        } else {
                            let tsfn: NapiTsfn<_> = env.tsfn(
                                "ctp-async-task",
                                cb,
                                move |_| Ok(()),
                                move |cb, data: CThostFtdcDepthMarketDataField| {
                                    let env = cb.env();
                                    let mut obj = env.object()?;
                                    obj.set("id", env.string(ctp::s(&data.InstrumentID))?)?;
                                    obj.set("price", env.double(data.LastPrice)?)?;
                                    obj.set("day", env.string(ctp::s(&data.TradingDay))?)?;
                                    obj.set("time", env.string(ctp::s(&data.UpdateTime))?)?;
                                    cb.call(env.object()?, obj)?;
                                    Ok(())
                                },
                            )?;

                            tsfn.acquire()?;

                            Promise::new(move |promise| {
                                while let Ok(data) = rx.recv() {
                                    tsfn.blocking(data).unwrap();
                                }
                                tsfn.release().unwrap();
                                promise.resolve(());
                            });
                        }
                    } else {
                        env.throw_error("should call from new_instance.")?;
                    }

                    env.undefined()
                })
                .build()?],
        )
    }
}
