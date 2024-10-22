use neon::prelude::*;
use neon::context::MethodContext;
use neon::result::Throw;
use crossbeam::channel::{
    Receiver,
    RecvError,
    bounded,
    unbounded,
};
use ctp::*;
use ctp::api::*;
use ctp::api::opts::*;

use crate::OnSignal;
use super::*;

type FnContext<'a> = MethodContext<'a, JsCTPTrader>;
type FnResult<'a> = Result<Handle<'a, JsValue>, Throw>;

#[inline]
pub fn register_front<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let front = ctx.argument::<JsString>(0)?;

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        // 注册前置地址
        trader.register_front(front.value());
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn subscribe_private_topic<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let flag = ctx.argument::<JsString>(0)?.value();

    let flag = match &flag[..] {
        "resume" => TopicTertFlag::Resume,
        "quick" => TopicTertFlag::Quick,
        _ => TopicTertFlag::Restart,
    };

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        trader.subscribe_private_topic(flag);
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn subscribe_public_topic<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let flag = ctx.argument::<JsString>(0)?.value();

    let flag = match &flag[..] {
        "resume" => TopicTertFlag::Resume,
        "quick" => TopicTertFlag::Quick,
        _ => TopicTertFlag::Restart,
    };

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        trader.subscribe_public_topic(flag);
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn init<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        // 初始化
        trader.init();
    });

    Ok(ctx.undefined().upcast())
}


