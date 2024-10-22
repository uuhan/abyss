#![allow(unused, non_snake_case)]
// linux x86_64 only
use super::*;
use crate::api::opts::*;
use crate::api::*;
use crate::*;
use crossbeam::channel::{unbounded, Receiver, Sender};
use ctp_sys::ctp::{TraderApi, TraderSpi};
use parking_lot::{Condvar, Mutex, RwLock};
use std::any::Any;
use std::net::TcpStream;
use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, LazyLock,
    },
};

mod utils;
use abyss_subscriber::*;
pub use utils::*;
#[cfg(feature = "node")]
pub mod node;

/// CTP Trader 构建参数
#[derive(Default, Debug, Clone)]
pub struct TraderBuilder {
    pub front: String,
    pub log_path: String,
    pub broker_id: String,
    pub user_id: String,
    pub user_password: String,
    pub app_id: String,
    pub auth_token: String,
}

impl TraderBuilder {
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

    pub fn with_app_id(mut self, app_id: impl AsRef<str>) -> Self {
        self.app_id = app_id.as_ref().to_string();
        self
    }

    pub fn with_auth_token(mut self, auth_token: impl AsRef<str>) -> Self {
        self.auth_token = auth_token.as_ref().to_string();
        self
    }

    pub fn with_log_path(mut self, log_path: impl AsRef<str>) -> Self {
        self.log_path = log_path.as_ref().to_string();
        self
    }

    #[allow(unsafe_code)]
    pub fn build(self) -> CTPTrader {
        let mut api = unsafe { TraderApi::new(&self.log_path) };
        let mut spi = unsafe { TraderSpi::new() };

        api.register_spi(&spi);
        let subscribers = Arc::new(Subscribers::new());

        let connected = Wait::<bool>::new();
        let ready = Wait::<bool>::new();

        let mut is_connected = connected.clone();
        let mut is_ready = ready.clone();
        // 服务器断开连接
        spi.add_on_front_disconnected(move |_| {
            tracing::debug!("[CTPTrader.connected] set to false.");
            tracing::debug!("[CTPTrader.ready] set to false.");
            is_connected.wake(false);
            is_ready.wake(false);
        });

        let mut is_connected = connected.clone();
        // 服务器连接
        spi.add_on_front_connected(move || {
            tracing::debug!("[CTPTrader.connected] set to true.");
            is_connected.wake(true);
        });

        let mut is_ready = ready.clone();
        // 登陆成功, 可以进行 ctp 请求
        spi.add_on_rsp_user_login(move |_, error, _, _| {
            if error.ErrorID == 0 {
                tracing::debug!("[CTPTrader.ready] set to true.");
                is_ready.wake(true);
            }
        });

        let mut is_ready = ready.clone();
        // 登出账号, ctp 请求不可用
        spi.add_on_rsp_user_logout(move |_, error, _, _| {
            if error.ErrorID == 0 {
                tracing::debug!("[CTPTrader.ready] set to false.");
                is_ready.wake(false);
            }
        });

        // 订阅合约状态
        let subs = subscribers.clone();
        spi.add_on_rtn_instrument_status(move |status| {
            // 品种ID
            let pid = s(&status.InstrumentID);
            if let Some(broadcast) = subs.exact(pid) {
                tracing::debug!("[{}] broadcast: {}", pid, status.InstrumentStatus);
                broadcast.complete(status);
            }
        });

        // spi.add_on_rtn_trading_notice(move |notice| {
        //     tracing::info!(
        //         "[notice] {} {} {}",
        //         ctp::s(&notice.SendTime),
        //         notice.SequenceSeries,
        //         ctp::s(&notice.FieldContent),
        //     );
        // });

        // let (tx, rx) = unbounded();

        CTPTrader {
            api: Arc::new(Mutex::new(api)),
            spi: Arc::new(Mutex::new(spi)),
            env: Arc::new(self),
            lock: Arc::new(Mutex::new(())),
            connected,
            ready,
            subscribers,
            // fut_tx: Arc::new(tx),
            // fut_rx: Arc::new(rx),
            bouncer: Arc::new(LazyLock::new(|| {
                Mutex::new(Bouncer::new(Duration::from_secs(1)))
            })),
        }
    }
}

/// Api 类型
pub type Api = Arc<Mutex<TraderApi>>;
/// Spi 类型
pub type Spi = Arc<Mutex<TraderSpi>>;

/// CTP Trader 封装
#[derive(Clone, Debug)]
pub struct CTPTrader {
    pub(crate) api: Api,
    pub(crate) spi: Spi,

