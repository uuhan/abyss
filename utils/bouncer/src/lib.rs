#![feature(lazy_cell)]

use std::time::{Duration, Instant};

/// 节流器
///
/// # Example
///
/// ```rust
/// use abyss_bouncer::Bouncer;
/// use std::time::Duration;
/// let mut bouncer = Bouncer::new(Duration::from_secs(1));
/// let mut count = 0;
/// bouncer.debounce(|| count += 1);
/// bouncer.debounce(|| count += 1);
/// bouncer.debounce(|| count += 1);
/// assert_eq!(count, 1);
///
/// bouncer.throttle(|| count += 1);
/// bouncer.throttle(|| count += 1);
/// bouncer.throttle(|| count += 1);
/// assert_eq!(count, 4);
/// ```
#[derive(Debug)]
pub struct Bouncer<T> {
    delay: Duration,
    last_run: Option<Instant>,
    func: Option<fn() -> T>,
    result: Option<T>,
}

impl<T> Bouncer<T> {
    pub fn new(delay: Duration) -> Self {
        return Bouncer {
            delay,
            last_run: None,
            func: None,
            result: None,
        };
    }

    pub fn with_func(mut self, func: fn() -> T) -> Self {
        self.func = Some(func);
        return self;
    }

    pub fn execute(&mut self) {
        if self.func.is_some() {
            let result = self.debounce(self.func.unwrap());
            self.result = result;
        }
    }

    pub fn get_result(&mut self) -> Option<&mut T> {
        return self.result.as_mut();
    }

    /// 防抖
    pub fn debounce(&mut self, mut func: impl FnMut() -> T) -> Option<T> {
        if let Some(then) = self.last_run {
            let now = Instant::now();
            if now.duration_since(then) > self.delay {
                self.last_run = Some(Instant::now());

                return Some(func());
            } else {
                let delta = then + self.delay - now;
                log::warn!("[bouncer] 请求触发防抖: {:?}", delta);
                return None;
            }
        } else {
            self.last_run = Some(Instant::now());
            return Some(func());
        }
    }

    /// 节流
    pub fn throttle(&mut self, mut func: impl FnMut() -> T) -> T {
        if let Some(then) = self.last_run {
            let now = Instant::now();
            if now.duration_since(then) < self.delay {
                let delta = then + self.delay - now;
                log::warn!("[bouncer] 请求触发节流: {:?}", delta);
                spin_sleep::sleep(delta);
            }

            self.last_run = Some(Instant::now());
            return func();
        } else {
            self.last_run = Some(Instant::now());
            return func();
        }
    }

    pub fn reset(&mut self) {
        self.last_run = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use std::sync::Mutex;
    use std::time::Duration;

    #[test]
    fn test_debounce() {
        static BOUNCER: LazyLock<Mutex<Bouncer<()>>> =
            LazyLock::new(|| Mutex::new(Bouncer::new(Duration::from_secs(1))));

        let mut count = 0;

        BOUNCER.lock().unwrap().debounce(|| {
            count += 1;
        });

        BOUNCER.lock().unwrap().debounce(|| {
            count += 1;
        });

        BOUNCER.lock().unwrap().debounce(|| {
            count += 1;
        });

        spin_sleep::sleep(Duration::from_secs(1));

        BOUNCER.lock().unwrap().debounce(|| {
            count += 1;
        });

        assert_eq!(count, 2);
    }

    #[test]
    fn test_throttle() {
        static BOUNCER: LazyLock<Mutex<Bouncer<()>>> =
            LazyLock::new(|| Mutex::new(Bouncer::new(Duration::from_secs(1))));

        let mut count = 0;

        BOUNCER.lock().unwrap().throttle(|| {
            count += 1;
        });

        BOUNCER.lock().unwrap().throttle(|| {
            count += 1;
        });

        BOUNCER.lock().unwrap().throttle(|| {
            count += 1;
        });

        BOUNCER.lock().unwrap().throttle(|| {
            count += 1;
        });

        assert_eq!(count, 4);
    }
}
