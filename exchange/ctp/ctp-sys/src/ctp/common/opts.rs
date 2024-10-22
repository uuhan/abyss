// linux x86_64 下 bidgen 生成的 char 为 i8
use crate::ctp::api::*;
use displaydoc::Display as DocDisplay;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderPriceType {
    /// 任意价(即: 市价单, 上期所,SHFE不支持)
    AnyPrice = THOST_FTDC_OPT_AnyPrice,
    /// 限价
    LimitPrice = THOST_FTDC_OPT_LimitPrice,
    /// 最优价(不支持)
    BestPrice = THOST_FTDC_OPT_BestPrice,
    /// 最新价
    LastPrice = THOST_FTDC_OPT_LastPrice,
    /// 最新价浮动上浮1个ticks
    LastPricePlusOneTicks = THOST_FTDC_OPT_LastPricePlusOneTicks,
    /// 最新价浮动上浮2个ticks
    LastPricePlusTwoTicks = THOST_FTDC_OPT_LastPricePlusTwoTicks,
    /// 最新价浮动上浮3个ticks
    LastPricePlusThreeTicks = THOST_FTDC_OPT_LastPricePlusThreeTicks,
    /// 卖一价
    AskPrice1 = THOST_FTDC_OPT_AskPrice1,
    /// 卖一价浮动上浮1个ticks
    AskPrice1PlusOneTicks = THOST_FTDC_OPT_AskPrice1PlusOneTicks,
    /// 卖一价浮动上浮2个ticks
    AskPrice1PlusTwoTicks = THOST_FTDC_OPT_AskPrice1PlusTwoTicks,
    /// 卖一价浮动上浮3个ticks
    AskPrice1PlusThreeTicks = THOST_FTDC_OPT_AskPrice1PlusThreeTicks,
    /// 买一价
    BidPrice1 = THOST_FTDC_OPT_BidPrice1,
    /// 买一价浮动上浮1个ticks
    BidPrice1PlusOneTicks = THOST_FTDC_OPT_BidPrice1PlusOneTicks,
    /// 买一价浮动上浮2个ticks
    BidPrice1PlusTwoTicks = THOST_FTDC_OPT_BidPrice1PlusTwoTicks,
    /// 买一价浮动上浮3个ticks
    BidPrice1PlusThreeTicks = THOST_FTDC_OPT_BidPrice1PlusThreeTicks,
    /// 五档价
    FiveLevelPrice = THOST_FTDC_OPT_FiveLevelPrice,
}
impl Default for OrderPriceType {
    fn default() -> Self {
        OrderPriceType::LimitPrice
    }
}