    /// 账户信息
    pub env: Arc<TraderBuilder>,
    /// 节流 & 防抖
    pub bouncer: Arc<LazyLock<Mutex<Bouncer<CTPApiResult>>>>,

    // pub fut_tx: Arc<Sender<FutWaker>>,
    // pub fut_rx: Arc<Receiver<FutWaker>>,
    /// 消息订阅器
    pub(crate) subscribers: Arc<Subscribers<CThostFtdcInstrumentStatusField>>,
    /// 独占请求
    pub(crate) lock: Arc<Mutex<()>>,
    /// 交易前置已连接
    pub(crate) connected: Wait<bool>,
    /// 交易会话已就绪
    pub(crate) ready: Wait<bool>,
}

impl CTPTrader {
    /// 交易是否就绪
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn is_ready(&self) -> bool {
        self.ready.get().unwrap_or(false)
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    /// 交易是否连接
    pub fn is_connected(&self) -> bool {
        self.connected.get().unwrap_or(false)
    }
}

impl Default for CTPTrader {
    fn default() -> Self {
        let trader = CTPTrader::builder()
            .with_log_path("log/")
            .with_front(env::var("CTP_TRADER_FRONT").unwrap())
            .with_broker_id(env::var("CTP_BROKER_ID").unwrap())
            .with_app_id(env::var("CTP_APP_ID").unwrap())
            .with_user_id(env::var("CTP_USER_ID").unwrap())
            .with_user_password(env::var("CTP_USER_PASSWORD").unwrap())
            .with_auth_token(env::var("CTP_AUTH_TOKEN").unwrap())
            .build();

        // 默认的 Trader 仅供查询使用, 忽略绝大多数的响应
        trader.subscribe_private_topic(TopicTertFlag::Quick);
        trader.subscribe_public_topic(TopicTertFlag::Quick);

        // TODO: 忽略更多的消息
        trader.on_rtn_instrument_status(empty!(_, _));
        trader.on_err_rtn_order_insert(empty!(_, _, _));
        trader.on_err_rtn_order_action(empty!(_, _, _));
        trader.on_rtn_order(empty!(_, _));
        trader.on_rtn_trade(empty!(_, _));
        trader.on_rtn_trading_notice(empty!(_, _));
        trader.on_rsp_error(empty!(_, _, _, _));
        trader.add_on_front_disconnected(move |_, reason| Ok(()));

        trader
    }
}

impl CTPTrader {
    /// 交易前置构造器
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn builder() -> TraderBuilder {
        TraderBuilder::default()
    }

    /// 注册前置机网络地址
    /// @param pszFrontAddress：前置机网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
    #[allow(clippy::missing_panics_doc)]
    pub fn register_front<Front: Into<String>>(&self, front: Front) -> &Self {
        self.api.lock().register_front(front);
        self
    }

    /// 初始化行情连接
    #[allow(clippy::missing_panics_doc)]
    pub fn init(&self) {
        self.api.lock().init();
    }

    /// 等待行情线程返回
    pub fn join(&self) -> CTPApiResult {
        self.api.lock().join()
    }

