use crate::*;
use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicUsize, Ordering::*},
    Arc,
};

mod redis;
use self::redis::RedisPuber;
mod amqp;
use self::amqp::AmqpPuber;

pub enum DodoQueuePuber {
    Redis(RedisPuber),
    Amqp(AmqpPuber),
}

type PuberInner = Arc<DodoQueuePuber>;

pub fn class(env: NapiEnv) -> NapiResult<JsClass> {
    JsClass::new(
        env,
        // Class Name
        "DodoQueuePuber",
        // The Constructor
        move |mut this, (url, queue_name): (JsString, JsValue)| {
            let env = this.env();

            this.set("__url__", url)?;
            this.set("__queue__", queue_name)?;

            let url = url.get()?;
            let queue_name = if let Ok(queue_name) = queue_name.cast_checked::<JsString>() {
                queue_name.get()?
            } else {
                "QUEUE".to_string()
            };

            let puber = if url.starts_with("redis://") {
                let puber = catch!(env, RedisPuber::new(&url, queue_name));

                this.set("publish", RedisPuber::publish(env)?)?;

                DodoQueuePuber::Redis(puber)
            } else if url.starts_with("amqp://") {
                let puber = catch!(env, AmqpPuber::new(&url, queue_name));

                this.set("publish", AmqpPuber::publish(env)?)?;

                DodoQueuePuber::Amqp(puber)
            } else {
                env.throw_error(format!("unsupported queue schema: {}", url))?;
                return env.undefined();
            };

            this.wrap(Arc::new(puber), move |_, _| {
                // DO SOME CLEANUP
                log::debug!("[RedisPuber] drops");
                Ok(())
            })?;

            env.undefined()
        },
        // The Descriptor
        &[],
        // &[DescriptorMethodBuilder::new()
        //     .with_utf8name("check_connection")
        //     .with_method(move |this, ()| {
        //         let env = this.env();
        //         if let Some(inner) = this.unwrap::<PuberInner>()? {
        //             env.boolean(inner.con.lock().check_connection())
        //         } else {
        //             env.boolean(false)
        //         }
        //     })
        //     .build()?],
    )
}
