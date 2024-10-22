use crate::*;

// DUMMY CALL BACK FUNCTION
macro_rules! empty {
    ($($t:tt),+) => ({
        move |$($t),+| { Ok(()) }
    })
}

macro_rules! signal {
    ($tx:expr, $res:expr) => {{
        $tx.send($res)
            .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
    }};

    ($tx:expr, $res:expr, $err:expr) => {{
        if $err.ErrorID == 0 {
            $tx.send(Ok($res))
                .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
        } else {
            $tx.send(Err(CTPError::CTP(*$err)))
                .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
        }
    }};
}

macro_rules! on_signal {
    ($rx:expr) => {{
        $rx.recv()
            .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
    }};
}

macro_rules! on_rsp {
    ($tx:expr, $error:expr) => {
        // 默认(>=3s)超时时长
        on_rsp!($tx, $error, 3)
    };

    ($tx:expr, $error:expr, $tm:literal) => {{
        let mut response = vec![];
        loop {
            crossbeam::select! {
                recv($tx) -> data => {
                    match data {
                        Ok((Some(data), last)) => {
                            response.push(data);
                            if last { break Ok(response) }
                        }
                        Ok((None, _)) => {
                            break Ok(response)
                        }
                        Err(e) => {
                            break Err(CTPError::Error(Arc::new(e)))
                        }
                    }
                }
                recv($error) -> error => {
                    match error {
                        Ok(error) => {
                            break Err(CTPError::CTP(error))
                        }
                        Err(e) => {
                            break Err(CTPError::Error(Arc::new(e)))
                        }
                    }
                }
                // NB: 响应超时, 则返回异常
                default(std::time::Duration::from_secs($tm)) => {
                    break Err(CTPError::TimeOut);
                }
            }
        }
    }};
}

macro_rules! rsp {
    ($tx:expr, $res:expr, $last:expr) => {{
        $tx.send(($res, $last))
            .map_err(|e| CTPError::Error(std::sync::Arc::new(e)))
    }};
}

/// 保证某些方法块只运行一次
#[macro_export]
macro_rules! once {
    ($args:block) => {
        use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
        static E: AtomicBool = AtomicBool::new(false);
        if E.compare_exchange(false, true, Relaxed, Relaxed).is_ok() {
            // only execute this once
            $args;
        }
    };
}

/// `f64::MAX` to NAN
#[macro_export]
macro_rules! nan {
    ($f:expr) => {{
        if $f == f64::MAX {
            f64::NAN
        } else {
            $f
        }
    }};
}

macro_rules! wait {
    ($cond:expr) => {{
        wait!($cond, |m: &mut bool| !*m, std::convert::identity)
    }};

    ($cond:expr, $f:expr) => {{
        wait!($cond, $f, std::convert::identity)
    }};

    ($cond:expr, $f:expr, $t:expr) => {{
        let (m, cond) = &*$cond;
        let mut result = cond.wait_while(m.lock()?, $f)?;
        $t(&mut *result)
    }};
}

macro_rules! wait_timeout {
    ($cond:expr, $duration:expr) => {{
        wait_timeout!($cond, |m: &mut bool| !*m, |m: &mut bool| *m, $duration)
    }};

    ($cond:expr, $f:expr, $duration:expr) => {{
        wait_timeout!($cond, $f, std::convert::identity, $duration)
    }};

    ($cond:expr, $f:expr, $t:expr, $duration:expr) => {{
        let (m, cond) = &*$cond;
        // $f 返回为 false 退出等待
        let (mut result, maybe_timeout) = cond.wait_timeout_while(m.lock()?, $duration, $f)?;
        ($t(&mut *result), maybe_timeout.timed_out())
    }};
}

/// 测量调用延迟
macro_rules! metric {
    ($name:expr) => {
        #[cfg(feature = "metrics")]
        let _request = $crate::metrics::CTP_REQ_DURATION
            .with_label_values(&[$name])
            .start_timer();
    };
}

macro_rules! req_ctp_function {
    ($($doc:literal)* $kind:ident->$function:ident, $($a:ident : $t:ty),*) => {
        $(#[doc = $doc])*
        #[inline]
        pub fn $function(&self, $($a:$t),*) -> CTPApiResult
        {
            metric!(stringify!($function));

            // 检查是否处于连接状态
            let is_ready = self.ready.get().unwrap_or(false);
            // CTP 未就绪
            if is_ready {
                self.api.lock().$function($($a),*)
            } else {
                tracing::warn!(func=stringify!($function), "ctp not ready.");
                Err(CTPError::NotReady)
            }
        }
    };

    // 节流(1req/s)
    ($($doc:literal)* T $kind:ident->$function:ident, $($a:ident : $t:ty),*) => {
        $(#[doc = $doc])*
        #[inline]
        pub fn $function(&self, $($a:$t),*) -> CTPApiResult
        {
            metric!(stringify!($function));

            // 检查是否处于连接状态
            let is_ready = self.ready.get().unwrap_or(false);
            // CTP 未就绪
            if is_ready {
                self.bouncer.lock().throttle(|| self.api.lock().$function($($a),*))
            } else {
                tracing::warn!(func=stringify!($function), "ctp not ready.");
                Err(CTPError::NotReady)
            }
        }
    };

    // 防抖
    ($($doc:literal)* D $kind:ident->$function:ident, $($a:ident : $t:ty),*) => {
        /// CTP 方法
        $(#[doc = $doc])*
        #[inline]
        pub fn $function(&self, $($a:$t),*) -> CTPApiResult
        {
            metric!(stringify!($function));

            // 检查是否处于连接状态
            let is_ready = *self.ready.0.lock();
            // CTP 未就绪
            if is_ready {
                self.bouncer.lock().debounce(|| self.api.lock().$function($($a),*))
            } else {
                tracing::warn!(func=stringify!($function), "ctp not ready.");
                Err(CTPError::NotReady)
            }
        }
    }
}

macro_rules! on_ctp_function {
    ($($doc:literal)* $kind:ident->$function:ident, $($arg:ident: $T:ty),* $(,)?) => {
        $(#[doc = $doc])*
        pub fn $function<F>(&self, mut cb: F) -> &Self
        where
            F: FnMut(Api, $($T),*) -> CTPApiResult + Send
        {
            let api = self.api.clone();
            self.spi.lock().$function(move |$($arg),*| {
                let api = api.clone();
                ctp_error_wrapper!(cb(api, $($arg),*))
            });
            self
        }

        paste::paste! {
            $(#[doc = $doc])*
            pub fn [<add_ $function>]<F>(&self, mut cb: F) -> &Self
            where
                F: FnMut(Api, $($T),*) -> CTPApiResult + Send
            {
                let api = self.api.clone();
                self.spi.lock().[<add_ $function>](move |$($arg),*| {
                    let api = api.clone();
                    ctp_error_wrapper!(cb(api, $($arg),*))
                });
                self
            }
        }
    }
}
