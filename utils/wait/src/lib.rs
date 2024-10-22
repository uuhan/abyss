/// Wait for the data ready.
use parking_lot::{Condvar, Mutex};
use std::{
    future::Future,
    ops::Deref,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration,
};

#[derive(Debug)]
pub struct Wait<T>(Arc<(Mutex<(Option<T>, Option<Waker>)>, Condvar)>);

impl<T> Clone for Wait<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Wait<T> {
    pub fn new() -> Self {
        Self(Arc::new((Mutex::new((None, None)), Condvar::new())))
    }

    #[inline]
    pub fn wake(&mut self, data: T) {
        let Wait(ref pair) = self;
        let (ref mut hold, ref mut waker) = *pair.0.lock();
        hold.replace(data);

        if let Some(waker) = waker.take() {
            waker.wake();
        }

        pair.1.notify_all();
    }

    /// Wait for a value.
    pub fn wait(&self) -> T {
        let (locker, cdv) = &*self.0;
        let mut waited = locker.lock();
        while waited.0.is_none() {
            cdv.wait(&mut waited);
        }

        waited.0.take().unwrap()
    }

    /// Wait for a value with time limit.
    pub fn timeout(&self, duration: Duration) -> Option<T> {
        let (locker, cdv) = &*self.0;
        let mut waited = locker.lock();

        while waited.0.is_none() {
            let maybe_timeout = cdv.wait_for(&mut waited, duration);
            if maybe_timeout.timed_out() {
                return None;
            }
        }

        waited.0.take()
    }
}

impl<T> Default for Wait<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Wait<T> {
    pub fn get(&self) -> Option<T> {
        let Wait(ref pair) = self;
        let (ref hold, _) = *pair.0.lock();
        hold.clone()
    }
}

impl<T: PartialEq> Wait<T> {
    /// Wait untill value matches.
    pub fn until(&self, target: T) -> T {
        let (locker, cdv) = &*self.0;
        let mut waited = locker.lock();

        while waited.0.as_ref() != Some(&target) {
            cdv.wait(&mut waited);
        }

        target
    }

    /// Wait for a value with time limit.
    pub fn until_timeout(&self, target: T, duration: Duration) -> Option<T> {
        let (locker, cdv) = &*self.0;
        let mut waited = locker.lock();

        while waited.0.as_ref() != Some(&target) {
            let maybe_timeout = cdv.wait_for(&mut waited, duration);
            if maybe_timeout.timed_out() {
                return None;
            }
        }

        Some(target)
    }
}

impl<T> Deref for Wait<T> {
    type Target = (Mutex<(Option<T>, Option<Waker>)>, Condvar);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Future for Wait<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let (locker, _) = &*self.0;
        let mut waited = locker.lock();

        if let Some(data) = waited.0.take() {
            Poll::Ready(data)
        } else {
            waited.1.replace(ctx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{self, sleep};
    use std::time::Duration;

    #[test]
    fn test_wait() {
        let cdv = Wait::<()>::new();
        let mut cdv_wake = cdv.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(10));
            cdv_wake.wake(());
        });

        assert_eq!(cdv.wait(), ());

        let cdv = Wait::<()>::new();
        let mut cdv_wake = cdv.clone();
        thread::spawn(move || {
            sleep(Duration::from_millis(10));
            cdv_wake.wake(());
        });

        futures_lite::future::block_on(async {
            assert_eq!(cdv.await, ());
        });

        let cdv = Wait::<()>::new();
        let mut cdv_wake = cdv.clone();
        thread::spawn(move || {
            futures_lite::future::block_on(async {
                assert_eq!(cdv.await, ());
            });
        });

        sleep(Duration::from_millis(10));
        cdv_wake.wake(());
    }

    #[test]
    fn test_wait_timeout() {
        let cdv = Wait::<()>::new();
        let mut cdv_wake = cdv.clone();

        thread::spawn(move || {
            sleep(Duration::from_millis(10));
            cdv_wake.wake(());
        });

        assert_eq!(cdv.timeout(Duration::from_millis(5)), None);
        assert_eq!(cdv.timeout(Duration::from_millis(10)), Some(()));
    }

    #[test]
    fn test_wait_until() {
        let cdv = Wait::<bool>::new();
        let mut cdv_wake = cdv.clone();

        thread::spawn(move || {
            cdv_wake.wake(false);
            sleep(Duration::from_millis(10));
            cdv_wake.wake(false);
            sleep(Duration::from_millis(10));
            cdv_wake.wake(false);
            sleep(Duration::from_millis(10));
            cdv_wake.wake(true);
        });

        assert_eq!(cdv.wait(), false);
        cdv.until(true);
        assert_eq!(cdv.wait(), true);
    }
}
