use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Bouncer {
    delay: Duration,
    last_run: Option<Instant>,
}

impl Bouncer {
    pub fn new(delay: Duration) -> Self {
        Bouncer {
            delay,
            last_run: None,
        }
    }

    /// 防抖
    pub fn debounce<T>(&mut self, mut func: impl FnMut() -> T) -> Option<T> {
        if let Some(then) = self.last_run {
            let now = Instant::now();
            if now.duration_since(then) > self.delay {
                self.last_run = Some(Instant::now());
                Some(func())
            } else {
                None
            }
        } else {
            self.last_run = Some(Instant::now());
            Some(func())
        }
    }

    /// 节流
    pub fn throttle<T>(&mut self, mut func: impl FnMut() -> T) -> T {
        if let Some(then) = self.last_run {
            let now = Instant::now();
            if now.duration_since(then) < self.delay {
                let delta = then + self.delay - now;
                spin_sleep::sleep(delta);
            }

            self.last_run = Some(Instant::now());
            func()
        } else {
            self.last_run = Some(Instant::now());
            func()
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
        static BOUNCER: LazyLock<Mutex<Bouncer>> =
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
        static BOUNCER: LazyLock<Mutex<Bouncer>> =
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
