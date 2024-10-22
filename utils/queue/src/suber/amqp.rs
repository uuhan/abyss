use super::*;
use futures_lite::stream::StreamExt;
#[allow(unused_imports)]
use lapin::{
    message::Delivery, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Connection as LapinConnection, ConnectionProperties,
};

#[derive(Clone)]
pub struct AmqpSuber {
    // Amqp 连接
    pub con: Arc<LapinConnection>,
    // 队列名字
    pub queue_name: String,
}

impl AmqpSuber {
    pub fn new(url: impl AsRef<str>, queue_name: String) -> Result<Self> {
        let url = url.as_ref();
        log::info!("[AmqpSuber] connect to: {}", url);

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
            log::info!("[AmqpSuber] queue declared: {:?}", queue);
            Result::Ok(con)
        })?;

        Ok(Self {
            con: Arc::new(con),
            queue_name,
        })
    }

    pub fn subscribe(env: NapiEnv) -> NapiResult<Function<JsUndefined>> {
        JsFunction::new(
            env,
            Some("subscribe"),
            move |this, worker: Function<JsUndefined>| {
                let env = this.env();
                let inner = if let Some(inner) = this.unwrap::<SuberInner>()? {
                    inner.clone()
                } else {
                    log::error!("Missing context of DodoQueueSuber!");
                    return env.undefined();
                };

                let tsfn: NapiTsfn<(Delivery, SuberInner)> = env.tsfn(
                    "AmqpSuber.subscribe",
                    worker,
                    move |_| Ok(()),
                    move |worker, (delivery, inner): (Delivery, SuberInner)| {
                        let env = worker.env();
                        let mut job = env.object()?;
                        log::debug!("[TSFN] worker fired: {:?}", delivery);

                        job.set(
                            "params",
                            env.string(
                                // NB: 我们暂时使用 `String` 传递消息
                                unsafe { String::from_utf8_unchecked(delivery.data) },
                            )?,
                        )?;

                        let reply_to = delivery.properties.reply_to().clone().unwrap();

                        job.set(
                            "publish",
                            env.func(move |this, result: JsString| {
                                let reply_to = reply_to.clone();
                                let env = this.env();
                                let result = result.get()?;

                                let suber = match &*inner {
                                    DodoQueueSuber::Amqp(suber) => suber,
                                    _ => {
                                        unreachable!()
                                    }
                                };

                                if let Err(e) = async_global_executor::block_on(async move {
                                    let channel = suber.con.create_channel().await?;
                                    let confirm = channel
                                        .basic_publish(
                                            "",
                                            reply_to.as_str(),
                                            BasicPublishOptions::default(),
                                            result.as_bytes(),
                                            BasicProperties::default(),
                                        )
                                        .await?
                                        .await?;

                                    // FIXME: better lifetime control
                                    debug_assert!(
                                        !reply_to.as_str().is_empty(),
                                        "reply_to should not be null"
                                    );

                                    log::debug!(
                                        "[AmqpSuber] reply_to: {}, {:?}",
                                        reply_to,
                                        confirm
                                    );
                                    Result::Ok(())
                                }) {
                                    log::error!("[AmqpSuber] callback error: {}", e);
                                }

                                env.undefined()
                            })?,
                        )?;

                        worker.call(job, job)?;
                        Ok(())
                    },
                )?;

                std::thread::spawn(move || {
                    if let Err(e) = async_global_executor::block_on(async move {
                        let suber = match &*inner {
                            DodoQueueSuber::Amqp(suber) => suber,
                            _ => {
                                unreachable!()
                            }
                        };

                        let channel = suber.con.create_channel().await?;
                        let mut consumer = channel
                            .basic_consume(
                                &suber.queue_name,
                                "",
                                BasicConsumeOptions::default(),
                                FieldTable::default(),
                            )
                            .await?;

                        while let Some(delivery) = consumer.next().await {
                            let delivery = delivery?;
                            log::debug!("[AmqpSuber] deliveried: {:#?}", delivery);
                            delivery.ack(BasicAckOptions::default()).await.expect("ack");
                            tsfn.non_blocking((delivery, inner.clone())).unwrap();
                        }

                        Result::Ok(())
                    }) {
                        log::error!("[AmqpSuber] subscribe: {}", e);
                    }
                });

                env.undefined()
            },
        )
    }
}
