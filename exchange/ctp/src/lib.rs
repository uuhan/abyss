#![feature(lazy_cell)]
#![allow(unused)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces
)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(unknown_lints)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::cast_precision_loss)]
#![allow(renamed_and_removed_lints)]
#![allow(clippy::unknown_clippy_lints)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::similar_names)]
#![allow(clippy::mutex_atomic)]
#![allow(clippy::useless_transmute)]

//! 基于ctp-sys的高级抽象
//!
//! NOTE: CTP/C++ 接口只支持 x86_64-unknown-linux-gnu
//!
pub use abyss_subscriber::{Subscriber, Subscribers};
use auto_impl::auto_impl;
use ctp_sys::ctp::api::*;
pub use ctp_sys::ctp::error::*;
use ctp_sys::ctp::opts::*;
pub use ctp_sys::ext::*;
use std::sync::{
    atomic::{AtomicI32, Ordering::Relaxed},
    Arc, RwLock,
};

#[macro_use]
extern crate ctp_sys;

#[macro_use]
mod mac;

mod ctp;
pub use self::ctp::Api;
pub use self::ctp::CTPMarket;
pub use self::ctp::CTPTrader;
pub use self::ctp::MarketBuilder;
pub use self::ctp::Spi;
pub use self::ctp::TraderBuilder;

// utils
pub use self::ctp::prepare_ctp_trader;

/// gRPC 支持
#[cfg(feature = "grpc")]
pub mod rpc;

/// 测量支持
#[cfg(feature = "metrics")]
pub mod metrics;

pub use abyss_bouncer::Bouncer;
pub use abyss_wait::*;

/// 交易
/// ```no_run
/// use ctp::*;
/// let trader1 = CTPTrader::default();
/// let trader2 = CTPTrader::default();
/// let trader3 = CTPTrader::default();
/// let _ = (trader1, trader2, trader3).req_qry_investor_position_sync().unwrap();
/// ```
#[auto_impl(&, Box, Arc)]
pub trait Trader {
    /// 错误类型
    type Error: std::error::Error;

    /// 用于克服 CTP 限流
    /// CTP 柜台一个 session 默认允许一秒查询两次
    #[inline]
    fn turn(&self) -> u8 {
        0
    }

    /// 获取投资者持仓信息
    fn req_qry_investor_position_sync(
        &self,
    ) -> Result<Vec<CThostFtdcInvestorPositionField>, Self::Error>;

    /// 查询投资者账号信息
    fn req_qry_trading_account_sync(
        &self,
    ) -> Result<Option<CThostFtdcTradingAccountField>, Self::Error>;

    /// 获取投资者持仓详情
    fn req_qry_investor_position_detail_sync(
        &self,
    ) -> Result<Vec<CThostFtdcInvestorPositionDetailField>, Self::Error>;

