use abyss_macros::default;
use abyss_macros::ftdc_field_builder;
use abyss_macros::ftdc_field_string;
use std::env;
use std::mem::size_of;

use crate::ctp::api::*;
use crate::ctp::error::CTPResult;
use crate::ctp::opts::*;

#[ftdc_field_builder]
pub struct CThostFtdcReqUserLoginField {
    // #[doc = "交易日"]
    // pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    pub Password: TThostFtdcPasswordType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "动态密码"]
    pub OneTimePassword: TThostFtdcPasswordType,
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
    #[doc = "登录备注"]
    pub LoginRemark: TThostFtdcLoginRemarkType,
}

#[doc = "用户登出请求"]
#[ftdc_field_builder]
pub struct CThostFtdcUserLogoutField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}

#[doc = "客户端认证请求"]
#[ftdc_field_builder]
pub struct CThostFtdcReqAuthenticateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "认证码"]
    pub AuthCode: TThostFtdcAuthCodeType,
    #[doc = "App代码"]
    pub AppID: TThostFtdcAppIDType,
}

#[ftdc_field_builder]
pub struct CThostFtdcQryInstrumentField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
}

#[ftdc_field_builder]
pub struct CThostFtdcQrySettlementInfoConfirmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}

#[ftdc_field_builder]
pub struct CThostFtdcSettlementInfoConfirmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    // #[doc = "确认日期"]
    // pub ConfirmDate: TThostFtdcDateType,
    // #[doc = "确认时间"]
    // pub ConfirmTime: TThostFtdcTimeType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    // #[doc = "币种代码"]
    // pub CurrencyID: TThostFtdcCurrencyIDType,
}

#[ftdc_field_builder]
struct CThostFtdcQryTradingAccountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
}

#[ftdc_field_builder]
pub struct CThostFtdcQryTradeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "成交编号"]
    pub TradeID: TThostFtdcTradeIDType,
    #[doc = "开始时间"]
    pub TradeTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub TradeTimeEnd: TThostFtdcTimeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}

#[doc = "输入报单"]
#[ftdc_field_string]
pub struct CThostFtdcInputOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    // #[doc = "GTD日期"]
    // pub GTDDate: TThostFtdcDateType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "交易编码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}

impl CThostFtdcInputOrderField {
    pub fn with_order_price_type(&mut self, opt: OrderPriceType) -> &mut Self {
        self.OrderPriceType = opt;
        self
    }

    pub fn with_direction(&mut self, direction: OrderDirection) -> &mut Self {
        self.Direction = direction;
        self
    }

    pub fn with_limit_price<Price: Into<f64>>(&mut self, price: Price) -> &mut Self {
        self.LimitPrice = price.into();
        self
    }

    pub fn with_time_condition(&mut self, condition: OrderTimeCondition) -> &mut Self {
        self.TimeCondition = condition;
        self
    }

    pub fn with_volume_condition(&mut self, condition: OrderVolumeCondition) -> &mut Self {
        self.VolumeCondition = condition;
        self
    }

    pub fn with_comb_offset_flag(&mut self, flag: OrderOffsetFlag) -> &mut Self {
        self.CombOffsetFlag = [flag as u8, 0, 0, 0, 0];
        self
    }

    pub fn with_comb_hedge_flag(&mut self, flag: OrderHedgeFlag) -> &mut Self {
        self.CombHedgeFlag = [flag as u8, 0, 0, 0, 0];
        self
    }

    pub fn with_min_volume<Volume: Into<i32>>(&mut self, volume: Volume) -> &mut Self {
        self.MinVolume = volume.into();
        self
    }

    pub fn with_total_volume<Volume: Into<i32>>(&mut self, volume: Volume) -> &mut Self {
        self.VolumeTotalOriginal = volume.into();
        self
    }

    pub fn with_contingent_condition(&mut self, condition: OrderContingentCondition) -> &mut Self {
        self.ContingentCondition = condition as u8;
        self
    }

    pub fn with_force_close_reason(&mut self, reason: OrderForceCloseReason) -> &mut Self {
        self.ForceCloseReason = reason as u8;
        self
    }

    pub fn with_auto_suspend(&mut self, suspend: bool) -> &mut Self {
        self.IsAutoSuspend = BoolType::from(suspend);
        self
    }

    pub fn with_user_force_close(&mut self, close: bool) -> &mut Self {
        self.UserForceClose = BoolType::from(close);
        self
    }

    pub fn with_order_ref(&mut self, order_ref: TThostFtdcOrderRefType) -> &mut Self {
        self.OrderRef = order_ref;
        self
    }

    /// 平今, 上期所, 能源所区分今仓、昨仓
    pub fn today_position(&mut self) -> &mut Self {
        self.CombOffsetFlag = [OrderOffsetFlag::CloseToday as u8, 0, 0, 0, 0];
        self
    }

