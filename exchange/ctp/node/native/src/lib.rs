use crossbeam::channel::{Receiver, Sender};
use ctp::*;
use neon::prelude::*;

struct OnSignal<T>
where
    T: serde::Serialize + Send + Sized + 'static,
{
    rx: Receiver<CTPResult<Option<T>>>,
}

type SignalSlot<T> = (Sender<CTPResult<Option<T>>>, Receiver<CTPResult<Option<T>>>);

impl<T> Task for OnSignal<T>
where
    T: serde::Serialize + Send + Sized + 'static,
{
    type Output = CTPResult<Option<T>>;
    type Error = CTPError;
    type JsEvent = JsValue;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        self.rx
            .recv()
            .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
    }

    fn complete(
        self,
        mut ctx: TaskContext,
        event: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match event {
            Ok(Ok(Some(data))) => Ok(neon_serde::to_value(&mut ctx, &data)?),
            Ok(Ok(None)) => Ok(ctx.null().upcast()),
            Ok(Err(e)) | Err(e) => ctx.throw_error(format!("{}", e)),
        }
    }
}

macro_rules! signal {
    ($tx:ident, $res:expr) => {{
        $tx.send(Ok($res))
            .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
    }};

    ($tx:ident, $res:expr, $error:expr) => {{
        // 这里注意，CTP 有些接口 pRspInfo 一直是空的
        if $error.ErrorID == 0 {
            $tx.send(Ok($res))
                .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
        } else {
            log::error!("{}", $error);
            $tx.send(Err(CTPError::CTP(*$error)))
                .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
        }
    }};
}

macro_rules! on_signal {
    ($ctx:ident, $signal:ident) => {{
        let this = $ctx.this();
        let callback = $ctx.argument::<JsFunction>(0)?;
        let rx = $ctx.borrow(&this, |market| market.$signal.1.clone());

        $crate::OnSignal { rx }.schedule(callback);

        Ok($ctx.undefined().upcast())
    }};
}

mod market;
mod trader;
use market::*;
use trader::*;

fn ctp_version(mut cx: FunctionContext) -> JsResult<JsString> {
    let version = ctp::api::version();
    Ok(cx.string(version))
}

register_module!(mut cx, {
    cx.export_function("ctp_version", ctp_version)?;
    cx.export_class::<JsCTPMarket>("NodeCTPMarket")?;
    cx.export_class::<JsCTPTrader>("NodeCTPTrader")?;

    Ok(())
});
