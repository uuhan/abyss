use ctp::{CTPMarket, CTPTrader};
use nodex::prelude::*;

nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    env_logger::init();
    // 使用 napi >= 8
    nodex::napi_guard!(env.napi_version()?);

    // ctp version
    exports.set_named_property("version", env.string(ctp::api::version())?)?;

    exports.set("CTPMarketClass", CTPMarket::class(env)?)?;
    exports.set("CTPTraderClass", CTPTrader::class(env)?)?;

    Ok(())
}