    /// 平昨, 上期所, 能源所区分今仓、昨仓
    pub fn yesterday_position(&mut self) -> &mut Self {
        self.CombOffsetFlag = [OrderOffsetFlag::CloseYesterday as u8, 0, 0, 0, 0];
        self
    }

    /// 强平
    pub fn force_position(&mut self) -> &mut Self {
        self.CombOffsetFlag = [OrderOffsetFlag::ForceClose as u8, 0, 0, 0, 0];
        self
    }

    /// 卖开仓
    pub fn open_short<Instrument: AsRef<str>>(instrument: Instrument, volume: i32) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_direction(OrderDirection::Sell)
            .with_time_condition(OrderTimeCondition::ThisDay)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// 卖平仓
    pub fn close_short<Instrument: AsRef<str>>(instrument: Instrument, volume: i32) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_direction(OrderDirection::Buy)
            .with_time_condition(OrderTimeCondition::ThisDay)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// 买开仓
    pub fn open_long<Instrument: AsRef<str>>(instrument: Instrument, volume: i32) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_direction(OrderDirection::Buy)
            .with_time_condition(OrderTimeCondition::ThisDay)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// 买平仓
    pub fn close_long<Instrument: AsRef<str>>(instrument: Instrument, volume: i32) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_direction(OrderDirection::Sell)
            .with_time_condition(OrderTimeCondition::ThisDay)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 买开仓
    pub fn open_long_fak<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 卖开仓
    pub fn open_short_fak<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 买平仓
    pub fn close_long_fak<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 卖平仓
    pub fn close_short_fak<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 买开仓(最小成交)
    pub fn open_long_fak_min<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
        min: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(min)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Min)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 卖开仓(最小成交)
    pub fn open_short_fak_min<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
        min: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(min)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Min)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 买平仓(最小成交)
    pub fn close_long_fak_min<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
        min: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(min)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Min)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FAK 卖平仓(最小成交)
    pub fn close_short_fak_min<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
        min: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_min_volume(min)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Min)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FOK 买开仓
    pub fn open_long_fok<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FOK 卖开仓
    pub fn open_short_fok<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FOK 买平仓
    pub fn close_long_fok<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// FOK 卖平仓
    pub fn close_short_fok<Instrument: AsRef<str>, Volume: Into<i32>>(
        instrument: Instrument,
        volume: Volume,
    ) -> Self {
        CThostFtdcInputOrderField::new()
            .with_instrument_id(instrument)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    pub fn build(&self) -> Self {
        CThostFtdcInputOrderField {
            BrokerID: self.BrokerID,
            InvestorID: self.InvestorID,
            InstrumentID: self.InstrumentID,
            UserID: self.UserID,
            GTDDate: self.GTDDate,
            BusinessUnit: self.BusinessUnit,
            ExchangeID: self.ExchangeID,
            InvestUnitID: self.InvestUnitID,
            AccountID: self.AccountID,
            CurrencyID: self.CurrencyID,
            ClientID: self.ClientID,
            IPAddress: self.IPAddress,
            MacAddress: self.MacAddress,

            OrderPriceType: self.OrderPriceType,
            Direction: self.Direction,
            LimitPrice: self.LimitPrice,
            TimeCondition: self.TimeCondition,
            VolumeCondition: self.VolumeCondition,
            CombOffsetFlag: self.CombOffsetFlag,
            CombHedgeFlag: self.CombHedgeFlag,
            MinVolume: self.MinVolume,
            VolumeTotalOriginal: self.VolumeTotalOriginal,
            ContingentCondition: self.ContingentCondition,
            ForceCloseReason: self.ForceCloseReason,
            IsAutoSuspend: self.IsAutoSuspend,
            UserForceClose: self.UserForceClose,
            OrderRef: self.OrderRef,

            ..Default::default()
        }
    }
}

#[doc = "输入报单操作"]
#[ftdc_field_string]
pub struct CThostFtdcInputOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    // #[doc = "投资者代码"]
    // pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    // #[doc = "报单编号"]
    // pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}

// ExchangeID + OrderSysID
// FrontID + SessionID + OrderRef + InstrumentID
impl CThostFtdcInputOrderActionField {
    pub fn with_front_id(&mut self, front_id: i32) -> &mut Self {
        self.FrontID = front_id;
        self
    }

    // 这里做了特殊处理, 因为一般情况下 UserID = InvestorID, 设置 InvestorID 的同时就把 UserID 也设置了
    // 如果有空置 UserID 的需求, 可以 .with_user_id("")
    pub fn with_investor_id<ID: AsRef<str>>(&mut self, investor_id: ID) -> &mut Self {
        self.with_user_id(investor_id);
        let l = self.InvestorID.len();
        for (idx, v) in self.UserID.iter().enumerate() {
            if idx < l {
                self.InvestorID[idx] = *v as u8;
            }
        }
        self
    }