    /// 查询合约信息
    fn req_qry_instrument_sync(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentField>, Self::Error>;

    /// 查询合约信息(模糊查询)
    fn req_qry_instruments_sync(
        &self,
        id: &str,
    ) -> Result<Vec<CThostFtdcInstrumentField>, Self::Error>;

    /// 查询产品信息
    fn req_qry_product_sync(&self, id: &str)
        -> Result<Vec<CThostFtdcInstrumentField>, Self::Error>;

    /// 查询合约保证金
    fn req_qry_instrument_margin_rate_sync(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentMarginRateField>, Self::Error>;

    /// 请求查询合约手续费率响应
    fn req_qry_instrument_commission_rate_sync(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentCommissionRateField>, Self::Error>;

    /// 查询行情数据
    fn req_qry_depth_market_data_sync(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcDepthMarketDataField>, Self::Error>;

    /// 报单录入请求(同步)
    /// NB: 所有交易所几乎只支持限价单报单
    fn req_order_insert_sync(
        &self,
        input_order_field: &CThostFtdcInputOrderField,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 开多仓(FAK)
    fn long_fak_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 开多仓(FOK)
    /// NB: 郑商所不支持FOK指令
    fn long_fok_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 开空仓(FAK)
    fn short_fak_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 开空仓(FOK)
    /// NB: 郑商所不支持FOK指令
    fn short_fok_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 平仓(同步)
    fn close_sync(
        &self,
        position: &CThostFtdcInvestorPositionField,
        price: Option<f64>,
        volume: Option<i32>,
    ) -> Result<Option<(f64, i32)>, Self::Error>;

    /// 合约可交易状态
    fn instrument_wait_status(
        &self,
        product_id: &str,
    ) -> Arc<dyn Fn() -> Option<InstrumentStatus> + Send + Sync>;

    /// 合约可交易状态
    fn instrument_get_status(
        &self,
        product_id: &str,
        status: InstrumentStatus,
    ) -> Arc<dyn Fn() -> InstrumentStatus + Send + Sync>;

    /// 合约可交易状态(异步)
    fn instrument_watch_status(
        &self,
        product_id: &str,
    ) -> Subscriber<CThostFtdcInstrumentStatusField>;

    /// 获取当前交易日
    /// @retrun 获取到的交易日
    /// @remark 只有登录成功后,才能得到正确的交易日
    fn get_trading_day(&self) -> Result<String, Self::Error>;

    /// 等待交易就绪
    fn ready(&self) -> Result<(), Self::Error>;
}

/// Async Trader
#[cfg(feature = "async")]
#[async_trait::async_trait]
#[auto_impl(&, Box)]
pub trait AsyncTrader {
    /// 错误类型
    type Error: std::error::Error;

    /// 用于克服 CTP 限流
    /// CTP 柜台一个 session 默认允许一秒查询两次
    #[inline]
    fn turn(&self) -> u8 {
        0
    }

    /// 获取投资者持仓信息
    async fn req_qry_investor_position_async(
        &self,
    ) -> Result<Vec<CThostFtdcInvestorPositionField>, Self::Error> {
        todo!()
    }

    /// 查询投资者账号信息
    #[allow(clippy::type_complexity)]
    async fn req_qry_trading_account_async(
        &self,
    ) -> Result<Option<CThostFtdcTradingAccountField>, Self::Error>;

    /// 获取投资者持仓详情
    async fn req_qry_investor_position_detail_async(
        &self,
    ) -> Result<Vec<CThostFtdcInvestorPositionDetailField>, Self::Error> {
        todo!()
    }

    /// 查询合约信息
    async fn req_qry_instrument_async(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentField>, Self::Error> {
        todo!()
    }

    /// 查询合约信息(模糊查询)
    async fn req_qry_instruments_async(
        &self,
        id: &str,
    ) -> Result<Vec<CThostFtdcInstrumentField>, Self::Error> {
        todo!()
    }

    /// 查询产品信息
    async fn req_qry_product_async(
        &self,
        id: &str,
    ) -> Result<Vec<CThostFtdcInstrumentField>, Self::Error> {
        todo!()
    }

    /// 查询合约保证金
    async fn req_qry_instrument_margin_rate_async(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentMarginRateField>, Self::Error> {
        todo!()
    }

    /// 请求查询合约手续费率响应
    async fn req_qry_instrument_commission_rate_async(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcInstrumentCommissionRateField>, Self::Error> {
        todo!()
    }

    /// 查询行情数据
    async fn req_qry_depth_market_data_async(
        &self,
        id: &str,
    ) -> Result<Option<CThostFtdcDepthMarketDataField>, Self::Error> {
        todo!()
    }

    /// 报单录入请求(同步)
    /// NB: 所有交易所几乎只支持限价单报单
    async fn req_order_insert_async(
        &self,
        input_order_field: &CThostFtdcInputOrderField,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }

    /// 开多仓(FAK)
    async fn long_fak_async(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }

    /// 开多仓(FOK)
    /// NB: 郑商所不支持FOK指令
    async fn long_fok_async(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }

    /// 开空仓(FAK)
    async fn short_fak_async(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }

    /// 开空仓(FOK)
    /// NB: 郑商所不支持FOK指令
    async fn short_fok_async(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }

    /// 平仓(同步)
    async fn close_async(
        &self,
        position: &CThostFtdcInvestorPositionField,
        price: Option<f64>,
    ) -> Result<Option<(f64, i32)>, Self::Error> {
        todo!()
    }
}

/// API 模块
pub mod api {
    pub use ctp_sys::ctp::api::*;
    pub use ctp_sys::ctp::field;
    pub use ctp_sys::ctp::opts;

    // NB: macos 使用 openctp 库
    cfg_if::cfg_if! {
        if #[cfg(target_arch="x86_64")] {
            pub use ctp_sys::version;
        } else {
            /// 此平台不支持CTP
            #[must_use]
            pub const fn version() -> &'static str {
                "NO CTP ENABLED."
            }
        }
    }
}

/// 生成唯一 ID
pub fn id() -> i32 {
    static ID: AtomicI32 = AtomicI32::new(1024);
    ID.fetch_add(1, Relaxed)
}

mod compile_time_assertions {
    use super::*;

    #[allow(unreachable_code)]
    fn assert_database_send_sync() {
        _assert_send_sync::<CTPMarket>(unreachable!());
        _assert_send_sync::<CTPTrader>(unreachable!());
    }

    fn _assert_send<S: Send>(_: &S) {}
    fn _assert_send_sync<S: Send + Sync>(_: &S) {}
}
