/// 订阅 CTP 消息
use std::{
    collections::{BTreeMap, HashMap},
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        mpsc::{sync_channel, Receiver, RecvTimeoutError, SyncSender, TryRecvError},
        Arc,
    },
    task::{Context, Poll, Waker},
    time::{Duration, Instant},
};

use abyss_promise::*;
use parking_lot::RwLock;

static ID_GEN: AtomicUsize = AtomicUsize::new(0);

type Senders<T> = HashMap<usize, (Option<Waker>, SyncSender<Promise<T>>)>;

/// A simple message broker
///
/// # Example
///
/// ```rust
/// use abyss_subscriber::Subscribers;
/// let subs = Subscribers::<()>::new();
/// let mut subscriber = subs.register(b"");
///
/// let sub = subs.reserve(b"test").unwrap();
///
/// sub.complete(&());
/// assert_eq!(subscriber.next(), Some(()));
///
/// drop(subs);
/// assert_eq!(subscriber.next(), None);
/// ```
pub struct Subscriber<T> {
    id: usize,
    rx: Receiver<Promise<T>>,
    existing: Option<Promise<T>>,
    home: Arc<RwLock<Senders<T>>>,
}

impl<T> Drop for Subscriber<T> {
    fn drop(&mut self) {
        let mut w_senders = self.home.write();
        w_senders.remove(&self.id);
    }
}

impl<T> Subscriber<T> {
    /// Attempts to wait for a value on this `Subscriber`, returning
    /// an error if no event arrives within the provided `Duration`
    /// or if the backing `Db` shuts down.
    pub fn next_timeout(&mut self, mut timeout: Duration) -> Result<T, RecvTimeoutError> {
        loop {
            let start = Instant::now();
            let promise = if let Some(promise) = self.existing.take() {
                promise
            } else {
                self.rx.recv_timeout(timeout)?
            };

            timeout = if let Some(timeout) = timeout.checked_sub(start.elapsed()) {
                timeout
            } else {
                Duration::from_nanos(0)
            };

            let start = Instant::now();
            match promise.timeout(timeout) {
                Ok(Some(event)) => return Ok(event),
                Ok(None) => (),
                Err(_) => {
                    return Err(RecvTimeoutError::Timeout);
                }
            }

            timeout = if let Some(timeout) = timeout.checked_sub(start.elapsed()) {
                timeout
            } else {
                Duration::from_nanos(0)
            };
        }
    }
}

impl<T> Future for Subscriber<T> {
    type Output = Option<T>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let mut promise = if let Some(promise) = self.existing.take() {
                promise
            } else {
                match self.rx.try_recv() {
                    Ok(promise) => promise,
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return Poll::Ready(None),
                }
            };

            match Future::poll(Pin::new(&mut promise), cx) {
                Poll::Ready(Some(event)) => return Poll::Ready(Some(event)),
                Poll::Ready(None) => continue,
                Poll::Pending => {
                    self.existing = Some(promise);
                    return Poll::Pending;
                }
            }
        }
        let mut home = self.home.write();

        // NB: 这里出现过 None 的情况, 导致 panic
        // 一般是 Subscribers 析构导致
        if let Some(entry) = home.get_mut(&self.id) {
            entry.0 = Some(cx.waker().clone());
            Poll::Pending
        } else {
            log::error!("missing subscriber.");
            Poll::Ready(None)
        }
    }
}

impl<T> Iterator for Subscriber<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.rx.recv().ok()?.resolve()
    }
}

#[derive(Debug, Default)]
pub struct Subscribers<T> {
    watched: RwLock<BTreeMap<Vec<u8>, Arc<RwLock<Senders<T>>>>>,
    ever_used: AtomicBool,
}

impl<T> Drop for Subscribers<T> {
    fn drop(&mut self) {
        let watched = self.watched.read();

        for senders in watched.values() {
            let senders = std::mem::take(&mut *senders.write());
            for (_, (waker, sender)) in senders {
                drop(sender);
                if let Some(waker) = waker {
                    waker.wake();
                }
            }
        }
    }
}

impl<T> Subscribers<T> {
    pub fn new() -> Self {
        Self {
            watched: RwLock::new(BTreeMap::new()),
            ever_used: AtomicBool::default(),
        }
    }

    pub fn register(&self, prefix: &[u8]) -> Subscriber<T> {
        self.ever_used.store(true, Relaxed);

        let r_mu = {
            let r_mu = self.watched.read();
            if r_mu.contains_key(prefix) {
                r_mu
            } else {
                drop(r_mu);
                let mut w_mu = self.watched.write();
                if !w_mu.contains_key(prefix) {
                    let old =
                        w_mu.insert(prefix.to_vec(), Arc::new(RwLock::new(HashMap::default())));
                    assert!(old.is_none());
                }
                drop(w_mu);
                self.watched.read()
            }
        };

        let (tx, rx) = sync_channel(2048);

        let arc_senders = &r_mu[prefix];
        let mut w_senders = arc_senders.write();

        let id = ID_GEN.fetch_add(1, Relaxed);

        w_senders.insert(id, (None, tx));

        Subscriber {
            id,
            rx,
            existing: None,
            home: arc_senders.clone(),
        }
    }