/// 订单价格类型
/// 注意: OnRtnTrade 中返回的价格类型为空
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderPriceSourceType {
    /// 未知
    Unknown = 0,
    /// 前成交价
    LastPrice = THOST_FTDC_PSRC_LastPrice,
    /// 买委托价
    Buy = THOST_FTDC_PSRC_Buy,
    /// 卖委托价
    Sell = THOST_FTDC_PSRC_Sell,
    /// 场外成交价
    OTC = THOST_FTDC_PSRC_OTC,
}
impl Default for OrderPriceSourceType {
    fn default() -> Self {
        OrderPriceSourceType::LastPrice
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderDirection {
    /// 买单
    Buy = THOST_FTDC_D_Buy,
    /// 卖单
    Sell = THOST_FTDC_D_Sell,
}
impl Default for OrderDirection {
    fn default() -> Self {
        OrderDirection::Buy
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PositionDirection {
    /// 一
    Net = THOST_FTDC_PD_Net,
    /// 多
    Long = THOST_FTDC_PD_Long,
    /// 空
    Short = THOST_FTDC_PD_Short,
}
impl Default for PositionDirection {
    fn default() -> Self {
        PositionDirection::Net
    }
}
impl FromStr for PositionDirection {
    type Err = std::char::ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arg = s.parse::<char>()? as u8;
        match arg {
            THOST_FTDC_PD_Long => Ok(PositionDirection::Long),
            THOST_FTDC_PD_Short => Ok(PositionDirection::Short),
            _ => Ok(PositionDirection::Net),
        }
    }
}

/// 基本交易所只支持 Immediately(IOC), ThisDay(GFD)
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderTimeCondition {
    /// 立即完成，否则撤销
    Immediately = THOST_FTDC_TC_IOC,
    /// 本节有效
    Section = THOST_FTDC_TC_GFS,
    /// 当日有效
    ThisDay = THOST_FTDC_TC_GFD,
    /// 指定日期前有效
    BeforeDate = THOST_FTDC_TC_GTD,
    /// 撤销前有效
    Always = THOST_FTDC_TC_GTC,
    /// 集合竞价有效
    Aggregate = THOST_FTDC_TC_GFA,
}
impl Default for OrderTimeCondition {
    fn default() -> Self {
        OrderTimeCondition::Immediately
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderVolumeCondition {
    /// 任何数量
    Any = THOST_FTDC_VC_AV,
    /// 最小数量
    Min = THOST_FTDC_VC_MV,
    /// 全部数量
    All = THOST_FTDC_VC_CV,
}
impl Default for OrderVolumeCondition {
    fn default() -> Self {
        OrderVolumeCondition::Any
    }
}

#[derive(DocDisplay, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[repr(u8)]
pub enum OrderOffsetFlag {
    /// 开仓
    Open = THOST_FTDC_OF_Open,
    /// 平仓
    Close = THOST_FTDC_OF_Close,
    /// 强平
    ForceClose = THOST_FTDC_OF_ForceClose,
    // 上期所, 能源交易所
    #[doc = "平今"]
    CloseToday = THOST_FTDC_OF_CloseToday,
    // 上期所, 能源交易所
    #[doc = "平昨"]
    CloseYesterday = THOST_FTDC_OF_CloseYesterday,
    /// 强减
    ForceOff = THOST_FTDC_OF_ForceOff,
    /// 本地强平
    LocalForceClose = THOST_FTDC_OF_LocalForceClose,
}

impl Default for OrderOffsetFlag {
    fn default() -> Self {
        OrderOffsetFlag::Open
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderHedgeFlag {
    /// 投机
    Speculation = THOST_FTDC_HF_Speculation,
    /// 套利
    Arbitrage = THOST_FTDC_HF_Arbitrage,
    /// 套保
    Hedge = THOST_FTDC_HF_Hedge,
    /// 做市商
    MarketMaker = THOST_FTDC_HF_MarketMaker,
    #[cfg(not(feature = "v6_3_13"))]
    /// 第一腿投机第二腿套保 大商所专用
    SpecHedge = THOST_FTDC_HF_SpecHedge,
    #[cfg(not(feature = "v6_3_13"))]
    /// 第一腿套保第二腿投机  大商所专用
    HedgeSpec = THOST_FTDC_HF_HedgeSpec,
}
impl Default for OrderHedgeFlag {
    fn default() -> Self {
        OrderHedgeFlag::Speculation
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderContingentCondition {
    /// 立即
    Immediately = THOST_FTDC_CC_Immediately,
    /// 止损
    Touch = THOST_FTDC_CC_Touch,
    /// 止赢
    TouchProfit = THOST_FTDC_CC_TouchProfit,
    /// 预埋单
    ParkedOrder = THOST_FTDC_CC_ParkedOrder,
    /// 最新价大于条件价
    LastPriceGreaterThanStopPrice = THOST_FTDC_CC_LastPriceGreaterThanStopPrice,
    /// 最新价大于等于条件价
    LastPriceGreaterEqualStopPrice = THOST_FTDC_CC_LastPriceGreaterEqualStopPrice,
    /// 最新价小于条件价
    LastPriceLesserThanStopPrice = THOST_FTDC_CC_LastPriceLesserThanStopPrice,
    /// 最新价小于等于条件价
    LastPriceLesserEqualStopPrice = THOST_FTDC_CC_LastPriceLesserEqualStopPrice,
    /// 卖一价大于条件价
    AskPriceGreaterThanStopPrice = THOST_FTDC_CC_AskPriceGreaterThanStopPrice,
    /// 卖一价大于等于条件价
    AskPriceGreaterEqualStopPrice = THOST_FTDC_CC_AskPriceGreaterEqualStopPrice,
    /// 卖一价小于条件价
    AskPriceLesserThanStopPrice = THOST_FTDC_CC_AskPriceLesserThanStopPrice,
    /// 卖一价小于等于条件价
    AskPriceLesserEqualStopPrice = THOST_FTDC_CC_AskPriceLesserEqualStopPrice,
    /// 买一价大于条件价
    BidPriceGreaterThanStopPrice = THOST_FTDC_CC_BidPriceGreaterThanStopPrice,
    /// 买一价大于等于条件价
    BidPriceGreaterEqualStopPrice = THOST_FTDC_CC_BidPriceGreaterEqualStopPrice,
    /// 买一价小于条件价
    BidPriceLesserThanStopPrice = THOST_FTDC_CC_BidPriceLesserThanStopPrice,
    /// 买一价小于等于条件价
    BidPriceLesserEqualStopPrice = THOST_FTDC_CC_BidPriceLesserEqualStopPrice,
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderForceCloseReason {
    /// 非强平
    NotForceClose = THOST_FTDC_FCC_NotForceClose,
    /// 资金不足
    LackDeposit = THOST_FTDC_FCC_LackDeposit,
    /// 客户超仓
    ClientOverPositionLimit = THOST_FTDC_FCC_ClientOverPositionLimit,
    /// 会员超仓
    MemberOverPositionLimit = THOST_FTDC_FCC_MemberOverPositionLimit,
    /// 持仓非整数倍
    NotMultiple = THOST_FTDC_FCC_NotMultiple,
    /// 违规
    Violation = THOST_FTDC_FCC_Violation,
    /// 其他
    Other = THOST_FTDC_FCC_Other,
    /// 自然人临近交割
    PersonDeliv = THOST_FTDC_FCC_PersonDeliv,
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderActionFlag {
    /// 删除订单操作
    Delete = THOST_FTDC_AF_Delete,
    /// 修改订单操作
    Modify = THOST_FTDC_AF_Modify,
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderSubmitStatus {
    /// 报单已经提交
    InsertSubmitted = THOST_FTDC_OSS_InsertSubmitted,
    /// 撤单已经提交
    CancelSubmitted = THOST_FTDC_OSS_CancelSubmitted,
    /// 修改已经提交
    ModifySubmitted = THOST_FTDC_OSS_ModifySubmitted,
    /// 已经接受
    Accepted = THOST_FTDC_OSS_Accepted,
    /// 报单已经被拒绝
    InsertRejected = THOST_FTDC_OSS_InsertRejected,
    /// 撤单已经被拒绝
    CancelRejected = THOST_FTDC_OSS_CancelRejected,
    /// 改单已经被拒绝
    ModifyRejected = THOST_FTDC_OSS_ModifyRejected,
}
impl Default for OrderSubmitStatus {
    fn default() -> Self {
        OrderSubmitStatus::InsertSubmitted
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum OrderStatus {
    /// 全部成交
    AllTraded = THOST_FTDC_OST_AllTraded,
    // 报单部分手数还未成交
    #[doc = "部分成交还在队列中"]
    PartTradedQueueing = THOST_FTDC_OST_PartTradedQueueing,
    // 这个字段, 在报单的时候设置了 "自动挂起" 即 IsAutoSuspend 为 true
    // 此时用户断线了, 但是部分单子已经成交, 报单的状态就是: 部分成交不在队列中
    // 这种状态不建议使用
    #[doc = "部分成交不在队列中"]
    PartTradedNotQueueing = THOST_FTDC_OST_PartTradedNotQueueing,
    // 报单所有手数都未成交
    #[doc = "未成交还在队列中"]
    NoTradeQueueing = THOST_FTDC_OST_NoTradeQueueing,
    // 这个字段, 在报单的时候设置了 "自动挂起" 即 IsAutoSuspend 为 true
    // 此时用户断线了, 报单的状态就是: 未成交不在队列中
    // 这种状态不建议使用
    #[doc = "未成交不在队列中"]
    NoTradeNotQueueing = THOST_FTDC_OST_NoTradeNotQueueing,
    /// 撤单
    Canceled = THOST_FTDC_OST_Canceled,
    /// 未知
    Unknown = THOST_FTDC_OST_Unknown,
    /// 尚未触发
    NotTouched = THOST_FTDC_OST_NotTouched,
    /// 已触发
    Touched = THOST_FTDC_OST_Touched,
}
impl Default for OrderStatus {
    fn default() -> Self {
        OrderStatus::AllTraded
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum ExchangeID {
    /// 上金所
    CFFEX,
    /// 上期所
    SHFE,
    /// 能源所
    INE,
    /// 大商所
    DCE,
    /// 郑商所
    CZCE,
    /// 未知所
    OTHER,
}

impl ExchangeID {
    pub fn from(id: &TThostFtdcExchangeIDType) -> Self {
        match id {
            [67, 70, 70, 69, 88, 0, 0, 0, 0] => Self::CFFEX,
            [67, 90, 67, 69, 0, 0, 0, 0, 0] => Self::CZCE,
            [68, 67, 69, 0, 0, 0, 0, 0, 0] => Self::DCE,
            [73, 78, 69, 0, 0, 0, 0, 0, 0] => Self::INE,
            [83, 72, 70, 69, 0, 0, 0, 0, 0] => Self::SHFE,
            _ => Self::OTHER,
        }
    }

    pub fn id(&self) -> TThostFtdcExchangeIDType {
        match self {
            Self::CFFEX => [67, 70, 70, 69, 88, 0, 0, 0, 0],
            Self::CZCE => [67, 90, 67, 69, 0, 0, 0, 0, 0],
            Self::DCE => [68, 67, 69, 0, 0, 0, 0, 0, 0],
            Self::INE => [73, 78, 69, 0, 0, 0, 0, 0, 0],
            Self::SHFE => [83, 72, 70, 69, 0, 0, 0, 0, 0],
            Self::OTHER => Default::default(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::CFFEX => "上金所",
            Self::CZCE => "郑商所",
            Self::DCE => "大商所",
            Self::INE => "能源所",
            Self::SHFE => "上期所",
            Self::OTHER => "其他所",
        }
    }
}

impl Into<String> for ExchangeID {
    fn into(self) -> String {
        match self {
            Self::CFFEX => "CFFEX",
            Self::CZCE => "CZCE",
            Self::DCE => "DCE",
            Self::INE => "INE",
            Self::SHFE => "SHFE",
            Self::OTHER => "OTHER",
        }
        .to_owned()
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PositionDate {
    /// 今日持仓
    Today = THOST_FTDC_PSD_Today, // 除了上期所,能源所之外,都是今日持仓
    /// 历史持仓
    History = THOST_FTDC_PSD_History, // 上期、能源区分昨仓今仓
}
impl Default for PositionDate {
    fn default() -> Self {
        PositionDate::Today
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TradeType {
    ///组合持仓拆分为单一持仓,初始化不应包含该类型的持仓
    SplitCombination = THOST_FTDC_TRDT_SplitCombination,
    ///普通成交
    Common = THOST_FTDC_TRDT_Common,
    ///期权执行
    OptionsExecution = THOST_FTDC_TRDT_OptionsExecution,
    ///OTC成交
    OTC = THOST_FTDC_TRDT_OTC,
    ///期转现衍生成交
    EFPDerived = THOST_FTDC_TRDT_EFPDerived,
    ///组合衍生成交
    CombinationDerived = THOST_FTDC_TRDT_CombinationDerived,
    #[cfg(not(feature = "v6_3_13"))]
    ///大宗交易成交
    BlockTrade = THOST_FTDC_TRDT_BlockTrade,
}

#[derive(DocDisplay, Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
#[repr(u32)]
pub enum TopicTertFlag {
    /// 从本交易日开始重传
    Restart = THOST_TE_RESUME_TYPE_THOST_TERT_RESTART,
    /// 从上次收到的续传
    Resume = THOST_TE_RESUME_TYPE_THOST_TERT_RESUME,
    /// 只传送登录后私有流的内容
    Quick = THOST_TE_RESUME_TYPE_THOST_TERT_QUICK,
}

/// 合约交易状态
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum InstrumentStatus {
    /// 开盘前
    BeforeTrading = THOST_FTDC_IS_BeforeTrading,
    /// 非交易
    NoTrading = THOST_FTDC_IS_NoTrading,
    /// 连续交易
    Continous = THOST_FTDC_IS_Continous,
    /// 集合竞价报单
    AuctionOrdering = THOST_FTDC_IS_AuctionOrdering,
    /// 集合竞价价格平衡
    AuctionBalance = THOST_FTDC_IS_AuctionBalance,
    /// 集合竞价撮合
    AuctionMatch = THOST_FTDC_IS_AuctionMatch,
    /// 收盘
    Closed = THOST_FTDC_IS_Closed,
}
impl Default for InstrumentStatus {
    fn default() -> Self {
        InstrumentStatus::Continous
    }
}

/// 进入本状态原因
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum InstrumentStatusReason {
    /// 自动切换
    Automatic = THOST_FTDC_IER_Automatic,
    /// 手动切换
    Manual = THOST_FTDC_IER_Manual,
    /// 熔断
    Fuse = THOST_FTDC_IER_Fuse,
}

/// 报单成交状态
#[derive(DocDisplay)]
pub enum OrderDealType {
    /// 多开
    OpenLong,
    /// 空开
    OpenShort,
    /// 双开
    OpenDouble,
    /// 多平
    CloseLong,
    /// 空平
    CloseShort,
    /// 多换
    ExchangeLong,
    /// 空换
    ExchangeShort,
}

/// 持仓类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PositionType {
    /// 净持仓
    Net = THOST_FTDC_PT_Net,
    /// 综合持仓
    Gross = THOST_FTDC_PT_Gross,
}
impl Default for PositionType {
    fn default() -> Self {
        PositionType::Net
    }
}

/// 持仓日期类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PositionDateType {
    /// 使用历史持仓
    UseHistory = THOST_FTDC_PDT_UseHistory,
    /// 不使用历史持仓
    NoUseHistory = THOST_FTDC_PDT_NoUseHistory,
}
impl Default for PositionDateType {
    fn default() -> Self {
        PositionDateType::UseHistory
    }
}

/// 合约生命周期
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum InstLifePhase {
    ///未上市
    NotStart = THOST_FTDC_IP_NotStart,
    ///上市
    Started = THOST_FTDC_IP_Started,
    ///停牌
    Pause = THOST_FTDC_IP_Pause,
    ///到期
    Expired = THOST_FTDC_IP_Expired,
}
impl Default for InstLifePhase {
    fn default() -> Self {
        InstLifePhase::Started
    }
}

/// 产品类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum ProductClass {
    ///期货
    Futures = THOST_FTDC_PC_Futures,
    ///期货期权
    Options = THOST_FTDC_PC_Options,
    ///组合
    Combination = THOST_FTDC_PC_Combination,
    ///即期
    Spot = THOST_FTDC_PC_Spot,
    ///期转现
    EFP = THOST_FTDC_PC_EFP,
    ///现货期权
    SpotOption = THOST_FTDC_PC_SpotOption,
    ///TAS合约
    TAS = THOST_FTDC_PC_TAS,
    ///金属指数
    MI = THOST_FTDC_PC_MI,
}
impl Default for ProductClass {
    fn default() -> Self {
        ProductClass::Futures
    }
}

/// 布尔类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum BoolType {
    /// 是
    True = 1u32,
    /// 否
    False = 0u32,
}
impl Default for BoolType {
    fn default() -> Self {
        BoolType::False
    }
}
impl BoolType {
    #[inline]
    pub fn from(b: bool) -> Self {
        match b {
            true => BoolType::True,
            false => BoolType::False,
        }
    }
}

/// 是否使用大额单边保证金算法
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum MaxMarginSideAlgorithmType {
    /// 不使用
    NO = THOST_FTDC_MMSA_NO,
    /// 使用
    YES = THOST_FTDC_MMSA_YES,
}

/// 期权类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OptionsType {
    /// 非期权
    NotOptions = 0,
    /// 看涨
    CallOptions = THOST_FTDC_CP_CallOptions,
    /// 看跌
    PutOptions = THOST_FTDC_CP_PutOptions,
}

/// 组合类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum CombinationType {
    /// 期货组合
    Future = THOST_FTDC_COMBT_Future,
    /// 垂直价差BUL
    BUL = THOST_FTDC_COMBT_BUL,
    /// 垂直价差BER
    BER = THOST_FTDC_COMBT_BER,
    /// 跨式组合
    STD = THOST_FTDC_COMBT_STD,
    /// 宽跨式组合
    STG = THOST_FTDC_COMBT_STG,
    /// 备兑组合
    PRT = THOST_FTDC_COMBT_PRT,
    /// 时间价差组合
    CLD = THOST_FTDC_COMBT_CLD,
    /// 期权对锁组合
    OPL = THOST_FTDC_COMBT_OPL,
    /// 买备兑组合
    BFO = THOST_FTDC_COMBT_BFO,
    /// 未知
    Unknown = 0,
}

/// 投资者范围类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum InvestorRangeType {
    /// 所有
    All = THOST_FTDC_IR_All,
    /// 投资者组
    Group = THOST_FTDC_IR_Group,
    /// 单一投资者
    Single = THOST_FTDC_IR_Single,
}
impl Default for InvestorRangeType {
    fn default() -> Self {
        InvestorRangeType::All
    }
}

/// 业务类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BizType {
    /// 期货
    Future = THOST_FTDC_BZTP_Future,
    /// 证券
    Stock = THOST_FTDC_BZTP_Stock,
    /// 未知
    Unknown = 0,
}
impl Default for BizType {
    fn default() -> Self {
        BizType::Future
    }
}

/// 订单成交类型
/// 注意: OnRtnTrade 中返回的成交类型为空
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderTradeType {
    /// 未知
    Unknown = 0,
    /// 组合持仓拆分为单一持仓,初始化不应包含该类型的持仓
    SplitCombination = THOST_FTDC_TRDT_SplitCombination,
    /// 普通成交
    Common = THOST_FTDC_TRDT_Common,
    /// 期权执行
    OptionsExecution = THOST_FTDC_TRDT_OptionsExecution,
    /// OTC成交
    OTC = THOST_FTDC_TRDT_OTC,
    /// 期转现衍生成交
    EFPDerived = THOST_FTDC_TRDT_EFPDerived,
    /// 组合衍生成交
    CombinationDerived = THOST_FTDC_TRDT_CombinationDerived,
    /// 大宗交易成交
    BlockTrade = THOST_FTDC_TRDT_BlockTrade,
}
impl Default for OrderTradeType {
    fn default() -> Self {
        OrderTradeType::Common
    }
}

/// 订单成交来源类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum OrderTradeSourceType {
    ///来自交易所普通回报
    NORMAL = THOST_FTDC_TSRC_NORMAL,
    ///来自查询
    QUERY = THOST_FTDC_TSRC_QUERY,
}
impl Default for OrderTradeSourceType {
    fn default() -> Self {
        OrderTradeSourceType::NORMAL
    }
}

/// 交易角色类型
#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum TradingRoleType {
    /// 未知
    Unknown = 0,
    ///代理
    Broker = THOST_FTDC_ER_Broker,
    ///自营
    Host = THOST_FTDC_ER_Host,
    ///做市商
    Maker = THOST_FTDC_ER_Maker,
}
impl Default for TradingRoleType {
    fn default() -> Self {
        TradingRoleType::Broker
    }
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
/// 交易所属性类型
pub enum ExchangeProperty {
    /// 正常
    Normal = THOST_FTDC_EXP_Normal,
    /// 根据成交生成报单
    GeoOrderByTrade = THOST_FTDC_EXP_GenOrderByTrade,
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
/// 证件类型
pub enum IdCardType {
    ///组织机构代码
    EID = THOST_FTDC_ICT_EID,
    ///中国公民身份证
    IDCard = THOST_FTDC_ICT_IDCard,
    ///军官证
    OfficerIDCard = THOST_FTDC_ICT_OfficerIDCard,
    ///警官证
    PoliceIDCard = THOST_FTDC_ICT_PoliceIDCard,
    ///士兵证
    SoldierIDCard = THOST_FTDC_ICT_SoldierIDCard,
    ///户口簿
    HouseholdRegister = THOST_FTDC_ICT_HouseholdRegister,
    ///护照
    Passport = THOST_FTDC_ICT_Passport,
    ///台胞证
    TaiwanCompatriotIDCard = THOST_FTDC_ICT_TaiwanCompatriotIDCard,
    ///回乡证
    HomeComingCard = THOST_FTDC_ICT_HomeComingCard,
    ///营业执照号
    LicenseNo = THOST_FTDC_ICT_LicenseNo,
    ///税务登记号/当地纳税ID
    TaxNo = THOST_FTDC_ICT_TaxNo,
    ///港澳居民来往内地通行证
    HMMainlandTravelPermit = THOST_FTDC_ICT_HMMainlandTravelPermit,
    ///台湾居民来往大陆通行证
    TwMainlandTravelPermit = THOST_FTDC_ICT_TwMainlandTravelPermit,
    ///驾照
    DrivingLicense = THOST_FTDC_ICT_DrivingLicense,
    ///当地社保ID
    SocialID = THOST_FTDC_ICT_SocialID,
    ///当地身份证
    LocalID = THOST_FTDC_ICT_LocalID,
    ///商业登记证
    BusinessRegistration = THOST_FTDC_ICT_BusinessRegistration,
    ///港澳永久性居民身份证
    HKMCIDCard = THOST_FTDC_ICT_HKMCIDCard,
    ///人行开户许可证
    AccountsPermits = THOST_FTDC_ICT_AccountsPermits,
    ///外国人永久居留证
    FrgPrmtRdCard = THOST_FTDC_ICT_FrgPrmtRdCard,
    ///资管产品备案函
    CptMngPrdLetter = THOST_FTDC_ICT_CptMngPrdLetter,
    ///统一社会信用代码
    UniformSocialCreditCode = THOST_FTDC_ICT_UniformSocialCreditCode,
    ///机构成立证明文件
    CorporationCertNo = THOST_FTDC_ICT_CorporationCertNo,
    ///其他证件
    OtherCard = THOST_FTDC_ICT_OtherCard,
}

#[derive(DocDisplay, Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
/// 转账交易状态类型
pub enum TransferStatusType {
    /// 正常
    Normal = THOST_FTDC_TRFS_Normal,
    /// 被冲正
    Repealed = THOST_FTDC_TRFS_Repealed,
}
