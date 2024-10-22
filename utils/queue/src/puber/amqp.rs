use super::*;
use futures_lite::StreamExt;
#[allow(unused_imports)]
use lapin::{
    message::{Delivery, DeliveryResult},
    options::*,
    publisher_confirm::Confirmation,
    types::FieldTable,
    types::*,
    BasicProperties, Channel, Connection as LapinConnection, ConnectionProperties,
};

#[derive(Clone)]
pub struct AmqpPuber {
    // Amqp 连接
    pub con: Arc<LapinConnection>,
    // 队列名字
    pub queue_name: String,
    // 任务唯一ID
    pub serial: Arc<AtomicUsize>,
}

impl AmqpPuber {
    pub fn new(url: impl AsRef<str>, queue_name: String) -> Result<Self> {
        let url = url.as_ref();
        log::info!("[AmqpPuber] connect to: {}", url);

        let con = async_global_executor::block_on(async {
            let con = LapinConnection::connect(url, ConnectionProperties::default()).await?;
            let channel = con.create_channel().await?;
            let queue = channel
                .queue_declare(
                    &queue_name,
                    QueueDeclareOptions {
                        ..Default::default()
                    },
                    FieldTable::default(),
                )
                .await?;
            log::info!("[AmqpPuber] queue declared: {:?}", queue);
            Result::Ok(con)
        })?;

        Ok(Self {
            con: Arc::new(con),
            queue_name,
            serial: Arc::new(AtomicUsize::new(0)),
        })
    }

    pub fn publish(env: NapiEnv) -> NapiResult<JsFunction> {
        JsFunction::new(
            env,
            Some("publish"),
            move |this, (params, timed): (JsString, JsValue)| {
                let env = this.env();
                let params = params.get()?;
                let _timed = if let Ok(timed) = timed.cast_checked::<JsNumber>() {
                    let timed = timed.get_value_double()?;
                    Some(std::time::Duration::from_secs_f64(timed))
                } else {
                    None
                };

                let promise: JsPromise<JsString, JsError> = JsPromise::new(env)?;

                let inner = if let Some(inner) = this.unwrap::<PuberInner>()? {
                    inner.clone()
                } else {
                    promise.reject(env.error("Missing context of DodoQueuePuber!")?)?;
                    return Ok(promise.value());
                };

                env.async_work(
                    "AmqpPuber.publish",
                    (
                        Option::<String>::None,
                        Option::<Error>::None,
                        Option::<(Channel, ShortString)>::None,
                    ),
                    move |(payload, error, channel_hold)| {
                        let puber = match &*inner {
                            DodoQueuePuber::Amqp(puber) => puber,
                            _ => {
                                unreachable!()
                            }
                        };

                        let params = params.clone();
                        match async_global_executor::block_on(async move {
                            let channel = puber.con.create_channel().await?;
                            let queue = channel
                                .queue_declare(
                                    "",
                                    QueueDeclareOptions {
                                        exclusive: true,
                                        ..Default::default()
                                    },
                                    FieldTable::default(),
                                )
                                .await?;

                            log::info!("declared queue: {:?}", queue);
                            let exclusive_queue_name = queue.name().clone();

                            let mut result_consumer = channel
                                .basic_consume(
                                    exclusive_queue_name.as_str(),
                                    "",
                                    BasicConsumeOptions::default(),
                                    FieldTable::default(),
                                )
                                .await?;

                            let publish_confirm = channel
                                .basic_publish(
                                    "",
                                    &puber.queue_name,
                                    BasicPublishOptions::default(),
                                    params.as_bytes(),
                                    BasicProperties::default()
                                        .with_reply_to(exclusive_queue_name.clone()),
                                )
                                .await?
                                .await?;
                            log::info!("[AmqpPuber] publish: {:?}", publish_confirm);

                            channel_hold.replace((channel, exclusive_queue_name));

                            if let Some(result_delivery) = result_consumer.next().await {
                                let result_delivery = result_delivery?;
                                result_delivery
                                    .ack(BasicAckOptions::default())
                                    .await
                                    .expect("ack");
                                let result =
                                    unsafe { String::from_utf8_unchecked(result_delivery.data) };
                                Result::Ok(result)
                            } else {
                                Result::Err(Error::CustomError("unexpected".into()))
                            }
                        }) {
                            Ok(result) => {
                                payload.replace(result);
                            }
                            Err(e) => {
                                error.replace(e);
                            }
                        }
                    },
                    move |env, status, (mut payload, mut error, mut r)| {
                        if let Some((channel, name)) = r.take() {
                            async_global_executor::spawn(async move {
                                if let Err(e) = channel
                                    .queue_delete(name.as_str(), QueueDeleteOptions::default())
                                    .await
                                {
                                    log::error!("delete queue: {}", e);
                                }
                            })
                            .detach();
                        }

                        if status == NapiStatus::Ok {
                            if let Some(payload) = payload.take() {
                                log::debug!("publish resolve with {}", payload);
                                promise.resolve(env.string(payload)?)?;
                            } else if let Some(err) = error.take() {
                                let error = err.to_string();
                                log::error!("publish error: {}", error);
                                promise.reject(env.error(error)?)?;
                            } else {
                                log::warn!("subscribe timed out");
                                promise.reject(env.error("subscribe timed out.")?)?;
                            }
                        } else {
                            log::error!("publish error: {}", status);
                            promise.reject(env.error(format!("{}", status))?)?;
                        }
                        Ok(())
                    },
                )?
                .queue()?;

                Ok(promise.value())
            },
        )
    }
}