    // 前缀匹配广播
    pub fn reserve<R: AsRef<[u8]>>(&self, key: R) -> Option<ReservedBroadcast<T>> {
        if !self.ever_used.load(Relaxed) {
            return None;
        }

        let r_mu = self.watched.read();
        let prefixes = r_mu.iter().filter(|(k, _)| key.as_ref().starts_with(k));

        let mut subscribers = vec![];
        let mut full_subs = vec![];

        for (key, subs_rwl) in prefixes {
            let subs = subs_rwl.read();

            for (id, (waker, sender)) in subs.iter() {
                let (tx, rx) = Promise::pair();
                if let Err(err) = sender.try_send(rx) {
                    log::error!("[subscriber/reserve] channel: {:?}, prefix: {:?}", err, key);
                    full_subs.push((*id, subs_rwl));
                    continue;
                }
                subscribers.push((waker.clone(), tx));
            }
        }

        if full_subs.len() > 0 {
            log::warn!("[subscriber/reserve] channel full: {:?}", full_subs);
            for (id, subs_rwl) in full_subs.into_iter() {
                let mut w_subs = subs_rwl.write();
                // remove full channel
                w_subs.remove(&id);
            }
        }

        if subscribers.is_empty() {
            None
        } else {
            Some(ReservedBroadcast { subscribers })
        }
    }

    // 精确匹配广播
    pub fn exact<R: AsRef<[u8]>>(&self, key: R) -> Option<ReservedBroadcast<T>> {
        if !self.ever_used.load(Relaxed) {
            return None;
        }
        let r_mu = self.watched.read();
        if let Some(sub_rwl) = r_mu.get(key.as_ref()) {
            let subs = sub_rwl.read();
            let mut subscribers = vec![];
            for (_id, (waker, sender)) in subs.iter() {
                let (tx, rx) = Promise::pair();
                if sender.send(rx).is_err() {
                    continue;
                }
                subscribers.push((waker.clone(), tx));
            }
            Some(ReservedBroadcast { subscribers })
        } else {
            None
        }
    }

    // 列出注册的键值
    // 有效的服务定义为: 存在对应 Subscriber 的键
    pub fn watched(&self) -> Vec<Vec<u8>> {
        let mut keys = vec![];
        let r_mu = self.watched.read();

        for (key, sub_rwl) in r_mu.iter() {
            let subs = sub_rwl.read();
            if subs.len() != 0 {
                keys.push(key.clone());
            }
        }

        keys
    }
}

pub struct ReservedBroadcast<T> {
    subscribers: Vec<(Option<Waker>, PromiseResolver<T>)>,
}

impl<T: Clone> ReservedBroadcast<T> {
    pub fn complete(self, event: &T) {
        let iter = self.subscribers.into_iter();

        for (waker, tx) in iter {
            tx.resolve(event.clone());
            if let Some(waker) = waker {
                waker.wake();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscriber_drop() {
        let subs = Subscribers::<()>::new();
        let mut subscriber = subs.register(b"");
        let sub = subs.reserve(b"test").unwrap();

        drop(sub);
        assert_eq!(subscriber.next(), None);

        let sub = subs.reserve(b"test").unwrap();
        sub.complete(&());
        assert_eq!(subscriber.next(), Some(()));
    }

    #[test]
    fn test_subscriber_iter() {
        let subs = Subscribers::new();
        let subscriber = subs.register(b"");

        Promise::new(move |promise| {
            subs.reserve(b"test").unwrap().complete(&());
            subs.reserve(b"test").unwrap().complete(&());
            subs.reserve(b"test").unwrap().complete(&());
            promise.resolve(());
            // NB: subs dropped here
        });

        assert_eq!(subscriber.collect::<Vec<_>>().len(), 3);
    }

    #[test]
    fn test_subscriber_timeout() {
        let subs = Subscribers::<()>::new();
        let mut subscriber = subs.register(b"");

        Promise::new(move |promise| {
            std::thread::sleep(std::time::Duration::from_millis(60));
            subs.reserve(b"test").unwrap().complete(&());
            subs.reserve(b"test").unwrap().complete(&());
            subs.reserve(b"test").unwrap().complete(&());
            promise.resolve(())
            // NB: subs dropped here
        });

        assert!(subscriber
            .next_timeout(std::time::Duration::from_millis(50))
            .is_err());
        assert!(subscriber
            .next_timeout(std::time::Duration::from_millis(50))
            .is_ok());
    }

    #[test]
    fn test_subscriber_fut() {
        loom::model(|| {
            let subs = Subscribers::new();
            let mut subscriber = subs.register(b"");

            loom::thread::spawn(move || {
                subs.reserve(b"test").unwrap().complete(&());
                subs.reserve(b"test").unwrap().complete(&());
                subs.reserve(b"test").unwrap().complete(&());
                // NB: subs dropped here
            })
            .join()
            .unwrap();

            loom::future::block_on(async {
                assert_eq!((&mut subscriber).await, Some(()));
                assert_eq!((&mut subscriber).await, Some(()));
                assert_eq!((&mut subscriber).await, Some(()));
                assert_eq!((&mut subscriber).await, None);
            });
        });
    }
}
