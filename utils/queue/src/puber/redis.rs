use super::*;
use ::redis::*;

#[derive(Clone)]
pub struct RedisPuber {
    // 队列信息
    pub client: Client,
    // 当前连接ID
    pub id: usize,
    // 默认的连接
    pub con: Arc<Mutex<Connection>>,
    // 队列名字
    pub queue_name: String,
    // 任务唯一ID
    pub serial: Arc<AtomicUsize>,
}

impl RedisPuber {
    pub fn new(url: impl AsRef<str>, queue_name: String) -> Result<Self> {
        let url = url.as_ref();
        log::info!("[RedisPuber] connect to: {}", url);

        let client = Client::open(url)?;
        let serial = Arc::new(AtomicUsize::new(0));
        let mut con = client.get_connection()?;
        let id: usize = redis::cmd("CLIENT").arg(&["ID"]).query(&mut con)?;
        log::info!("[RedisPuber] connected client id: {}", id);

        Ok(Self {
            client,
            id,
            con: Arc::new(Mutex::new(con)),
            queue_name,
            serial,
        })
    }

    pub fn connection(&self) -> Result<Connection> {
        Ok(self.client.get_connection()?)
    }

    pub fn publish(env: NapiEnv) -> NapiResult<JsFunction> {
        JsFunction::new(
            env,
            Some("publish"),
            move |this, (params, timed): (JsString, JsValue)| {
                let env = this.env();
                let params = params.get()?;
                let timed = if let Ok(timed) = timed.cast_checked::<JsNumber>() {
                    let timed = timed.get_value_double()?;
                    Some(std::time::Duration::from_secs_f64(timed))
                } else {
                    None
                };

                let promise: JsPromise<JsString, JsError> = JsPromise::new(env)?;

                let inner = if let Some(inner) = this.unwrap::<PuberInner>()? {
                    inner.clone()
                } else {
                    promise.reject(env.error("unreachable!")?)?;
                    return Ok(promise.value());
                };

                let puber = match &*inner {
                    DodoQueuePuber::Redis(puber) => puber,
                    _ => {
                        unreachable!()
                    }
                };

                // 任务结构
                let task = Task {
                    client: puber.id,
                    serial: puber.serial.fetch_add(1, Relaxed),
                    params,
                };

                log::debug!("current client id: {}", puber.id);

                env.async_work(
                    "RedisPuber.publish",
                    (Option::<String>::None, Option::<Error>::None),
                    move |(payload, error)| {
                        let puber = match &*inner {
                            DodoQueuePuber::Redis(puber) => puber,
                            _ => {
                                unreachable!()
                            }
                        };

                        // 订阅任务结果
                        let mut pscon = match puber.connection() {
                            Ok(con) => con,
                            Err(e) => {
                                log::error!("puber connect error: {:?}", e);
                                error.replace(e);
                                return;
                            }
                        };

                        // 监听任务结果
                        let mut pubsub = pscon.as_pubsub();
                        let subscribtion =
                            format!("{}:{}:{}", puber.queue_name, puber.id, task.serial);
                        log::debug!("subscribe: {}", subscribtion);
                        if let Err(e) = pubsub.subscribe(&subscribtion) {
                            log::error!("{}", e);
                            return;
                        };

                        // 压入任务到队列
                        let queue_name = &puber.queue_name;
                        let task_s = if let Ok(task_s) = serde_json::to_string(&task) {
                            task_s
                        } else {
                            log::error!("{:?} serialize failed.", &task);
                            return;
                        };

                        let mut con = puber.con.lock();
                        while let Err(e) = con.rpush::<_, _, usize>(queue_name, &task_s) {
                            log::error!("[RPUSH] {}: {}", queue_name, e);

                            'reconnect: loop {
                                match puber.connection() {
                                    Ok(new) => {
                                        *con = new;
                                        log::debug!("puber reconnected");
                                        break 'reconnect;
                                    }
                                    Err(e) => {
                                        log::error!(
                                            "puber reconnect error: {}, reconnect in 1s",
                                            e
                                        );
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                    }
                                }
                            }
                        }

                        drop(con);
                        log::debug!("[RPUSH] {}: {}", queue_name, task_s);

                        match pubsub
                            // NB: 这里设置最长的等待时间 10 秒
                            .set_read_timeout(timed)
                            .and_then(|_| pubsub.get_message())
                            .and_then(|msg| msg.get_payload())
                        {
                            Ok(message) => {
                                log::debug!("subscribed: {}", message);
                                payload.replace(message);
                            }
                            Err(e) => {
                                log::error!("subscribe error: {}", e);
                            }
                        }

                        if let Err(e) = pubsub.unsubscribe(&subscribtion) {
                            log::error!("unsubscribe {} failed {}", subscribtion, e);
                        }
                    },
                    move |env, status, (mut payload, mut error)| {
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
