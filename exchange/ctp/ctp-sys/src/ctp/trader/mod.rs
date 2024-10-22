use super::api::*;
pub mod api;
pub mod spi;

#[derive(Debug)]
pub struct TraderApi(&'static mut CThostFtdcTraderApi);
#[derive(Debug)]
pub struct TraderSpi(&'static mut CTPTraderSpi);

unsafe impl Send for TraderApi {}
unsafe impl Send for TraderSpi {}

impl Drop for TraderApi {
    fn drop(&mut self) {
        tracing::debug!("Drop TraderApi");
        unsafe { TraderApi_Release(self.0) }
    }
}

impl Drop for TraderSpi {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctp::api::*;
    use crate::ctp::*;
    use crate::ext::{cstr, gbk};
}