    pub fn with_session_id(&mut self, session_id: i32) -> &mut Self {
        self.SessionID = session_id;
        self
    }

    // OrderSysID是CTP返回用于标示报单唯一的字段, 没有字符串属性, 没必要放到 ftdc_field_string 中.
    pub fn with_order_sys_id(&mut self, sys_id: TThostFtdcOrderSysIDType) -> &mut Self {
        self.OrderSysID = sys_id;
        self
    }

    pub fn with_action_flag(&mut self, flag: OrderActionFlag) -> &mut Self {
        self.ActionFlag = flag as u8;
        self
    }

    pub fn build(&self) -> Self {
        CThostFtdcInputOrderActionField {
            BrokerID: self.BrokerID,
            InvestorID: self.InvestorID,
            ExchangeID: self.ExchangeID,
            OrderSysID: self.OrderSysID,
            // 用于接收 OnErrRtnOrderAction 消息
            // 默认同 InvestorID
            UserID: self.UserID,
            InstrumentID: self.InstrumentID,
            InvestUnitID: self.InvestUnitID,
            IPAddress: self.IPAddress,
            MacAddress: self.MacAddress,

            FrontID: self.FrontID,
            SessionID: self.SessionID,
            ActionFlag: self.ActionFlag,

            ..Default::default()
        }
    }
}

// 报单返回
impl AsMut<CThostFtdcInputOrderActionField> for CThostFtdcOrderField {
    fn as_mut(&mut self) -> &mut CThostFtdcInputOrderActionField {
        todo!()
    }
}

// 成交返回
impl AsMut<CThostFtdcInputOrderActionField> for CThostFtdcTradeField {
    fn as_mut(&mut self) -> &mut CThostFtdcInputOrderActionField {
        todo!()
    }
}

#[doc = "查询投资者持仓"]
#[ftdc_field_builder]
pub struct CThostFtdcQryInvestorPositionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}

#[doc = "查询投资者持仓明细"]
#[ftdc_field_builder]
pub struct CThostFtdcQryInvestorPositionDetailField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}

#[doc = "用户口令变更"]
#[ftdc_field_builder]
pub struct CThostFtdcUserPasswordUpdateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "原来的口令"]
    pub OldPassword: TThostFtdcPasswordType,
    #[doc = "新的口令"]
    pub NewPassword: TThostFtdcPasswordType,
}

#[doc = "查询行情"]
#[ftdc_field_builder]
pub struct CThostFtdcQryDepthMarketDataField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}

#[doc = "查询手续费率"]
#[ftdc_field_builder]
pub struct CThostFtdcQryInstrumentCommissionRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}

#[doc = "查询合约保证金率"]
#[ftdc_field_string]
pub struct CThostFtdcQryInstrumentMarginRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}

impl CThostFtdcQryInstrumentMarginRateField {
    pub fn with_hedge_flag(&mut self, flag: OrderHedgeFlag) -> &mut Self {
        self.HedgeFlag = flag;
        self
    }

    pub fn build(&self) -> Self {
        CThostFtdcQryInstrumentMarginRateField {
            BrokerID: self.BrokerID,
            InvestorID: self.InvestorID,
            InstrumentID: self.InstrumentID,
            ExchangeID: self.ExchangeID,
            InvestUnitID: self.InvestUnitID,
            HedgeFlag: self.HedgeFlag,
            ..Default::default()
        }
    }
}

#[doc = "查询客户通知"]
#[ftdc_field_builder]
pub struct CThostFtdcQryNoticeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}

pub trait InputOrderFieldTrait {
    /// 买开仓, FOK
    fn open_long_fok(&self) -> CThostFtdcInputOrderField;
    /// 买开仓, FAK
    fn open_long_fak(&self) -> CThostFtdcInputOrderField;
    /// 卖开仓, FOK
    fn open_short_fok(&self) -> CThostFtdcInputOrderField;
    /// 卖开仓, FAK
    fn open_short_fak(&self) -> CThostFtdcInputOrderField;

    /// 买平仓, FOK
    fn close_long_fok(&self) -> CThostFtdcInputOrderField;
    /// 买平仓, FAK
    fn close_long_fak(&self) -> CThostFtdcInputOrderField;
    /// 卖平仓, FOK
    fn close_short_fok(&self) -> CThostFtdcInputOrderField;
    /// 卖平仓, FOK
    fn close_short_fak(&self) -> CThostFtdcInputOrderField;
}

#[cfg(test)]
mod tests {
    use super::*;
}
