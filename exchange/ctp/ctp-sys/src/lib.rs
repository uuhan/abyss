//! 这个crate是对ctp接口的封装.
//!
//! # mod ctp
//!
//! # mod ext
//!
use std::ffi::{c_void, CStr};

/// 保证某些方法块只运行一次
#[macro_export]
macro_rules! once {
    ($block:block) => {
        use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
        static E: AtomicBool = AtomicBool::new(false);
        if E.compare_exchange(false, true, Relaxed, Relaxed).is_ok() {
            // only execute this once
            $block;
        }
    };
}

#[macro_export]
macro_rules! ret_value_wrapper {
    (once $body:expr) => {{
        // CTP 请求
        // -1: 网络连接失败
        // -2: 库函数限流, 在途的查询只允许有一个
        // -3: 每秒发送请求超过许可数
        let ret = $body;
        // 如果遇到库函数限流, 重复请求
        // 其他情况不重复请求
        match ret {
            0 => {
                tracing::trace!(func = stringify!($body), "请求成功");
            }
            -1 => {
                tracing::error!(func = stringify!($body), "请求失败");
            }
            -2 => {
                tracing::warn!(func = stringify!($body), "触发限流");
            }
            -3 => {
                tracing::warn!(func = stringify!($body), "服务限流");
            }
            n => {
                tracing::error!("异常请求状态 RET = {}", n);
            }
        }

        if ret == 0 {
            Ok(())
        } else {
            Err($crate::ctp::error::CTPError::RequestError(ret))
        }
    }};

    ($body:expr) => {{
        // 返回值
        let mut ret = 0;
        // 重试计数
        let mut count = 0;

        // CTP 请求
        // -1: 网络连接失败
        // -2: 库函数限流, 在途的查询只允许有一个
        // -3: 每秒发送请求超过许可数
        loop {
            ret = $body;
            // 如果遇到库函数限流, 重复请求
            // 其他情况不重复请求
            match ret {
                0 => {
                    tracing::trace!(count, func = stringify!($body), "请求成功");
                    break;
                }
                -1 => {
                    tracing::error!(count, func = stringify!($body), "请求失败");
                    break;
                }
                -2 => {
                    tracing::warn!(count, func = stringify!($body), "触发限流");
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    count += 1;

                    if count >= 4 {
                        break;
                    }
                }
                -3 => {
                    tracing::warn!(count, func = stringify!($body), "服务限流");
                    // CTP session级别1s一个请求
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    count += 1;

                    if count >= 4 {
                        break;
                    }
                }
                n => {
                    tracing::error!("异常请求状态 RET = {}", n);
                    break;
                }
            }
        }

        if ret == 0 {
            Ok(())
        } else {
            Err($crate::ctp::error::CTPError::RequestError(ret))
        }
    }};
}

#[macro_export]
macro_rules! ctp_error_wrapper {
    ($expr:expr) => {{
        match $expr {
            Ok(()) => {}
            Err(error) => {
                tracing::error!(?error, "ctp error.");
            }
        }
    }};
}

