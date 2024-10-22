use crate::*;
use parking_lot::Mutex;
use std::sync::Arc;

mod redis;
use self::redis::RedisSuber;
mod amqp;
use self::amqp::AmqpSuber;

pub enum DodoQueueSuber {
    Redis(RedisSuber),
    Amqp(AmqpSuber),
}

type SuberInner = Arc<DodoQueueSuber>;

pub fn class(env: NapiEnv) -> NapiResult<JsClass> {
    JsClass::new(
        env,
        // Class Name
        "RedisSuber",
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

            let suber = if url.starts_with("redis://") {
                let suber = catch!(env, RedisSuber::new(&url, queue_name));

                this.set("subscribe", RedisSuber::subscribe(env)?)?;

                DodoQueueSuber::Redis(suber)
            } else if url.starts_with("amqp://") {
                let suber = catch!(env, AmqpSuber::new(&url, queue_name));

                this.set("subscribe", AmqpSuber::subscribe(env)?)?;

                DodoQueueSuber::Amqp(suber)
            } else {
                env.throw_error(format!("unsupport queue schema: {}", url))?;
                return env.undefined();
            };

            this.wrap(Arc::new(suber), move |_, _| {
                // DO SOME CLEANUP
                log::debug!("[RedisSuber] drops");
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
        //         if let Some(inner) = this.unwrap::<SuberInner>()? {
        //             env.boolean(inner.con.lock().check_connection())
        //         } else {
        //             env.boolean(false)
        //         }
        //     })
        //     .build()?],
    )
}