#[inline]
pub fn req_user_login<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let mut this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let broker_id = field_object
        .get(&mut ctx, "broker_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let user_id = field_object
        .get(&mut ctx, "user_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let password = field_object
        .get(&mut ctx, "password")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let app_id = field_object
        .get(&mut ctx, "app_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let auth_token = field_object
        .get(&mut ctx, "auth_token")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();

    let (tx, rx) = bounded(1);

    ctx.borrow_mut(&mut this, |mut trader| {
        trader.auth.broker_id.replace(broker_id.clone());
        trader.auth.user_id.replace(user_id.clone());
        trader.auth.password.replace(password.clone());
        trader.auth.app_id.replace(app_id.clone());
        trader.auth.auth_token.replace(auth_token.clone());

        let trader = &trader.trader;

        let mut auth_field = CThostFtdcReqAuthenticateField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&user_id)
            .with_auth_code(&auth_token)
            .with_app_id(&app_id)
            .build();

        trader.req_authenticate(&mut auth_field, 0).unwrap();

        trader.on_rsp_authenticate(move |api, _, error, _, _| {
            if error.ErrorID == 0 {
                log::info!("鉴权成功: {}", error);
                let mut api = api.lock().unwrap();
                let mut login_field = CThostFtdcReqUserLoginField::new()
                    .with_broker_id(&broker_id)
                    .with_user_id(&user_id)
                    .with_password(&password)
                    .build();
                api.req_user_login(&mut login_field, 0)
            } else {
                log::error!("鉴权失败: {}", error);
                panic!()
            }
        });

        // 账户登陆
        trader.on_rsp_user_login(move |_, _, error, _, _| {
            signal!(tx, Some(()), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_qry_investor_position<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let callback = ctx.argument::<JsFunction>(0)?;
    let (tx, rx) = unbounded();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        // 请求持仓
        trader.req_qry_investor_position(
            &mut CThostFtdcQryInvestorPositionField::new()
        , 0).unwrap();

        trader.on_rsp_qry_investor_position(move |_, position, _, _, last| {
            if let Some(position) = position {
                tx.send(Some((*position, last))).unwrap();
            } else {
                tx.send(None).unwrap();
            }

            Ok(())
        });
    });

    struct MarketTask {
        rx: Receiver<Option<(CThostFtdcInvestorPositionField, bool)>>,
    }

    impl Task for MarketTask {
        type Output = Vec<CThostFtdcInvestorPositionField>;
        type Error = RecvError;
        type JsEvent = JsArray;

        fn perform(&self) -> Result<Self::Output, Self::Error> {
            let mut r = vec![];
            loop {
                if let Some((position, last)) = self.rx.recv()? {
                    r.push(position);

                    if last {
                        break;
                    }
                } else {
                    break;
                }
            }

            Ok(r)
        }

        fn complete(self, mut ctx: TaskContext, event: Result<Self::Output, Self::Error>)
            -> JsResult<Self::JsEvent>
        {
            match event {
                Ok(vec) => {
                    let result = JsArray::new(&mut ctx, vec.len() as u32);
                    for (i, obj) in vec.iter().enumerate() {
                        let obj = neon_serde::to_value(&mut ctx, obj)?;
                        result.set(&mut ctx, i as u32, obj).unwrap();
                    }

                    Ok(result)
                }
                Err(e) => ctx.throw_error(format!("{}", e))
            }
        }
    }

    MarketTask {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_qry_instrument<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let (tx, rx) = unbounded();

    let instrument_id = field_object
        .get(&mut ctx, "instrument_id")?.downcast::<JsString>()
        .or_throw(&mut ctx)?.value();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        trader.req_qry_instrument(&mut CThostFtdcQryInstrumentField::new()
            .with_instrument_id(&instrument_id), 0).unwrap();

        trader.on_rsp_qry_instrument(move |_, instrument, _, _, _| {
            signal!(tx, instrument.map(|i| *i))
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())

}

#[inline]
pub fn req_order_insert<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let (tx, rx) = unbounded();

    let instrument_id = field_object
        .get(&mut ctx, "instrument_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let volume = field_object
        .get(&mut ctx, "volume")?
        .downcast::<JsNumber>()
        .or_throw(&mut ctx)?
        .value();
    let limit_price = field_object
        .get(&mut ctx, "limit_price")?
        .downcast::<JsNumber>()
        .or_throw(&mut ctx)?
        .value();
    let position = field_object
        .get(&mut ctx, "position")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let direction = field_object
        .get(&mut ctx, "direction")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let mode = field_object
        .get(&mut ctx, "mode")?;
        // .downcast::<JsString>()
        // .or_throw(&mut ctx)?
        // .value();
    // let investor_id = field_object
    //     .get(&mut ctx, "investor_id")?
    //     .downcast::<JsString>()
    //     .or_throw(&mut ctx)?
    //     .value();

    ctx.borrow(&this, |trader| {
        let broker_id = trader.auth.broker_id.clone().unwrap_or_default();
        let user_id = trader.auth.user_id.clone().unwrap_or_default();
        let trader = &trader.trader;

        let mut field = CThostFtdcInputOrderField::new()
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_instrument_id(&instrument_id)
            .with_min_volume(1)
            .with_total_volume(volume as i32)
            .with_broker_id(broker_id)
            .with_user_id(&user_id)
            .with_investor_id(&user_id)
            .with_limit_price(limit_price)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_time_condition(OrderTimeCondition::Immediately)
            .build();

        match &position[..] {
            "open" => {
                field.with_comb_offset_flag(OrderOffsetFlag::Open);
            },
            "close" => {
                field.with_comb_offset_flag(OrderOffsetFlag::Close);
            },
            _ => {
                signal!(tx, None).unwrap();
            }
        }

        match &direction[..] {
            "buy" => {
                field.with_direction(OrderDirection::Buy);
            }
            "sell" => {
                field.with_direction(OrderDirection::Sell);
            }
            _ => {
                signal!(tx, None).unwrap();
            }
        }

        trader.on_err_rtn_order_insert(move |_, _, error| {
            log::error!("{}", error);
            Ok(())
        });

        trader.req_order_insert(&field , 0).unwrap();

        trader.on_rsp_order_insert(move |_, order, error, _, _| {
            signal!(tx, Some(*order), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);
    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_settlement_info_confirm<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let (tx, rx) = unbounded();

    let broker_id = field_object
        .get(&mut ctx, "broker_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let investor_id = field_object
        .get(&mut ctx, "investor_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;

        let mut field = CThostFtdcSettlementInfoConfirmField::new()
            .with_broker_id(&broker_id)
            .with_investor_id(&investor_id)
            .build();

        trader.req_settlement_info_confirm(
            &mut field
        , 0).unwrap();

        trader.on_rsp_settlement_info_confirm(move |_, confirm, error, _, _| {
            signal!(tx, Some(*confirm), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_qry_trading_account<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let (tx, rx) = unbounded();

    let broker_id = field_object
        .get(&mut ctx, "broker_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();
    let investor_id = field_object
        .get(&mut ctx, "investor_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        let mut field = CThostFtdcQryTradingAccountField::new()
            .with_broker_id(&broker_id)
            .with_investor_id(&investor_id)
            .build();

        trader.req_qry_trading_account(
            &mut field
        , 0).unwrap();

        trader.on_rsp_qry_trading_account(move |_, account, error, _, _| {
            if let Some(account) = account {
                signal!(tx, Some(*account))
            } else {
                signal!(tx, None)
            }
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_qry_depth_market_data<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let instrument_id = ctx.argument::<JsString>(0)?.value();
    let callback = ctx.argument::<JsFunction>(1)?;

    let (tx, rx) = unbounded();

    ctx.borrow(&this, |trader| {
        let trader = &trader.trader;
        let &mut field = CThostFtdcQryDepthMarketDataField::new()
            .with_instrument_id(instrument_id);

        trader.req_qry_depth_market_data(&field, 0).unwrap();

        trader.on_rsp_qry_depth_market_data(move |_, market, _, _, _| {
            signal!(tx, market.cloned())
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}
