#![allow(unused)]
//// linux x86_64 only
use super::*;
use crossbeam::channel::bounded;
use ctp_sys::ctp::api::*;
use ctp_sys::ctp::error::*;
use ctp_sys::ctp::{MarketApi, MarketSpi};
use parking_lot::{Condvar, Mutex};
use std::net::TcpStream;
use std::sync::{
    atomic::{AtomicBool, Ordering::Relaxed},
    Arc,
};

#[cfg(feature = "node")]
pub mod node;

mod utils;
pub use utils::*;

/// CTP Market 构建参数
#[derive(Default, Debug, Clone)]
pub struct MarketBuilder {
    pub front: String,
    pub log_path: String,
    pub broker_id: String,
    pub user_id: String,
    pub user_password: String,
    pub is_udp: bool,
    pub is_multicast: bool,
}

impl MarketBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_front(mut self, front: impl AsRef<str>) -> Self {
        self.front = front.as_ref().to_string();
        self
    }

    pub fn with_broker_id(mut self, broker_id: impl AsRef<str>) -> Self {
        self.broker_id = broker_id.as_ref().to_string();
        self
    }
    pub fn with_user_id(mut self, user_id: impl AsRef<str>) -> Self {
        self.user_id = user_id.as_ref().to_string();
        self
    }

    pub fn with_user_password(mut self, user_password: impl AsRef<str>) -> Self {
        self.user_password = user_password.as_ref().to_string();
        self
    }

    pub fn with_log_path(mut self, log_path: impl AsRef<str>) -> Self {
        self.log_path = log_path.as_ref().to_string();
        self
    }

    #[must_use]
    pub const fn with_udp(mut self, is_udp: bool) -> Self {
        self.is_udp = is_udp;
        self
    }

    #[must_use]
    pub const fn with_multicast(mut self, is_multicast: bool) -> Self {
        self.is_multicast = is_multicast;
        self
    }

    #[allow(unsafe_code)]
    pub fn build(self) -> CTPMarket {
        let mut api = unsafe { MarketApi::new(&self.log_path, self.is_udp, self.is_multicast) };
        let mut spi = unsafe { MarketSpi::new() };
        api.register_spi(&spi);

        let connected = Wait::new();
        let ready = Wait::new();

        let mut is_connected = connected.clone();
        let mut is_ready = ready.clone();
        spi.add_on_front_disconnected(move |_| {
            tracing::debug!("[CTPMarket.connected] set to false.");
            tracing::debug!("[CTPMarket.ready] set to false.");
            is_connected.wake(false);
            is_ready.wake(false);
        });

        let mut is_connected = connected.clone();
        spi.add_on_front_connected(move || {
            tracing::debug!("[CTPMarket.connected] set to true.");
            is_connected.wake(true);
        });

        let mut is_ready = ready.clone();
        spi.add_on_rsp_user_login(move |_, error, _, _| {
            if error.ErrorID == 0 {
                tracing::debug!("[CTPMarket.ready] set to true.");
                is_ready.wake(true);
            }
        });

        let mut is_ready = ready.clone();
        spi.add_on_rsp_user_logout(move |_, error, _, _| {
            if error.ErrorID == 0 {
                tracing::debug!("[CTPMarket.ready] set to false.");
                is_ready.wake(false);
            }
        });

        CTPMarket {
            api: Arc::new(Mutex::new(api)),
            spi: Arc::new(Mutex::new(spi)),
            env: Arc::new(self),
            connected,
            ready,
        }
    }
}

type Api = Arc<Mutex<MarketApi>>;
type Spi = Arc<Mutex<MarketSpi>>;

/// CTP Market 封装
#[derive(Debug)]
pub struct CTPMarket {
    pub(crate) api: Api,
    pub(crate) spi: Spi,
    /// 账户信息
    pub env: Arc<MarketBuilder>,
    /// 行情前置已连接
    pub(crate) connected: Wait<bool>,
    /// 行情前置已就绪
    pub(crate) ready: Wait<bool>,
}

impl CTPMarket {
    /// 行情是否就绪
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.ready.get().unwrap_or(false)
    }

    #[must_use]
    /// 行情是否连接
    pub fn is_connected(&self) -> bool {
        self.connected.get().unwrap_or(false)
    }
}

impl Default for CTPMarket {
    fn default() -> Self {
        CTPMarket::builder()
            .with_log_path("log/")
            .with_udp(false)
            .with_multicast(false)
            .with_front(env::var("CTP_MARKET_FRONT").unwrap())
            .with_broker_id(env::var("CTP_BROKER_ID").unwrap())
            .with_user_id(env::var("CTP_USER_ID").unwrap())
            .with_user_password(env::var("CTP_USER_PASSWORD").unwrap())
            .build()
    }
}

impl CTPMarket {
    /// 行情前置构造器
    #[must_use]
    pub fn builder() -> MarketBuilder {
        MarketBuilder::default()
    }

    /// 注册前置机网络地址
    /// @param pszFrontAddress：前置机网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
    #[allow(clippy::missing_panics_doc)]
    pub fn register_front(&self, front: impl Into<String>) -> &Self {
        self.api.lock().register_front(front);
        self
    }

    /// 初始化
    /// @remark 初始化运行环境,只有调用后,接口才开始工作
    #[allow(clippy::missing_panics_doc)]
    pub fn init(&self) {
        self.api.lock().init();
    }

    /// 等待行情线程返回
    /// @return 线程退出代码
    pub fn join(&self) -> CTPApiResult {
        self.api.lock().join()
    }

