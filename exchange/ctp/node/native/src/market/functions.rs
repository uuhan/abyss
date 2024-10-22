use neon::prelude::*;
use neon::context::MethodContext;
use neon::result::Throw;
use crossbeam::channel::{
    bounded,
};
use ctp::*;
use ctp::api::*;

use crate::OnSignal;
use super::*;

type FnContext<'a> = MethodContext<'a, JsCTPMarket>;
type FnResult<'a> = Result<Handle<'a, JsValue>, Throw>;

#[inline]
pub fn register_front<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let front = ctx.argument::<JsString>(0)?;

    ctx.borrow(&this, |market| {
        log::info!("注册前置");
        // 注册前置地址
        market.market.register_front(front.value());
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn init<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    ctx.borrow(&this, |market| {
        log::info!("初始化行情");
        market.market.init();
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn release<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    ctx.borrow(&this, |market| {
        // 初始化
        market.market.init();
    });

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_user_login<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let broker_id = field_object
        .get(&mut ctx, "broker_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();

    let (tx, rx) = bounded(1);

    ctx.borrow(&this, |market| {
        let mut login_field = CThostFtdcReqUserLoginField::new()
            .with_broker_id(broker_id)
            .build();

        // 账户登陆
        market.market.req_user_login(&mut login_field, 0).unwrap();
    });

    ctx.borrow(&this, |market| {
        // 前置地址连接
        market.market.on_rsp_user_login(move |_, _, error, _, _| {
            signal!(tx, Some(()), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn req_user_logout<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let field_object = ctx.argument::<JsObject>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let broker_id = field_object
        .get(&mut ctx, "broker_id")?
        .downcast::<JsString>()
        .or_throw(&mut ctx)?
        .value();

    let (tx, rx) = bounded(1);

    ctx.borrow(&this, |market| {
        let mut logout_field = CThostFtdcUserLogoutField::new()
            .with_broker_id(broker_id)
            .build();

        // 账户登陆
        market.market.req_user_logout(&mut logout_field, 0).unwrap();
        market.market.on_rsp_user_login(move |_, _, error, _, _| {
            signal!(tx, Some(()), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn subscribe_market_data<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let instruments = ctx.argument::<JsArray>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let instruments: Vec<Handle<JsValue>> = instruments.to_vec(&mut ctx)?;
    let instruments: Vec<String> = instruments.iter().map(|ins| {
        ins.downcast::<JsString>().or_throw(&mut ctx).unwrap().value()
    }).collect();

    let (tx, rx) = bounded(1);

    ctx.borrow(&this, |market| {
        // 订阅合约
        market.market.subscribe_market_data(&instruments).unwrap();
        market.market.on_rsp_sub_market_data(move |_, _, error, _, _| {
            signal!(tx, Some(()), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}

#[inline]
pub fn unsubscribe_market_data<'a>(mut ctx: FnContext<'a>) -> FnResult<'a> {
    let this = ctx.this();
    let instruments = ctx.argument::<JsArray>(0)?;
    let callback = ctx.argument::<JsFunction>(1)?;

    let instruments: Vec<Handle<JsValue>> = instruments.to_vec(&mut ctx)?;
    let instruments: Vec<String> = instruments.iter().map(|ins| {
        ins.downcast::<JsString>().or_throw(&mut ctx).unwrap().value()
    }).collect();

    let (tx, rx) = bounded(1);

    ctx.borrow(&this, |market| {
        // 退订合约
        market.market.unsubscribe_market_data(&instruments).unwrap();
        market.market.on_rsp_unsub_market_data(move |_, _, error, _, _| {
            signal!(tx, Some(()), error)
        });
    });

    OnSignal {
        rx,
    }.schedule(callback);

    Ok(ctx.undefined().upcast())
}