    /// 注册名字服务器网络地址
    /// @param pszNsAddress：名字服务器网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
    /// @remark `RegisterNameServer` 优先于 `RegisterFront`
    #[allow(clippy::missing_panics_doc)]
    pub fn register_name_server<S: Into<String>>(&self, ns: S) -> &Self {
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

    /// 订阅私有流。
    /// @param `resume_type` 私有流重传方式
    ///         `THOST_TERT_RESTART`:从本交易日开始重传
    ///         `THOST_TERT_RESUME`:从上次收到的续传
    ///         `THOST_TERT_QUICK`:只传送登录后私有流的内容
    /// @remark 该方法要在Init方法前调用。若不调用则不会收到私有流的数据。
    #[allow(clippy::missing_panics_doc, clippy::must_use_candidate)]
    pub fn subscribe_private_topic(&self, resume_type: TopicTertFlag) -> &Self {
        self.api.lock().subscribe_private_topic(resume_type);
        self
    }

    /// 订阅公共流。
    /// @param `resume_type` 公共流重传方式
    ///         `THOST_TERT_RESTART`:从本交易日开始重传
    ///         `THOST_TERT_RESUME`:从上次收到的续传
    ///         `THOST_TERT_QUICK`:只传送登录后公共流的内容
    /// @remark 该方法要在Init方法前调用。若不调用则不会收到公共流的数据。
    #[allow(clippy::missing_panics_doc, clippy::must_use_candidate)]
    pub fn subscribe_public_topic(&self, resume_type: TopicTertFlag) -> &Self {
        self.api.lock().subscribe_public_topic(resume_type);
        self
    }

    req_ctp_function! {
        "客户端认证请求"
        trader->req_authenticate,
        field: &CThostFtdcReqAuthenticateField,
        id: i32
    }

    req_ctp_function! {
        "用户登录请求"
        trader->req_user_login,
        field: &CThostFtdcReqUserLoginField,
        id: i32
    }

    req_ctp_function! {
        "用户登出请求"
        trader->req_user_logout,
        field: &CThostFtdcUserLogoutField,
        id: i32
    }

    req_ctp_function! {
        "用户口令更新请求"
        trader->req_user_password_update,
        user_password_update_field: &CThostFtdcUserPasswordUpdateField, request_id: i32
    }

    req_ctp_function! {
        "资金账户口令更新请求"
        trader->req_trading_account_password_update,
        trading_account_password_update_field: &CThostFtdcTradingAccountPasswordUpdateField, request_id: i32
    }

    req_ctp_function! {
        "报单录入请求"
        trader->req_order_insert,
        input_order_field: &CThostFtdcInputOrderField, request_id: i32
    }

    req_ctp_function! {
        "预埋单录入请求"
        trader->req_parked_order_insert,
        parked_order_field: &CThostFtdcParkedOrderField, request_id: i32
    }

    req_ctp_function! {
        "预埋撤单录入请求"
        trader->req_parked_order_action,
        parked_order_action_field: &CThostFtdcParkedOrderActionField, request_id: i32
    }

    req_ctp_function! {
        "报单操作请求"
        trader->req_order_action,
        input_order_action_field: &CThostFtdcInputOrderActionField, request_id: i32
    }

    req_ctp_function! {
        "查询最大报单数量请求"
        trader->req_query_max_order_volume,
        query_max_order_volume: &CThostFtdcQueryMaxOrderVolumeField, request_id: i32
    }

    req_ctp_function! {
        "投资者结算结果确认"
        trader->req_settlement_info_confirm,
        settlement_info_confirm_field: &CThostFtdcSettlementInfoConfirmField, request_id: i32
    }

    req_ctp_function! {
        "请求删除预埋单"
        trader->req_remove_parked_order,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderField, nRequestID: i32
    }

    req_ctp_function! {
        "请求删除预埋撤单"
        trader->req_remove_parked_order_action,
        pRemoveParkedOrderAction: &CThostFtdcRemoveParkedOrderActionField, nRequestID: i32
    }

    req_ctp_function! {
        "执行宣告录入请求"
        trader->req_exec_order_insert,
        pInputExecOrder: &CThostFtdcInputExecOrderField, nRequestID: i32
    }

    req_ctp_function! {
        "执行宣告操作请求"
        trader->req_exec_order_action,
        pInputExecOrderAction: &CThostFtdcInputExecOrderActionField, nRequestID: i32
    }

    req_ctp_function! {
        "询价录入请求"
        trader->req_for_quote_insert,
        pInputForQuote: &CThostFtdcInputForQuoteField, nRequestID: i32
    }

    req_ctp_function! {
        "报价录入请求"
        trader->req_quote_insert,
        pInputQuote: &CThostFtdcInputQuoteField, nRequestID: i32
    }

    req_ctp_function! {
        "报价操作请求"
        trader->req_quote_action,
        pInputQuoteAction: &CThostFtdcInputQuoteActionField, nRequestID: i32
    }

    req_ctp_function! {
        "批量报单操作请求"
        trader->req_batch_order_action,
        pInputBatchOrderAction: &CThostFtdcInputBatchOrderActionField, nRequestID: i32
    }

    req_ctp_function! {
        "申请组合录入请求"
        trader->req_comb_action_insert,
        pInputCombAction: &CThostFtdcInputCombActionField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询报单"
        T trader->req_qry_order,
        pQryOrder: &CThostFtdcQryOrderField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询成交"
        T trader->req_qry_trade,
        pQryTrade: &CThostFtdcQryTradeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资者持仓"
        T trader->req_qry_investor_position,
        qry_investor_position_field: &CThostFtdcQryInvestorPositionField, request_id: i32
    }

    req_ctp_function! {
        "请求查询资金账户"
        T trader->req_qry_trading_account,
        qry_trading_account_field: &CThostFtdcQryTradingAccountField, request_id: i32
    }

    req_ctp_function! {
        "请求查询投资者账户"
        T trader->req_qry_investor,
        qry_investor_field: &CThostFtdcQryInvestorField, request_id: i32
    }

    req_ctp_function! {
        "请求查询交易编码"
        T trader->req_qry_trading_code,
        pQryTradingCode: &CThostFtdcQryTradingCodeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询合约保证金率"
        T trader->req_qry_instrument_margin_rate,
        pQryInstrumentMarginRate: &CThostFtdcQryInstrumentMarginRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询合约手续费率"
        T trader->req_qry_instrument_commission_rate,
        pQryInstrumentCommissionRate: &CThostFtdcQryInstrumentCommissionRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询交易所"
        T trader->req_qry_exchange,
        pQryExchange: &CThostFtdcQryExchangeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询产品"
        T trader->req_qry_product,
        pQryProduct: &CThostFtdcQryProductField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询合约"
        T trader->req_qry_instrument,
        pQryInstrument: &CThostFtdcQryInstrumentField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询行情"
        T trader->req_qry_depth_market_data,
        pQryDepthMarketData: &CThostFtdcQryDepthMarketDataField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资者结算结果"
        T trader->req_qry_settlement_info,
        pQrySettlementInfo: &CThostFtdcQrySettlementInfoField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询转帐银行"
        T trader->req_qry_transfer_bank,
        pQryTransferBank: &CThostFtdcQryTransferBankField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资者持仓明细"
        T trader->req_qry_investor_position_detail,
        pQryInvestorPositionDetail: &CThostFtdcQryInvestorPositionDetailField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询客户通知"
        T trader->req_qry_notice,
        pQryNotice: &CThostFtdcQryNoticeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询结算信息确认"
        T trader->req_qry_settlement_info_confirm,
        pQrySettlementInfoConfirm: &CThostFtdcQrySettlementInfoConfirmField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资者持仓明细"
        T trader->req_qry_investor_position_combine_detail,
        pQryInvestorPositionCombineDetail: &CThostFtdcQryInvestorPositionCombineDetailField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询保证金监管系统经纪公司资金账户密钥"
        T trader->req_qry_cfmmc_trading_account_key,
        pQryCFMMCTradingAccountKey: &CThostFtdcQryCFMMCTradingAccountKeyField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询仓单折抵信息"
        T trader->req_qry_e_warrant_offset,
        pQryEWarrantOffset: &CThostFtdcQryEWarrantOffsetField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资者品种/跨品种保证金"
        T trader->req_qry_investor_product_group_margin,
        pQryInvestorProductGroupMargin: &CThostFtdcQryInvestorProductGroupMarginField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询交易所保证金率"
        T trader->req_qry_exchange_margin_rate,
        pQryExchangeMarginRate: &CThostFtdcQryExchangeMarginRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询交易所调整保证金率"
        T trader->req_qry_exchange_margin_rate_adjust,
        pQryExchangeMarginRateAdjust: &CThostFtdcQryExchangeMarginRateAdjustField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询汇率"
        T trader->req_qry_exchange_rate,
        pQryExchangeRate: &CThostFtdcQryExchangeRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询二级代理操作员银期权限"
        T trader->req_qry_sec_agent_acid_map,
        pQrySecAgentACIDMap: &CThostFtdcQrySecAgentACIDMapField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询产品报价汇率"
        T trader->req_qry_product_exch_rate,
        pQryProductExchRate: &CThostFtdcQryProductExchRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询产品组"
        T trader->req_qry_product_group,
        pQryProductGroup: &CThostFtdcQryProductGroupField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询做市商合约手续费率"
        T trader->req_qry_mm_instrument_commission_rate,
        pQryMMInstrumentCommissionRate: &CThostFtdcQryMMInstrumentCommissionRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询做市商期权合约手续费"
        T trader->req_qry_mm_option_instr_comm_rate,
        pQryMMOptionInstrCommRate: &CThostFtdcQryMMOptionInstrCommRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询报单手续费"
        T trader->req_qry_instrument_order_comm_rate,
        pQryInstrumentOrderCommRate: &CThostFtdcQryInstrumentOrderCommRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询资金账户"
        T trader->req_qry_sec_agent_trading_account,
        pQryTradingAccount: &CThostFtdcQryTradingAccountField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询二级代理商资金校验模式"
        T trader->req_qry_sec_agent_check_mode,
        pQrySecAgentCheckMode: &CThostFtdcQrySecAgentCheckModeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询二级代理商信息"
        T trader->req_qry_sec_agent_trade_info,
        pQrySecAgentTradeInfo: &CThostFtdcQrySecAgentTradeInfoField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询期权交易成本"
        T trader->req_qry_option_instr_trade_cost,
        pQryOptionInstrTradeCost: &CThostFtdcQryOptionInstrTradeCostField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询期权合约手续费"
        T trader->req_qry_option_instr_comm_rate,
        pQryOptionInstrCommRate: &CThostFtdcQryOptionInstrCommRateField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询执行宣告"
        T trader->req_qry_exec_order,
        pQryExecOrder: &CThostFtdcQryExecOrderField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询询价"
        T trader->req_qry_for_quote,
        pQryForQuote: &CThostFtdcQryForQuoteField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询报价"
        T trader->req_qry_quote,
        pQryQuote: &CThostFtdcQryQuoteField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询期权自对冲"
        T trader->req_qry_option_self_close,
        pQryOptionSelfClose: &CThostFtdcQryOptionSelfCloseField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询投资单元"
        T trader->req_qry_invest_unit,
        pQryInvestUnit: &CThostFtdcQryInvestUnitField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询组合合约安全系数"
        T trader->req_qry_comb_instrument_guard,
        pQryCombInstrumentGuard: &CThostFtdcQryCombInstrumentGuardField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询申请组合"
        T trader->req_qry_comb_action,
        pQryCombAction: &CThostFtdcQryCombActionField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询转帐流水"
        T trader->req_qry_transfer_serial,
        pQryTransferSerial: &CThostFtdcQryTransferSerialField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询银期签约关系"
        T trader->req_qry_account_register,
        pQryAccountregister: &CThostFtdcQryAccountregisterField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询签约银行"
        T trader->req_qry_contract_bank,
        pQryContractBank: &CThostFtdcQryContractBankField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询预埋单"
        T trader->req_qry_parked_order,
        pQryParkedOrder: &CThostFtdcQryParkedOrderField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询预埋撤单"
        T trader->req_qry_parked_order_action,
        pQryParkedOrderAction: &CThostFtdcQryParkedOrderActionField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询交易通知"
        T trader->req_qry_trading_notice,
        pQryTradingNotice: &CThostFtdcQryTradingNoticeField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询经纪公司交易参数"
        T trader->req_qry_broker_trading_params,
        pQryBrokerTradingParams: &CThostFtdcQryBrokerTradingParamsField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询经纪公司交易算法"
        T trader->req_qry_broker_trading_algos,
        pQryBrokerTradingAlgos: &CThostFtdcQryBrokerTradingAlgosField, nRequestID: i32
    }

    req_ctp_function! {
        "请求查询监控中心用户令牌"
        trader->req_query_cfmmc_trading_account_token,
        pQueryCFMMCTradingAccountToken: &CThostFtdcQueryCFMMCTradingAccountTokenField, nRequestID: i32
    }

    req_ctp_function! {
        "期货发起银行资金转期货请求"
        trader->req_from_bank_to_future_by_future,
        pReqTransfer: &CThostFtdcReqTransferField, nRequestID: i32
    }

    req_ctp_function! {
        "期货发起期货资金转银行请求"
        trader->req_from_future_to_bank_by_future,
        pReqTransfer: &CThostFtdcReqTransferField, nRequestID: i32
    }

    req_ctp_function! {
        "期货发起查询银行余额请求"
        trader->req_query_bank_account_money_by_future,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField, nRequestID: i32
    }

    // ======== on_xxx ========

    on_ctp_function! {
        "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
        trader->on_front_connected,
    }

    on_ctp_function! {
        "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，Api会自动重新连接，客户端可不做处理。"
        "@param nReason 错误原因"
        "        0x1001 网络读失败"
        "        0x1002 网络写失败"
        "        0x2001 接收心跳超时"
        "        0x2002 发送心跳失败"
        "        0x2003 收到错误报文"
        trader->on_front_disconnected,
        reason: i32,
    }

    // /// 心跳超时警告。当长时间未收到报文时，该方法被调用。
    // /// @param nTimeLapse 距离上次接收报文的时间
    // pub fn on_heartbeat_warning<F>(&self, mut cb: F) -> &Self
    //     where F: FnMut(Api, i32) -> CTPApiResult
    // {
    //     let api = self.api.clone();
    //     self.spi.lock()?.on_heartbeat_warning(move |time_lapse| {
    //         let api = api.clone();
    //         ctp_error_wrapper!(cb(api, time_lapse));
    //     });
    //     self
    // }

    on_ctp_function!(
        "心跳超时警告。当长时间未收到报文时，该方法被调用。"
        "@param nTimeLapse 距离上次接收报文的时间"
        "注意: 这个方法永远不会被调用到"
        trader->on_heartbeat_warning,
        nTimeLapse: i32,
    );

    on_ctp_function!(
        "错误应答"
        trader->on_rsp_error,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "客户端认证响应"
        trader->on_rsp_authenticate,
        pRspAuthenticateField: &CThostFtdcRspAuthenticateField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "用户口令更新请求响应"
        trader->on_rsp_user_password_update,
        pUserPasswordUpdate: &CThostFtdcUserPasswordUpdateField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "资金账户口令更新请求响应"
        trader->on_rsp_trading_account_password_update,
        pTradingAccountPasswordUpdate: &CThostFtdcTradingAccountPasswordUpdateField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "查询用户当前支持的认证模式的回复"
        trader->on_rsp_user_auth_method,
        pRspUserAuthMethod: &CThostFtdcRspUserAuthMethodField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "获取图形验证码请求的回复"
        trader->on_rsp_gen_user_captcha,
        pRspGenUserCaptcha: &CThostFtdcRspGenUserCaptchaField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "获取短信验证码请求的回复"
        trader->on_rsp_gen_user_text,
        pRspGenUserText: &CThostFtdcRspGenUserTextField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "登录请求响应"
        trader->on_rsp_user_login,
        pRspUserLogin: &CThostFtdcRspUserLoginField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "登出请求响应"
        trader->on_rsp_user_logout,
        pUserLogout: &CThostFtdcUserLogoutField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "投资者结算结果确认响应"
        trader->on_rsp_settlement_info_confirm,
        pSettlementInfoConfirm: &CThostFtdcSettlementInfoConfirmField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询合约响应"
        trader->on_rsp_qry_instrument,
        pInstrument: Option<&CThostFtdcInstrumentField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询行情响应"
        trader->on_rsp_qry_depth_market_data,
        pDepthMarketData: Option<&CThostFtdcDepthMarketDataField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询投资者结算结果响应"
        trader->on_rsp_qry_settlement_info,
        pSettlementInfo: Option<&CThostFtdcSettlementInfoField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询转帐银行响应"
        trader->on_rsp_qry_transfer_bank,
        pTransferBank: Option<&CThostFtdcTransferBankField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询投资者持仓明细响应"
        trader->on_rsp_qry_investor_position_detail,
        pInvestorPositionDetail: Option<&CThostFtdcInvestorPositionDetailField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询客户通知响应"
        trader->on_rsp_qry_notice,
        pNotice: Option<&CThostFtdcNoticeField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询资金账户响应"
        trader->on_rsp_qry_trading_account,
        pTradingAccount: Option<&CThostFtdcTradingAccountField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询投资者响应"
        trader->on_rsp_qry_investor,
        pInvestor: &CThostFtdcInvestorField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询交易编码响应"
        trader->on_rsp_qry_trading_code,
        pTradingCode: &CThostFtdcTradingCodeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询合约保证金率响应"
        trader->on_rsp_qry_instrument_margin_rate,
        pInstrumentMarginRate: Option<&CThostFtdcInstrumentMarginRateField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询合约手续费率响应"
        trader->on_rsp_qry_instrument_commission_rate,
        pInstrumentCommissionRate: Option<&CThostFtdcInstrumentCommissionRateField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询投资者持仓响应"
        trader->on_rsp_qry_investor_position,
        pInvestorPosition: Option<&CThostFtdcInvestorPositionField>,
        pRspInfo: Option<&CThostFtdcRspInfoField>,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "报单录入请求响应"
        trader->on_rsp_order_insert,
        pInputOrder: &CThostFtdcInputOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "报单操作请求响应"
        trader->on_rsp_order_action,
        pInputOrder: &CThostFtdcInputOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "报单通知"
        trader->on_rtn_order,
        pOrder: &CThostFtdcOrderField,
    );

    on_ctp_function!(
        "执行宣告通知"
        trader->on_rtn_exec_order,
        pExecOrder: &CThostFtdcExecOrderField,
    );

    on_ctp_function!(
        "报价通知"
        trader->on_rtn_quote,
        pQuote: &CThostFtdcQuoteField,
    );

    on_ctp_function!(
        "执行宣告录入错误回报"
        trader->on_err_rtn_exec_order_insert,
        pInputExecOrder: &CThostFtdcInputExecOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "执行宣告操作错误回报"
        trader->on_err_rtn_exec_order_action,
        pExecOrderAction: &CThostFtdcExecOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "成交通知"
        trader->on_rtn_trade,
        pTrade: &CThostFtdcTradeField,
    );

    on_ctp_function!(
        "报单录入错误回报"
        trader->on_err_rtn_order_insert,
        pInputOrder: &CThostFtdcInputOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "报单操作错误回报"
        trader->on_err_rtn_order_action,
        pOrderAction: &CThostFtdcOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "合约交易状态通知"
        trader->on_rtn_instrument_status,
        pInstrumentStatus: &CThostFtdcInstrumentStatusField,
    );

    on_ctp_function!(
        "交易所公告通知"
        trader->on_rtn_bulletin,
        pBulletin: &CThostFtdcBulletinField,
    );

    on_ctp_function!(
        "交易通知"
        trader->on_rtn_trading_notice,
        pTradingNoticeInfo: &CThostFtdcTradingNoticeInfoField,
    );

    on_ctp_function!(
        "提示条件单校验错误"
        trader->on_rtn_error_conditional_order,
        pErrorConditionalOrder: &CThostFtdcErrorConditionalOrderField,
    );

    on_ctp_function!(
        "报价录入错误回报"
        trader->on_err_rtn_quote_insert,
        pInputQuote: &CThostFtdcInputQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "报价操作错误回报"
        trader->on_err_rtn_quote_action,
        pQuoteAction: &CThostFtdcQuoteActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "询价录入错误回报"
        trader->on_err_rtn_for_quote_insert,
        pInputForQuote: &CThostFtdcInputForQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "询价通知"
        trader->on_rtn_for_quote_rsp,
        pForQuoteRsp: &CThostFtdcForQuoteRspField,
    );

    on_ctp_function!(
        "保证金监控中心用户令牌"
        trader->on_rtn_cfmmc_trading_account_token,
        pCFMMCTradingAccountToken: &CThostFtdcCFMMCTradingAccountTokenField,
    );

    on_ctp_function!(
        "批量报单操作错误回报"
        trader->on_err_rtn_batch_order_action,
        pBatchOrderAction: &CThostFtdcBatchOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "期权自对冲通知"
        trader->on_rtn_option_self_close,
        pOptionSelfClose: &CThostFtdcOptionSelfCloseField,
    );

    on_ctp_function!(
        "期权自对冲录入错误回报"
        trader->on_err_rtn_option_self_close_insert,
        pInputOptionSelfClose: &CThostFtdcInputOptionSelfCloseField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "期权自对冲操作错误回报"
        trader->on_err_rtn_option_self_close_action,
        pOptionSelfCloseAction: &CThostFtdcOptionSelfCloseActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "申请组合录入错误回报"
        trader->on_err_rtn_comb_action_insert,
        pInputCombAction: &CThostFtdcInputCombActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "请求查询签约银行响应"
        trader->on_rsp_qry_contract_bank,
        pContractBank: &CThostFtdcContractBankField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询预埋单响应"
        trader->on_rsp_qry_parked_order,
        pParkedOrder: &CThostFtdcParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询预埋撤单响应"
        trader->on_rsp_qry_parked_order_action,
        pParkedOrderAction: &CThostFtdcParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询交易通知响应"
        trader->on_rsp_qry_trading_notice,
        pTradingNotice: &CThostFtdcTradingNoticeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询经纪公司交易参数响应"
        trader->on_rsp_qry_broker_trading_params,
        pBrokerTradingParams: &CThostFtdcBrokerTradingParamsField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询经纪公司交易算法响应"
        trader->on_rsp_qry_broker_trading_algos,
        pBrokerTradingParams: &CThostFtdcBrokerTradingAlgosField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询监控中心用户令牌"
        trader->on_rsp_query_cfmmc_trading_account_token,
        pQueryCFMMCTradingAccountToken: &CThostFtdcQueryCFMMCTradingAccountTokenField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "银行发起银行资金转期货通知"
        trader->on_rtn_from_bank_to_future_by_bank,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "银行发起期货资金转银行通知"
        trader->on_rtn_from_future_to_bank_by_bank,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "银行发起冲正银行转期货通知"
        trader->on_rtn_repeal_from_bank_to_future_by_bank,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "银行发起冲正期货转银行通知"
        trader->on_rtn_repeal_from_future_to_bank_by_bank,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "申请组合通知"
        trader->on_rtn_comb_action,
        pCombAction: &CThostFtdcCombActionField,
    );

    on_ctp_function!(
        "预埋单录入请求响应"
        trader->on_rsp_parked_order_insert,
        pParkedOrder: &CThostFtdcParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "预埋撤单录入请求响应"
        trader->on_rsp_parked_order_action,
        pParkedOrder: &CThostFtdcParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "查询最大报单数量响应"
        trader->on_rsp_query_max_order_volume,
        pQueryMaxOrderVolume: &CThostFtdcQueryMaxOrderVolumeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "删除预埋单响应"
        trader->on_rsp_remove_parked_order,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "删除预埋撤单响应"
        trader->on_rsp_remove_parked_order_action,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "执行宣告录入请求响应"
        trader->on_rsp_exec_order_insert,
        pInputExecOrder: &CThostFtdcInputExecOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "执行宣告操作请求响应"
        trader->on_rsp_exec_order_action,
        pInputExecOrderAction: &CThostFtdcInputExecOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "询价录入请求响应"
        trader->on_rsp_for_quote_insert,
        pInputForQuote: &CThostFtdcInputForQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "报价录入请求响应"
        trader->on_rsp_quote_insert,
        pInputQuote: &CThostFtdcInputQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "报价操作请求响应"
        trader->on_rsp_quote_action,
        pInputQuoteAction: &CThostFtdcInputQuoteActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "批量报单操作请求响应"
        trader->on_rsp_batch_order_action,
        pInputBatchOrderAction: &CThostFtdcInputBatchOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "期权自对冲录入请求响应"
        trader->on_rsp_option_self_close_insert,
        pInputOptionSelfClose: &CThostFtdcInputOptionSelfCloseField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "期权自对冲操作请求响应"
        trader->on_rsp_option_self_close_action,
        pInputOptionSelfCloseAction: &CThostFtdcInputOptionSelfCloseActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "申请组合录入请求响应"
        trader->on_rsp_comb_action_insert,
        pInputCombAction: &CThostFtdcInputCombActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询报单响应"
        trader->on_rsp_qry_order,
        pOrder: &CThostFtdcOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询成交响应"
        trader->on_rsp_qry_trade,
        pOrder: &CThostFtdcTradeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询交易所响应"
        trader->on_rsp_qry_exchange,
        pExchange: &CThostFtdcExchangeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "请求查询产品响应"
        trader->on_rsp_qry_product,
        pProduct: &CThostFtdcProductField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "期货发起银行资金转期货通知"
        trader->on_rtn_from_bank_to_future_by_future,
        pRspTransfer: &CThostFtdcRspTransferField,
    );

    on_ctp_function!(
        "期货发起期货资金转银行通知"
        trader->on_rtn_from_future_to_bank_by_future,
        pRspTransfer: &CThostFtdcRspTransferField,
    );

    on_ctp_function!(
        "系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_bank_to_future_by_future_manual,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_future_to_bank_by_future_manual,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "期货发起查询银行余额通知"
        trader->on_rtn_query_bank_balance_by_future,
        pNotifyQueryAccount: &CThostFtdcNotifyQueryAccountField,
    );

    on_ctp_function!(
        "期货发起银行资金转期货错误回报"
        trader->on_err_rtn_bank_to_future_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "期货发起期货资金转银行错误回报"
        trader->on_err_rtn_future_to_bank_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "系统运行时期货端手工发起冲正银行转期货错误回报"
        trader->on_err_rtn_repeal_bank_to_future_by_future_manual,
        pReqRepeal: &CThostFtdcReqRepealField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "系统运行时期货端手工发起冲正期货转银行错误回报"
        trader->on_err_rtn_repeal_future_to_bank_by_future_manual,
        pReqRepeal: &CThostFtdcReqRepealField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "期货发起查询银行余额错误回报"
        trader->on_err_rtn_query_bank_balance_by_future,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    on_ctp_function!(
        "期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_bank_to_future_by_future,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_future_to_bank_by_future,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    on_ctp_function!(
        "期货发起银行资金转期货应答"
        trader->on_rsp_from_bank_to_future_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "期货发起期货资金转银行应答"
        trader->on_rsp_from_future_to_bank_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "期货发起查询银行余额应答"
        trader->on_rsp_query_bank_account_money_by_future,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    on_ctp_function!(
        "银行发起银期开户通知"
        trader->on_rtn_open_account_by_bank,
        pOpenAccount: &CThostFtdcOpenAccountField,
    );

    on_ctp_function!(
        "银行发起银期销户通知"
        trader->on_rtn_cancel_account_by_bank,
        pCancelAccount: &CThostFtdcCancelAccountField,
    );

    on_ctp_function!(
        "银行发起变更银行账号通知"
        trader->on_rtn_change_account_by_bank,
        pChangeAccount: &CThostFtdcChangeAccountField,
    );
}

#[cfg(test)]
mod tests {
    use crate::api::opts::*;
    use crate::api::*;
    use crate::*;
}