    // 删除接口对象本身
    // @remark 不再使用本接口对象时,调用该函数删除接口对象
    // fn release(self) {
    //     self.api.lock().release();
    // }

    /// 获取当前交易日
    /// @retrun 获取到的交易日
    /// @remark 只有登录成功后,才能得到正确的交易日
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn get_trading_day(&self) -> String {
        self.api.lock().get_trading_day().to_owned()
    }

    /// 注册名字服务器网络地址
    /// @param pszNsAddress：名字服务器网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
    /// @remark `RegisterNameServer` 优先于 `RegisterFront`
    #[allow(clippy::missing_panics_doc)]
    pub fn register_name_server(&self, ns: impl Into<String>) -> &Self {
        self.api.lock().register_name_server(ns);
        self
    }

    /// 注册名字服务器用户信息
    /// @param pFensUserInfo：用户信息。
    #[allow(clippy::missing_panics_doc, clippy::must_use_candidate)]
    pub fn register_fens_user_info(&self, info: &CThostFtdcFensUserInfoField) -> &Self {
        self.api.lock().register_fens_user_info(info);
        self
    }

    /// 注册回调接口
    /// @param pSpi 派生自回调接口类的实例
    fn register_spi(&self, spi: &mut MarketSpi) -> &Self {
        self.api.lock().register_spi(spi);
        self
    }

    /// 订阅行情。
    /// @param instruments 合约ID
    /// @param nCount 要订阅/退订行情的合约个数
    /// @remark
    pub fn subscribe_market_data<S: ToString>(&self, instruments: &[S]) -> CTPApiResult {
        self.api.lock().subscribe_market_data(instruments)
    }

    /// 退订行情。
    /// @param instruments 合约ID
    /// @param nCount 要订阅/退订行情的合约个数
    /// @remark
    pub fn unsubscribe_market_data<S: ToString>(&self, instruments: &[S]) -> CTPApiResult {
        self.api.lock().unsubscribe_market_data(instruments)
    }

    /// 订阅询价。
    /// @param instruments 合约ID
    /// @param nCount 要订阅/退订行情的合约个数
    /// @remark
    pub fn subscribe_for_quote_rsp<S: ToString>(&self, instruments: &[S]) -> CTPApiResult {
        self.api.lock().subscribe_for_quote_rsp(instruments)
    }

    /// 退订询价。
    /// @param instruments 合约ID
    /// @param nCount 要订阅/退订行情的合约个数
    /// @remark
    pub fn unsubscribe_for_quote_rsp<S: ToString>(&self, instruments: &[S]) -> CTPApiResult {
        self.api.lock().unsubscribe_for_quote_rsp(instruments)
    }

    req_ctp_function! {
        "用户登录请求"
        md->req_user_login,
        user_login_field: &CThostFtdcReqUserLoginField, request_id: i32
    }

    req_ctp_function! {
        "登出请求"
        md->req_user_logout,
        user_logout_field: &CThostFtdcUserLogoutField, request_id: i32
    }

    #[cfg(feature = "v6_3_19")]
    req_ctp_function! {
        "请求查询组播合约"
        md->req_qry_multicast_instrument,
        field: &CThostFtdcQryMulticastInstrumentField, request_id: i32
    }

    on_ctp_function! {
        "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
        md->on_front_connected,
    }

    on_ctp_function! {
        "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。"
        "@param nReason 错误原因"
        "        0x1001 网络读失败"
        "        0x1002 网络写失败"
        "        0x2001 接收心跳超时"
        "        0x2002 发送心跳失败"
        "        0x2003 收到错误报文"
        md->on_front_disconnected,
        reason: i32,
    }

    // /// 心跳超时警告。当长时间未收到报文时，该方法被调用。
    // /// @param nTimeLapse 距离上次接收报文的时间
    // pub fn on_heartbeat_warning<F>(&self, mut cb: F) -> &Self
    //     where F: FnMut(Api, i32) -> CTPApiResult
    // {
    //     let api = self.api.clone();
    //     self.spi.lock().on_heartbeat_warning(move |time_lapse| {
    //         let api = api.clone();
    //         ctp_error_wrapper!(cb(api, time_lapse));
    //     });
    //     self
    // }

    on_ctp_function! {
        "错误应答"
        md->on_rsp_error,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    #[cfg(feature = "v6_3_19")]
    on_ctp_function! {
        "请求查询组播合约响应"
        md->on_rsp_qry_multicast_instrument,
        field: &CThostFtdcMulticastInstrumentField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "订阅询价应答"
        md->on_rsp_sub_for_quote_rsp,
        field: &CThostFtdcSpecificInstrumentField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "订阅行情应答"
        md->on_rsp_sub_market_data,
        field: &CThostFtdcSpecificInstrumentField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "取消订阅询价应答"
        md->on_rsp_unsub_for_quote_rsp,
        field: &CThostFtdcSpecificInstrumentField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "取消订阅行情应答"
        md->on_rsp_unsub_market_data,
        field: &CThostFtdcSpecificInstrumentField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "登录请求响应"
        md->on_rsp_user_login,
        field: &CThostFtdcRspUserLoginField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "登出请求响应"
        md->on_rsp_user_logout,
        field: &CThostFtdcUserLogoutField,
        error: &CThostFtdcRspInfoField,
        id: i32, last: bool,
    }

    on_ctp_function! {
        "深度行情通知"
        md->on_rtn_depth_market_data,
        field: &CThostFtdcDepthMarketDataField,
    }

    on_ctp_function! {
        "询价通知"
        md->on_rtn_for_quote_rsp,
        field: &CThostFtdcForQuoteRspField,
    }
}
