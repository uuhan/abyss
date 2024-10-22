use super::api::*;
pub mod api;
pub mod spi;

#[derive(Debug)]
pub struct MarketApi(&'static mut CThostFtdcMdApi);
#[derive(Debug)]
pub struct MarketSpi(&'static mut CTPMarketSpi);

unsafe impl Send for MarketApi {}
unsafe impl Send for MarketSpi {}

impl Drop for MarketApi {
    fn drop(&mut self) {
        unsafe { MdApi_Release(self.0) }
    }
}

impl Drop for MarketSpi {
    fn drop(&mut self) {
        unsafe { std::ptr::drop_in_place(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctp::api::*;
    use crate::ctp::*;
}
