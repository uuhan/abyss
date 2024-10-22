use nodex::prelude::*;

mod error;
mod mac;
pub use error::*;

// 节流器
pub mod bouncer;
pub mod driver;

mod puber;
mod suber;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Task {
    pub client: usize,
    pub serial: usize,
    pub params: String,
}

nodex::napi_module!(init);

fn init(env: NapiEnv, mut exports: JsObject) -> NapiResult<()> {
    env_logger::init();
    nodex::napi_guard!(env.napi_version()?);

    exports.set("DodoQueuePuber", puber::class(env)?)?;
    exports.set("DodoQueueSuber", suber::class(env)?)?;

    Ok(())
}