macro_rules! ctp_fn_wrapper {
    ($($doc: literal)* $kind:ident->$function:ident, $($arg:ty),* $(,)?) => {
        paste::paste! {
        $(#[doc = $doc])*
        #[inline]
        pub fn $function(&mut self, mut cb: impl FnMut($($arg),*) + Send) -> &Self
        {
            if (!self.0.[<_ $kind spi_ $function _cb>].is_null()) {
                unsafe {
                    // 转化回调指针
                    let cbs_t: *mut (
                        Option<Box<dyn FnMut($($arg),*)>>,
                        Vec<Box<dyn FnMut($($arg),*)>>
                    ) = std::mem::transmute(self.0.[<_ $kind spi_ $function _cb>]);
                    // 获取回调列表
                    let mut cbs_t: Box<(
                        Option<Box<dyn FnMut($($arg),*)>>,
                        Vec<Box<dyn FnMut($($arg),*)>>
                    )> = Box::from_raw(cbs_t);

                    // on_xxx 类的方法, 替换
                    cbs_t.0.replace(Box::new(cb));
                    std::mem::forget(cbs_t);
                }
            } else {
                let cbs_t: Box<(
                    Option<Box<dyn FnMut($($arg),*)>>,
                    Vec<Box<dyn FnMut($($arg),*)>>
                )> = Box::new((Some(Box::new(cb)), vec![]));

                self.0.[<_ $kind spi_ $function _cb>] = Box::into_raw(cbs_t) as *mut std::os::raw::c_void;
            }

            self
        }

        $(#[doc = $doc])*
        pub fn [<add_ $function>](&mut self, mut cb: impl FnMut($($arg),*) + Send) -> &Self
        {
            if (!self.0.[<_ $kind spi_ $function _cb>].is_null()) {
                unsafe {
                    // 转化回调指针
                    let cbs_t: *mut (
                        Option<Box<dyn FnMut($($arg),*)>>,
                        Vec<Box<dyn FnMut($($arg),*)>>
                    ) = std::mem::transmute(self.0.[<_ $kind spi_ $function _cb>]);
                    // 获取回调列表
                    let mut cbs_t: Box<(
                        Option<Box<dyn FnMut($($arg),*)>>,
                        Vec<Box<dyn FnMut($($arg),*)>>
                    )> = Box::from_raw(cbs_t);

                    // add_on_xxx 类的方法, 添加
                    // NB: 注意此类方法目前的结构无法被删除, 小心使用
                    cbs_t.1.push(Box::new(cb));
                    tracing::debug!("[{}] 添加回调: {}_cb", cbs_t.1.len(), stringify!($function));
                    std::mem::forget(cbs_t);
                }
            } else {
                let cbs_t: Box<(
                    Option<Box<dyn FnMut($($arg),*)>>,
                    Vec<Box<dyn FnMut($($arg),*)>>
                )> = Box::new((None, vec![Box::new(cb)]));

                self.0.[<_ $kind spi_ $function _cb>] = Box::into_raw(cbs_t) as *mut std::os::raw::c_void;
            }

            self
        }
        }
    };

    ($($doc: literal)* $kind:ident->$function:ident, $($p:ident: $arg:ty),* $(,)?) => {
        ctp_fn_wrapper!($($doc)* $kind->$function, $($arg),*);
    };
}

/// 回收回调函数内存
macro_rules! ctp_fn_dropper {
    ($($doc: literal)* $self:ident, $kind:ident->$function:ident, $($arg:ty),* $(,)?) => {
        paste::paste! {

        if (!$self.[<_ $kind spi_ $function _cb>].is_null()) {
            unsafe {
                tracing::debug!(concat!("[", stringify!($kind), "]", " drop {}"), stringify!([<_ $kind spi_ $function _cb>]));
                // 转化回调指针
                let cbs_t: *mut (
                    Option<Box<dyn FnMut($($arg),*)>>,
                    Vec<Box<dyn FnMut($($arg),*)>>
                ) = std::mem::transmute($self.[<_ $kind spi_ $function _cb>]);

                // 获取回调列表
                let _: Box<(
                    Option<Box<dyn FnMut($($arg),*)>>,
                    Vec<Box<dyn FnMut($($arg),*)>>
                )> = Box::from_raw(cbs_t);
            }
        }
        }
    };

    ($($doc: literal)* $self:ident, $kind:ident->$function:ident, $($p:ident: $arg:ty),* $(,)?) => {
        ctp_fn_dropper!($($doc)* $self, $kind->$function, $($arg),*);
    }
}

macro_rules! ctp_extern {
    ($($doc: literal)* $kind:ident->$func:ident, $($arg:ident: $T:ty),* $(,)?) => {
        paste::paste! {
        #[doc = concat!("[", stringify!($kind), "] ", stringify!($func))]
        $(#[doc = $doc])*
        #[no_mangle]
        pub extern "C" fn [<rust_ $kind spi_ $func>](
            cb: &mut std::ffi::c_void,
            $($arg: $T),*
        ) {
            tracing::debug!(concat!("[I] rust_", stringify!($kind), "spi_", stringify!($func)));
            unsafe {
                let closures: &mut (
                    Option<Box<dyn FnMut($($T),*)>>,
                    Vec<Box<dyn FnMut($($T),*)>>,
                ) = std::mem::transmute(cb);

                closures.0.iter_mut().for_each(|closure| closure($($arg),*));
                closures.1.iter_mut().for_each(|closure| closure($($arg),*));
            }
        }
        }
    };
}

#[macro_export]
macro_rules! ne {
    ($m:literal) => {
        CTPError::NoneError($m)
    };

    ($m:expr) => {
        CTPError::NoneError($m.to_string().as_ref())
    };
}

pub mod ctp;
pub mod ext;
pub use ext::*;

pub fn version() -> &'static str {
    unsafe {
        let v = ctp::api::CThostFtdcTraderApi_GetApiVersion();
        CStr::from_ptr(v).to_str().unwrap()
    }
}

// log utils
#[no_mangle]
pub extern "C" fn rust_log_trace(msg: *const c_void) {
    unsafe {
        let msg = CStr::from_ptr(msg as _).to_bytes();
        tracing::trace!("{}", gbk(msg));
    }
}

#[no_mangle]
pub extern "C" fn rust_log_debug(msg: *const c_void) {
    unsafe {
        let msg = CStr::from_ptr(msg as _).to_bytes();
        tracing::debug!("{}", gbk(msg));
    }
}

#[no_mangle]
pub extern "C" fn rust_log_info(msg: *const c_void) {
    unsafe {
        let msg = CStr::from_ptr(msg as _).to_bytes();
        tracing::info!("{}", gbk(msg));
    }
}

#[no_mangle]
pub extern "C" fn rust_log_warn(msg: *const c_void) {
    unsafe {
        let msg = CStr::from_ptr(msg as _).to_bytes();
        tracing::warn!("{}", gbk(msg));
    }
}

#[no_mangle]
pub extern "C" fn rust_log_error(msg: *const c_void) {
    unsafe {
        let msg = CStr::from_ptr(msg as _).to_bytes();
        tracing::error!("{}", gbk(msg));
    }
}
