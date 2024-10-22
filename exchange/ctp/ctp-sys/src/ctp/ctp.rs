use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::fmt;

use crate::ctp::common::opts::*;
use crate::ext::{gbk, s};

pub const THOST_FTDC_EXP_Normal: u8 = 48u8;
pub const THOST_FTDC_EXP_GenOrderByTrade: u8 = 49u8;
pub const THOST_FTDC_ICT_EID: u8 = 48u8;
pub const THOST_FTDC_ICT_IDCard: u8 = 49u8;
pub const THOST_FTDC_ICT_OfficerIDCard: u8 = 50u8;
pub const THOST_FTDC_ICT_PoliceIDCard: u8 = 51u8;
pub const THOST_FTDC_ICT_SoldierIDCard: u8 = 52u8;
pub const THOST_FTDC_ICT_HouseholdRegister: u8 = 53u8;
pub const THOST_FTDC_ICT_Passport: u8 = 54u8;
pub const THOST_FTDC_ICT_TaiwanCompatriotIDCard: u8 = 55u8;
pub const THOST_FTDC_ICT_HomeComingCard: u8 = 56u8;
pub const THOST_FTDC_ICT_LicenseNo: u8 = 57u8;
pub const THOST_FTDC_ICT_TaxNo: u8 = 65u8;
pub const THOST_FTDC_ICT_HMMainlandTravelPermit: u8 = 66u8;
pub const THOST_FTDC_ICT_TwMainlandTravelPermit: u8 = 67u8;
pub const THOST_FTDC_ICT_DrivingLicense: u8 = 68u8;
pub const THOST_FTDC_ICT_SocialID: u8 = 70u8;
pub const THOST_FTDC_ICT_LocalID: u8 = 71u8;
pub const THOST_FTDC_ICT_BusinessRegistration: u8 = 72u8;
pub const THOST_FTDC_ICT_HKMCIDCard: u8 = 73u8;
pub const THOST_FTDC_ICT_AccountsPermits: u8 = 74u8;
pub const THOST_FTDC_ICT_FrgPrmtRdCard: u8 = 75u8;
pub const THOST_FTDC_ICT_CptMngPrdLetter: u8 = 76u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_ICT_UniformSocialCreditCode: u8 = 78u8;
pub const THOST_FTDC_ICT_CorporationCertNo: u8 = 79u8;
}}
pub const THOST_FTDC_ICT_OtherCard: u8 = 120u8;
pub const THOST_FTDC_IR_All: u8 = 49u8;
pub const THOST_FTDC_IR_Group: u8 = 50u8;
pub const THOST_FTDC_IR_Single: u8 = 51u8;
pub const THOST_FTDC_DR_All: u8 = 49u8;
pub const THOST_FTDC_DR_Group: u8 = 50u8;
pub const THOST_FTDC_DR_Single: u8 = 51u8;
pub const THOST_FTDC_DS_Asynchronous: u8 = 49u8;
pub const THOST_FTDC_DS_Synchronizing: u8 = 50u8;
pub const THOST_FTDC_DS_Synchronized: u8 = 51u8;
pub const THOST_FTDC_BDS_Synchronized: u8 = 49u8;
pub const THOST_FTDC_BDS_Synchronizing: u8 = 50u8;
pub const THOST_FTDC_ECS_NoConnection: u8 = 49u8;
pub const THOST_FTDC_ECS_QryInstrumentSent: u8 = 50u8;
pub const THOST_FTDC_ECS_GotInformation: u8 = 57u8;
pub const THOST_FTDC_TCS_NotConnected: u8 = 49u8;
pub const THOST_FTDC_TCS_Connected: u8 = 50u8;
pub const THOST_FTDC_TCS_QryInstrumentSent: u8 = 51u8;
pub const THOST_FTDC_TCS_SubPrivateFlow: u8 = 52u8;
pub const THOST_FTDC_FC_DataAsync: u8 = 49u8;
pub const THOST_FTDC_FC_ForceUserLogout: u8 = 50u8;
pub const THOST_FTDC_FC_UserPasswordUpdate: u8 = 51u8;
pub const THOST_FTDC_FC_BrokerPasswordUpdate: u8 = 52u8;
pub const THOST_FTDC_FC_InvestorPasswordUpdate: u8 = 53u8;
pub const THOST_FTDC_FC_OrderInsert: u8 = 54u8;
pub const THOST_FTDC_FC_OrderAction: u8 = 55u8;
pub const THOST_FTDC_FC_SyncSystemData: u8 = 56u8;
pub const THOST_FTDC_FC_SyncBrokerData: u8 = 57u8;
pub const THOST_FTDC_FC_BachSyncBrokerData: u8 = 65u8;
pub const THOST_FTDC_FC_SuperQuery: u8 = 66u8;
pub const THOST_FTDC_FC_ParkedOrderInsert: u8 = 67u8;
pub const THOST_FTDC_FC_ParkedOrderAction: u8 = 68u8;
pub const THOST_FTDC_FC_SyncOTP: u8 = 69u8;
pub const THOST_FTDC_FC_DeleteOrder: u8 = 70u8;
pub const THOST_FTDC_BFC_ForceUserLogout: u8 = 49u8;
pub const THOST_FTDC_BFC_UserPasswordUpdate: u8 = 50u8;
pub const THOST_FTDC_BFC_SyncBrokerData: u8 = 51u8;
pub const THOST_FTDC_BFC_BachSyncBrokerData: u8 = 52u8;
pub const THOST_FTDC_BFC_OrderInsert: u8 = 53u8;
pub const THOST_FTDC_BFC_OrderAction: u8 = 54u8;
pub const THOST_FTDC_BFC_AllQuery: u8 = 55u8;
pub const THOST_FTDC_BFC_log: u8 = 97u8;
pub const THOST_FTDC_BFC_BaseQry: u8 = 98u8;
pub const THOST_FTDC_BFC_TradeQry: u8 = 99u8;
pub const THOST_FTDC_BFC_Trade: u8 = 100u8;
pub const THOST_FTDC_BFC_Virement: u8 = 101u8;
pub const THOST_FTDC_BFC_Risk: u8 = 102u8;
pub const THOST_FTDC_BFC_Session: u8 = 103u8;
pub const THOST_FTDC_BFC_RiskNoticeCtl: u8 = 104u8;
pub const THOST_FTDC_BFC_RiskNotice: u8 = 105u8;
pub const THOST_FTDC_BFC_BrokerDeposit: u8 = 106u8;
pub const THOST_FTDC_BFC_QueryFund: u8 = 107u8;
pub const THOST_FTDC_BFC_QueryOrder: u8 = 108u8;
pub const THOST_FTDC_BFC_QueryTrade: u8 = 109u8;
pub const THOST_FTDC_BFC_QueryPosition: u8 = 110u8;
pub const THOST_FTDC_BFC_QueryMarketData: u8 = 111u8;
pub const THOST_FTDC_BFC_QueryUserEvent: u8 = 112u8;
pub const THOST_FTDC_BFC_QueryRiskNotify: u8 = 113u8;
pub const THOST_FTDC_BFC_QueryFundChange: u8 = 114u8;
pub const THOST_FTDC_BFC_QueryInvestor: u8 = 115u8;
pub const THOST_FTDC_BFC_QueryTradingCode: u8 = 116u8;
pub const THOST_FTDC_BFC_ForceClose: u8 = 117u8;
pub const THOST_FTDC_BFC_PressTest: u8 = 118u8;
pub const THOST_FTDC_BFC_RemainCalc: u8 = 119u8;
pub const THOST_FTDC_BFC_NetPositionInd: u8 = 120u8;
pub const THOST_FTDC_BFC_RiskPredict: u8 = 121u8;
pub const THOST_FTDC_BFC_DataExport: u8 = 122u8;
pub const THOST_FTDC_BFC_RiskTargetSetup: u8 = 65u8;
pub const THOST_FTDC_BFC_MarketDataWarn: u8 = 66u8;
pub const THOST_FTDC_BFC_QryBizNotice: u8 = 67u8;
pub const THOST_FTDC_BFC_CfgBizNotice: u8 = 68u8;
pub const THOST_FTDC_BFC_SyncOTP: u8 = 69u8;
pub const THOST_FTDC_BFC_SendBizNotice: u8 = 70u8;
pub const THOST_FTDC_BFC_CfgRiskLevelStd: u8 = 71u8;
pub const THOST_FTDC_BFC_TbCommand: u8 = 72u8;
pub const THOST_FTDC_BFC_DeleteOrder: u8 = 74u8;
pub const THOST_FTDC_BFC_ParkedOrderInsert: u8 = 75u8;
pub const THOST_FTDC_BFC_ParkedOrderAction: u8 = 76u8;
pub const THOST_FTDC_BFC_ExecOrderNoCheck: u8 = 77u8;
pub const THOST_FTDC_BFC_Designate: u8 = 78u8;
pub const THOST_FTDC_BFC_StockDisposal: u8 = 79u8;
pub const THOST_FTDC_BFC_BrokerDepositWarn: u8 = 81u8;
pub const THOST_FTDC_BFC_CoverWarn: u8 = 83u8;
pub const THOST_FTDC_BFC_PreExecOrder: u8 = 84u8;
pub const THOST_FTDC_BFC_ExecOrderRisk: u8 = 80u8;
pub const THOST_FTDC_BFC_PosiLimitWarn: u8 = 85u8;
pub const THOST_FTDC_BFC_QryPosiLimit: u8 = 86u8;
pub const THOST_FTDC_BFC_FBSign: u8 = 87u8;
pub const THOST_FTDC_BFC_FBAccount: u8 = 88u8;
pub const THOST_FTDC_OAS_Submitted: u8 = 97u8;
pub const THOST_FTDC_OAS_Accepted: u8 = 98u8;
pub const THOST_FTDC_OAS_Rejected: u8 = 99u8;
pub const THOST_FTDC_OST_AllTraded: u8 = 48u8;
pub const THOST_FTDC_OST_PartTradedQueueing: u8 = 49u8;
pub const THOST_FTDC_OST_PartTradedNotQueueing: u8 = 50u8;
pub const THOST_FTDC_OST_NoTradeQueueing: u8 = 51u8;
pub const THOST_FTDC_OST_NoTradeNotQueueing: u8 = 52u8;
pub const THOST_FTDC_OST_Canceled: u8 = 53u8;
pub const THOST_FTDC_OST_Unknown: u8 = 97u8;
pub const THOST_FTDC_OST_NotTouched: u8 = 98u8;
pub const THOST_FTDC_OST_Touched: u8 = 99u8;
pub const THOST_FTDC_OSS_InsertSubmitted: u8 = 48u8;
pub const THOST_FTDC_OSS_CancelSubmitted: u8 = 49u8;
pub const THOST_FTDC_OSS_ModifySubmitted: u8 = 50u8;
pub const THOST_FTDC_OSS_Accepted: u8 = 51u8;
pub const THOST_FTDC_OSS_InsertRejected: u8 = 52u8;
pub const THOST_FTDC_OSS_CancelRejected: u8 = 53u8;
pub const THOST_FTDC_OSS_ModifyRejected: u8 = 54u8;
pub const THOST_FTDC_PSD_Today: u8 = 49u8;
pub const THOST_FTDC_PSD_History: u8 = 50u8;
pub const THOST_FTDC_PDT_UseHistory: u8 = 49u8;
pub const THOST_FTDC_PDT_NoUseHistory: u8 = 50u8;
pub const THOST_FTDC_ER_Broker: u8 = 49u8;
pub const THOST_FTDC_ER_Host: u8 = 50u8;
pub const THOST_FTDC_ER_Maker: u8 = 51u8;
pub const THOST_FTDC_PC_Futures: u8 = 49u8;
pub const THOST_FTDC_PC_Options: u8 = 50u8;
pub const THOST_FTDC_PC_Combination: u8 = 51u8;
pub const THOST_FTDC_PC_Spot: u8 = 52u8;
pub const THOST_FTDC_PC_EFP: u8 = 53u8;
pub const THOST_FTDC_PC_SpotOption: u8 = 54u8;
cfg_if::cfg_if! {
if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_PC_TAS: u8 = 55u8;
pub const THOST_FTDC_PC_MI: u8 = 73u8;
pub const THOST_FTDC_APC_FutureSingle: u8 = 49u8;
pub const THOST_FTDC_APC_OptionSingle: u8 = 50u8;
pub const THOST_FTDC_APC_Futures: u8 = 51u8;
pub const THOST_FTDC_APC_Options: u8 = 52u8;
pub const THOST_FTDC_APC_TradingComb: u8 = 53u8;
pub const THOST_FTDC_APC_UnTradingComb: u8 = 54u8;
pub const THOST_FTDC_APC_AllTrading: u8 = 55u8;
pub const THOST_FTDC_APC_All: u8 = 56u8;
}}
pub const THOST_FTDC_IP_NotStart: u8 = 48u8;
pub const THOST_FTDC_IP_Started: u8 = 49u8;
pub const THOST_FTDC_IP_Pause: u8 = 50u8;
pub const THOST_FTDC_IP_Expired: u8 = 51u8;
pub const THOST_FTDC_D_Buy: u8 = 48u8;
pub const THOST_FTDC_D_Sell: u8 = 49u8;
pub const THOST_FTDC_PT_Net: u8 = 49u8;
pub const THOST_FTDC_PT_Gross: u8 = 50u8;
pub const THOST_FTDC_PD_Net: u8 = 49u8;
pub const THOST_FTDC_PD_Long: u8 = 50u8;
pub const THOST_FTDC_PD_Short: u8 = 51u8;
pub const THOST_FTDC_SS_NonActive: u8 = 49u8;
pub const THOST_FTDC_SS_Startup: u8 = 50u8;
pub const THOST_FTDC_SS_Operating: u8 = 51u8;
pub const THOST_FTDC_SS_Settlement: u8 = 52u8;
pub const THOST_FTDC_SS_SettlementFinished: u8 = 53u8;
pub const THOST_FTDC_RA_Trade: u8 = 48u8;
pub const THOST_FTDC_RA_Settlement: u8 = 49u8;
pub const THOST_FTDC_HF_Speculation: u8 = 49u8;
pub const THOST_FTDC_HF_Arbitrage: u8 = 50u8;
pub const THOST_FTDC_HF_Hedge: u8 = 51u8;
pub const THOST_FTDC_HF_MarketMaker: u8 = 53u8;
cfg_if::cfg_if! {if #[cfg(not(feature = "v6_3_13"))] {
pub const THOST_FTDC_HF_SpecHedge: u8 = 54u8;
pub const THOST_FTDC_HF_HedgeSpec: u8 = 55u8;
}}
pub const THOST_FTDC_BHF_Speculation: u8 = 49u8;
pub const THOST_FTDC_BHF_Arbitrage: u8 = 50u8;
pub const THOST_FTDC_BHF_Hedge: u8 = 51u8;
pub const THOST_FTDC_CIDT_Speculation: u8 = 49u8;
pub const THOST_FTDC_CIDT_Arbitrage: u8 = 50u8;
pub const THOST_FTDC_CIDT_Hedge: u8 = 51u8;
pub const THOST_FTDC_CIDT_MarketMaker: u8 = 53u8;
pub const THOST_FTDC_OPT_AnyPrice: u8 = 49u8;
pub const THOST_FTDC_OPT_LimitPrice: u8 = 50u8;
pub const THOST_FTDC_OPT_BestPrice: u8 = 51u8;
pub const THOST_FTDC_OPT_LastPrice: u8 = 52u8;
pub const THOST_FTDC_OPT_LastPricePlusOneTicks: u8 = 53u8;
pub const THOST_FTDC_OPT_LastPricePlusTwoTicks: u8 = 54u8;
pub const THOST_FTDC_OPT_LastPricePlusThreeTicks: u8 = 55u8;
pub const THOST_FTDC_OPT_AskPrice1: u8 = 56u8;
pub const THOST_FTDC_OPT_AskPrice1PlusOneTicks: u8 = 57u8;
pub const THOST_FTDC_OPT_AskPrice1PlusTwoTicks: u8 = 65u8;
pub const THOST_FTDC_OPT_AskPrice1PlusThreeTicks: u8 = 66u8;
pub const THOST_FTDC_OPT_BidPrice1: u8 = 67u8;
pub const THOST_FTDC_OPT_BidPrice1PlusOneTicks: u8 = 68u8;
pub const THOST_FTDC_OPT_BidPrice1PlusTwoTicks: u8 = 69u8;
pub const THOST_FTDC_OPT_BidPrice1PlusThreeTicks: u8 = 70u8;
pub const THOST_FTDC_OPT_FiveLevelPrice: u8 = 71u8;
pub const THOST_FTDC_OF_Open: u8 = 48u8;
pub const THOST_FTDC_OF_Close: u8 = 49u8;
pub const THOST_FTDC_OF_ForceClose: u8 = 50u8;
pub const THOST_FTDC_OF_CloseToday: u8 = 51u8;
pub const THOST_FTDC_OF_CloseYesterday: u8 = 52u8;
pub const THOST_FTDC_OF_ForceOff: u8 = 53u8;
pub const THOST_FTDC_OF_LocalForceClose: u8 = 54u8;
pub const THOST_FTDC_FCC_NotForceClose: u8 = 48u8;
pub const THOST_FTDC_FCC_LackDeposit: u8 = 49u8;
pub const THOST_FTDC_FCC_ClientOverPositionLimit: u8 = 50u8;
pub const THOST_FTDC_FCC_MemberOverPositionLimit: u8 = 51u8;
pub const THOST_FTDC_FCC_NotMultiple: u8 = 52u8;
pub const THOST_FTDC_FCC_Violation: u8 = 53u8;
pub const THOST_FTDC_FCC_Other: u8 = 54u8;
pub const THOST_FTDC_FCC_PersonDeliv: u8 = 55u8;
pub const THOST_FTDC_ORDT_Normal: u8 = 48u8;
pub const THOST_FTDC_ORDT_DeriveFromQuote: u8 = 49u8;
pub const THOST_FTDC_ORDT_DeriveFromCombination: u8 = 50u8;
pub const THOST_FTDC_ORDT_Combination: u8 = 51u8;
pub const THOST_FTDC_ORDT_ConditionalOrder: u8 = 52u8;
pub const THOST_FTDC_ORDT_Swap: u8 = 53u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_ORDT_DeriveFromBlockTrade: u8 = 54u8;
pub const THOST_FTDC_ORDT_DeriveFromEFPTrade: u8 = 55u8;
}}
cfg_if::cfg_if! {if #[cfg(feature = "v6_3_13")] {
pub const THOST_FTDC_ORDT_DeriveFromEFP: u8 = 54u8;
}}
pub const THOST_FTDC_TC_IOC: u8 = 49u8;
pub const THOST_FTDC_TC_GFS: u8 = 50u8;
pub const THOST_FTDC_TC_GFD: u8 = 51u8;
pub const THOST_FTDC_TC_GTD: u8 = 52u8;
pub const THOST_FTDC_TC_GTC: u8 = 53u8;
pub const THOST_FTDC_TC_GFA: u8 = 54u8;
pub const THOST_FTDC_VC_AV: u8 = 49u8;
pub const THOST_FTDC_VC_MV: u8 = 50u8;
pub const THOST_FTDC_VC_CV: u8 = 51u8;
pub const THOST_FTDC_CC_Immediately: u8 = 49u8;
pub const THOST_FTDC_CC_Touch: u8 = 50u8;
pub const THOST_FTDC_CC_TouchProfit: u8 = 51u8;
pub const THOST_FTDC_CC_ParkedOrder: u8 = 52u8;
pub const THOST_FTDC_CC_LastPriceGreaterThanStopPrice: u8 = 53u8;
pub const THOST_FTDC_CC_LastPriceGreaterEqualStopPrice: u8 = 54u8;
pub const THOST_FTDC_CC_LastPriceLesserThanStopPrice: u8 = 55u8;
pub const THOST_FTDC_CC_LastPriceLesserEqualStopPrice: u8 = 56u8;
pub const THOST_FTDC_CC_AskPriceGreaterThanStopPrice: u8 = 57u8;
pub const THOST_FTDC_CC_AskPriceGreaterEqualStopPrice: u8 = 65u8;
pub const THOST_FTDC_CC_AskPriceLesserThanStopPrice: u8 = 66u8;
pub const THOST_FTDC_CC_AskPriceLesserEqualStopPrice: u8 = 67u8;
pub const THOST_FTDC_CC_BidPriceGreaterThanStopPrice: u8 = 68u8;
pub const THOST_FTDC_CC_BidPriceGreaterEqualStopPrice: u8 = 69u8;
pub const THOST_FTDC_CC_BidPriceLesserThanStopPrice: u8 = 70u8;
pub const THOST_FTDC_CC_BidPriceLesserEqualStopPrice: u8 = 72u8;
pub const THOST_FTDC_AF_Delete: u8 = 48u8;
pub const THOST_FTDC_AF_Modify: u8 = 51u8;
pub const THOST_FTDC_TR_Allow: u8 = 48u8;
pub const THOST_FTDC_TR_CloseOnly: u8 = 49u8;
pub const THOST_FTDC_TR_Forbidden: u8 = 50u8;
pub const THOST_FTDC_OSRC_Participant: u8 = 48u8;
pub const THOST_FTDC_OSRC_Administrator: u8 = 49u8;
pub const THOST_FTDC_TRDT_SplitCombination: u8 = 35u8;
pub const THOST_FTDC_TRDT_Common: u8 = 48u8;
pub const THOST_FTDC_TRDT_OptionsExecution: u8 = 49u8;
pub const THOST_FTDC_TRDT_OTC: u8 = 50u8;
pub const THOST_FTDC_TRDT_EFPDerived: u8 = 51u8;
pub const THOST_FTDC_TRDT_CombinationDerived: u8 = 52u8;
pub const THOST_FTDC_TRDT_BlockTrade: u8 = 53u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_SPOST_Common: u8 = 35u8;
pub const THOST_FTDC_SPOST_Tas: u8 = 48u8;
}}
pub const THOST_FTDC_PSRC_LastPrice: u8 = 48u8;
pub const THOST_FTDC_PSRC_Buy: u8 = 49u8;
pub const THOST_FTDC_PSRC_Sell: u8 = 50u8;
cfg_if::cfg_if! {if #[cfg(not(feature = "v6_3_13"))] {
pub const THOST_FTDC_PSRC_OTC: u8 = 51u8;
}}
pub const THOST_FTDC_IS_BeforeTrading: u8 = 48u8;
pub const THOST_FTDC_IS_NoTrading: u8 = 49u8;
pub const THOST_FTDC_IS_Continous: u8 = 50u8;
pub const THOST_FTDC_IS_AuctionOrdering: u8 = 51u8;
pub const THOST_FTDC_IS_AuctionBalance: u8 = 52u8;
pub const THOST_FTDC_IS_AuctionMatch: u8 = 53u8;
pub const THOST_FTDC_IS_Closed: u8 = 54u8;
pub const THOST_FTDC_IER_Automatic: u8 = 49u8;
pub const THOST_FTDC_IER_Manual: u8 = 50u8;
pub const THOST_FTDC_IER_Fuse: u8 = 51u8;
pub const THOST_FTDC_BS_NoUpload: u8 = 49u8;
pub const THOST_FTDC_BS_Uploaded: u8 = 50u8;
pub const THOST_FTDC_BS_Failed: u8 = 51u8;
pub const THOST_FTDC_RS_All: u8 = 49u8;
pub const THOST_FTDC_RS_ByProduct: u8 = 50u8;
pub const THOST_FTDC_RP_ByVolume: u8 = 49u8;
pub const THOST_FTDC_RP_ByFeeOnHand: u8 = 50u8;
pub const THOST_FTDC_RL_Level1: u8 = 49u8;
pub const THOST_FTDC_RL_Level2: u8 = 50u8;
pub const THOST_FTDC_RL_Level3: u8 = 51u8;
pub const THOST_FTDC_RL_Level4: u8 = 52u8;
pub const THOST_FTDC_RL_Level5: u8 = 53u8;
pub const THOST_FTDC_RL_Level6: u8 = 54u8;
pub const THOST_FTDC_RL_Level7: u8 = 55u8;
pub const THOST_FTDC_RL_Level8: u8 = 56u8;
pub const THOST_FTDC_RL_Level9: u8 = 57u8;
pub const THOST_FTDC_RSD_ByPeriod: u8 = 49u8;
pub const THOST_FTDC_RSD_ByStandard: u8 = 50u8;
pub const THOST_FTDC_MT_Out: u8 = 48u8;
pub const THOST_FTDC_MT_In: u8 = 49u8;
pub const THOST_FTDC_ISPI_MortgageRatio: u8 = 52u8;
pub const THOST_FTDC_ISPI_MarginWay: u8 = 53u8;
pub const THOST_FTDC_ISPI_BillDeposit: u8 = 57u8;
pub const THOST_FTDC_ESPI_MortgageRatio: u8 = 49u8;
pub const THOST_FTDC_ESPI_OtherFundItem: u8 = 50u8;
pub const THOST_FTDC_ESPI_OtherFundImport: u8 = 51u8;
pub const THOST_FTDC_ESPI_CFFEXMinPrepa: u8 = 54u8;
pub const THOST_FTDC_ESPI_CZCESettlementType: u8 = 55u8;
pub const THOST_FTDC_ESPI_ExchDelivFeeMode: u8 = 57u8;
pub const THOST_FTDC_ESPI_DelivFeeMode: u8 = 48u8;
pub const THOST_FTDC_ESPI_CZCEComMarginType: u8 = 65u8;
pub const THOST_FTDC_ESPI_DceComMarginType: u8 = 66u8;
pub const THOST_FTDC_ESPI_OptOutDisCountRate: u8 = 97u8;
pub const THOST_FTDC_ESPI_OptMiniGuarantee: u8 = 98u8;
pub const THOST_FTDC_SPI_InvestorIDMinLength: u8 = 49u8;
pub const THOST_FTDC_SPI_AccountIDMinLength: u8 = 50u8;
pub const THOST_FTDC_SPI_UserRightLogon: u8 = 51u8;
pub const THOST_FTDC_SPI_SettlementBillTrade: u8 = 52u8;
pub const THOST_FTDC_SPI_TradingCode: u8 = 53u8;
pub const THOST_FTDC_SPI_CheckFund: u8 = 54u8;
pub const THOST_FTDC_SPI_CommModelRight: u8 = 55u8;
pub const THOST_FTDC_SPI_MarginModelRight: u8 = 57u8;
pub const THOST_FTDC_SPI_IsStandardActive: u8 = 56u8;
pub const THOST_FTDC_SPI_UploadSettlementFile: u8 = 85u8;
pub const THOST_FTDC_SPI_DownloadCSRCFile: u8 = 68u8;
pub const THOST_FTDC_SPI_SettlementBillFile: u8 = 83u8;
pub const THOST_FTDC_SPI_CSRCOthersFile: u8 = 67u8;
pub const THOST_FTDC_SPI_InvestorPhoto: u8 = 80u8;
pub const THOST_FTDC_SPI_CSRCData: u8 = 82u8;
pub const THOST_FTDC_SPI_InvestorPwdModel: u8 = 73u8;
pub const THOST_FTDC_SPI_CFFEXInvestorSettleFile: u8 = 70u8;
pub const THOST_FTDC_SPI_InvestorIDType: u8 = 97u8;
pub const THOST_FTDC_SPI_FreezeMaxReMain: u8 = 114u8;
pub const THOST_FTDC_SPI_IsSync: u8 = 65u8;
pub const THOST_FTDC_SPI_RelieveOpenLimit: u8 = 79u8;
pub const THOST_FTDC_SPI_IsStandardFreeze: u8 = 88u8;
pub const THOST_FTDC_SPI_CZCENormalProductHedge: u8 = 66u8;
pub const THOST_FTDC_TPID_EncryptionStandard: u8 = 69u8;
pub const THOST_FTDC_TPID_RiskMode: u8 = 82u8;
pub const THOST_FTDC_TPID_RiskModeGlobal: u8 = 71u8;
pub const THOST_FTDC_TPID_modeEncode: u8 = 80u8;
pub const THOST_FTDC_TPID_tickMode: u8 = 84u8;
pub const THOST_FTDC_TPID_SingleUserSessionMaxNum: u8 = 83u8;
pub const THOST_FTDC_TPID_LoginFailMaxNum: u8 = 76u8;
pub const THOST_FTDC_TPID_IsAuthForce: u8 = 65u8;
pub const THOST_FTDC_TPID_IsPosiFreeze: u8 = 70u8;
pub const THOST_FTDC_TPID_IsPosiLimit: u8 = 77u8;
pub const THOST_FTDC_TPID_ForQuoteTimeInterval: u8 = 81u8;
pub const THOST_FTDC_TPID_IsFuturePosiLimit: u8 = 66u8;
pub const THOST_FTDC_TPID_IsFutureOrderFreq: u8 = 67u8;
pub const THOST_FTDC_TPID_IsExecOrderProfit: u8 = 72u8;
pub const THOST_FTDC_TPID_IsCheckBankAcc: u8 = 73u8;
pub const THOST_FTDC_TPID_PasswordDeadLine: u8 = 74u8;
pub const THOST_FTDC_TPID_IsStrongPassword: u8 = 75u8;
pub const THOST_FTDC_TPID_BalanceMorgage: u8 = 97u8;
pub const THOST_FTDC_TPID_MinPwdLen: u8 = 79u8;
pub const THOST_FTDC_TPID_LoginFailMaxNumForIP: u8 = 85u8;
pub const THOST_FTDC_TPID_PasswordPeriod: u8 = 86u8;
pub const THOST_FTDC_FI_SettlementFund: u8 = 70u8;
pub const THOST_FTDC_FI_Trade: u8 = 84u8;
pub const THOST_FTDC_FI_InvestorPosition: u8 = 80u8;
pub const THOST_FTDC_FI_SubEntryFund: u8 = 79u8;
pub const THOST_FTDC_FI_CZCECombinationPos: u8 = 67u8;
pub const THOST_FTDC_FI_CSRCData: u8 = 82u8;
pub const THOST_FTDC_FI_CZCEClose: u8 = 76u8;
pub const THOST_FTDC_FI_CZCENoClose: u8 = 78u8;
pub const THOST_FTDC_FI_PositionDtl: u8 = 68u8;
pub const THOST_FTDC_FI_OptionStrike: u8 = 83u8;
pub const THOST_FTDC_FI_SettlementPriceComparison: u8 = 77u8;
pub const THOST_FTDC_FI_NonTradePosChange: u8 = 66u8;
pub const THOST_FTDC_FUT_Settlement: u8 = 48u8;
pub const THOST_FTDC_FUT_Check: u8 = 49u8;
pub const THOST_FTDC_FFT_Txt: u8 = 48u8;
pub const THOST_FTDC_FFT_Zip: u8 = 49u8;
pub const THOST_FTDC_FFT_DBF: u8 = 50u8;
pub const THOST_FTDC_FUS_SucceedUpload: u8 = 49u8;
pub const THOST_FTDC_FUS_FailedUpload: u8 = 50u8;
pub const THOST_FTDC_FUS_SucceedLoad: u8 = 51u8;
pub const THOST_FTDC_FUS_PartSucceedLoad: u8 = 52u8;
pub const THOST_FTDC_FUS_FailedLoad: u8 = 53u8;
pub const THOST_FTDC_TD_Out: u8 = 48u8;
pub const THOST_FTDC_TD_In: u8 = 49u8;
pub const THOST_FTDC_SC_NoSpecialRule: u8 = 48u8;
pub const THOST_FTDC_SC_NoSpringFestival: u8 = 49u8;
pub const THOST_FTDC_IPT_LastSettlement: u8 = 49u8;
pub const THOST_FTDC_IPT_LaseClose: u8 = 50u8;
pub const THOST_FTDC_PLP_Active: u8 = 49u8;
pub const THOST_FTDC_PLP_NonActive: u8 = 50u8;
pub const THOST_FTDC_PLP_Canceled: u8 = 51u8;
pub const THOST_FTDC_DM_CashDeliv: u8 = 49u8;
pub const THOST_FTDC_DM_CommodityDeliv: u8 = 50u8;
pub const THOST_FTDC_FIOT_FundIO: u8 = 49u8;
pub const THOST_FTDC_FIOT_Transfer: u8 = 50u8;
pub const THOST_FTDC_FIOT_SwapCurrency: u8 = 51u8;
pub const THOST_FTDC_FT_Deposite: u8 = 49u8;
pub const THOST_FTDC_FT_ItemFund: u8 = 50u8;
pub const THOST_FTDC_FT_Company: u8 = 51u8;
pub const THOST_FTDC_FT_InnerTransfer: u8 = 52u8;
pub const THOST_FTDC_FD_In: u8 = 49u8;
pub const THOST_FTDC_FD_Out: u8 = 50u8;
pub const THOST_FTDC_FS_Record: u8 = 49u8;
pub const THOST_FTDC_FS_Check: u8 = 50u8;
pub const THOST_FTDC_FS_Charge: u8 = 51u8;
pub const THOST_FTDC_PS_None: u8 = 49u8;
pub const THOST_FTDC_PS_Publishing: u8 = 50u8;
pub const THOST_FTDC_PS_Published: u8 = 51u8;
pub const THOST_FTDC_ES_NonActive: u8 = 49u8;
pub const THOST_FTDC_ES_Startup: u8 = 50u8;
pub const THOST_FTDC_ES_Initialize: u8 = 51u8;
pub const THOST_FTDC_ES_Initialized: u8 = 52u8;
pub const THOST_FTDC_ES_Close: u8 = 53u8;
pub const THOST_FTDC_ES_Closed: u8 = 54u8;
pub const THOST_FTDC_ES_Settlement: u8 = 55u8;
pub const THOST_FTDC_STS_Initialize: u8 = 48u8;
pub const THOST_FTDC_STS_Settlementing: u8 = 49u8;
pub const THOST_FTDC_STS_Settlemented: u8 = 50u8;
pub const THOST_FTDC_STS_Finished: u8 = 51u8;
pub const THOST_FTDC_CT_Person: u8 = 48u8;
pub const THOST_FTDC_CT_Company: u8 = 49u8;
pub const THOST_FTDC_CT_Fund: u8 = 50u8;
pub const THOST_FTDC_CT_SpecialOrgan: u8 = 51u8;
pub const THOST_FTDC_CT_Asset: u8 = 52u8;
pub const THOST_FTDC_BT_Trade: u8 = 48u8;
pub const THOST_FTDC_BT_TradeSettle: u8 = 49u8;
pub const THOST_FTDC_FAS_Low: u8 = 49u8;
pub const THOST_FTDC_FAS_Normal: u8 = 50u8;
pub const THOST_FTDC_FAS_Focus: u8 = 51u8;
pub const THOST_FTDC_FAS_Risk: u8 = 52u8;
pub const THOST_FTDC_FAS_ByTrade: u8 = 49u8;
pub const THOST_FTDC_FAS_ByDeliv: u8 = 50u8;
pub const THOST_FTDC_FAS_None: u8 = 51u8;
pub const THOST_FTDC_FAS_FixFee: u8 = 52u8;
pub const THOST_FTDC_PWDT_Trade: u8 = 49u8;
pub const THOST_FTDC_PWDT_Account: u8 = 50u8;
pub const THOST_FTDC_AG_All: u8 = 49u8;
pub const THOST_FTDC_AG_OnlyLost: u8 = 50u8;
pub const THOST_FTDC_AG_OnlyGain: u8 = 51u8;
pub const THOST_FTDC_AG_None: u8 = 52u8;
pub const THOST_FTDC_ICP_Include: u8 = 48u8;
pub const THOST_FTDC_ICP_NotInclude: u8 = 50u8;
pub const THOST_FTDC_AWT_Enable: u8 = 48u8;
pub const THOST_FTDC_AWT_Disable: u8 = 50u8;
pub const THOST_FTDC_AWT_NoHoldEnable: u8 = 51u8;
pub const THOST_FTDC_FPWD_UnCheck: u8 = 48u8;
pub const THOST_FTDC_FPWD_Check: u8 = 49u8;
pub const THOST_FTDC_TT_BankToFuture: u8 = 48u8;
pub const THOST_FTDC_TT_FutureToBank: u8 = 49u8;
pub const THOST_FTDC_TVF_Invalid: u8 = 48u8;
pub const THOST_FTDC_TVF_Valid: u8 = 49u8;
pub const THOST_FTDC_TVF_Reverse: u8 = 50u8;
pub const THOST_FTDC_RN_CD: u8 = 48u8;
pub const THOST_FTDC_RN_ZT: u8 = 49u8;
pub const THOST_FTDC_RN_QT: u8 = 50u8;
pub const THOST_FTDC_SEX_None: u8 = 48u8;
pub const THOST_FTDC_SEX_Man: u8 = 49u8;
pub const THOST_FTDC_SEX_Woman: u8 = 50u8;
pub const THOST_FTDC_UT_Investor: u8 = 48u8;
pub const THOST_FTDC_UT_Operator: u8 = 49u8;
pub const THOST_FTDC_UT_SuperUser: u8 = 50u8;
pub const THOST_FTDC_RATETYPE_MarginRate: u8 = 50u8;
pub const THOST_FTDC_NOTETYPE_TradeSettleBill: u8 = 49u8;
pub const THOST_FTDC_NOTETYPE_TradeSettleMonth: u8 = 50u8;
pub const THOST_FTDC_NOTETYPE_CallMarginNotes: u8 = 51u8;
pub const THOST_FTDC_NOTETYPE_ForceCloseNotes: u8 = 52u8;
pub const THOST_FTDC_NOTETYPE_TradeNotes: u8 = 53u8;
pub const THOST_FTDC_NOTETYPE_DelivNotes: u8 = 54u8;
pub const THOST_FTDC_SBS_Day: u8 = 49u8;
pub const THOST_FTDC_SBS_Volume: u8 = 50u8;
pub const THOST_FTDC_ST_Day: u8 = 48u8;
pub const THOST_FTDC_ST_Month: u8 = 49u8;
pub const THOST_FTDC_URT_Logon: u8 = 49u8;
pub const THOST_FTDC_URT_Transfer: u8 = 50u8;
pub const THOST_FTDC_URT_EMail: u8 = 51u8;
pub const THOST_FTDC_URT_Fax: u8 = 52u8;
pub const THOST_FTDC_URT_ConditionOrder: u8 = 53u8;
pub const THOST_FTDC_MPT_PreSettlementPrice: u8 = 49u8;
pub const THOST_FTDC_MPT_SettlementPrice: u8 = 50u8;
pub const THOST_FTDC_MPT_AveragePrice: u8 = 51u8;
pub const THOST_FTDC_MPT_OpenPrice: u8 = 52u8;
pub const THOST_FTDC_BGS_None: u8 = 48u8;
pub const THOST_FTDC_BGS_NoGenerated: u8 = 49u8;
pub const THOST_FTDC_BGS_Generated: u8 = 50u8;
pub const THOST_FTDC_AT_HandlePositionAlgo: u8 = 49u8;
pub const THOST_FTDC_AT_FindMarginRateAlgo: u8 = 50u8;
pub const THOST_FTDC_HPA_Base: u8 = 49u8;
pub const THOST_FTDC_HPA_DCE: u8 = 50u8;
pub const THOST_FTDC_HPA_CZCE: u8 = 51u8;
pub const THOST_FTDC_FMRA_Base: u8 = 49u8;
pub const THOST_FTDC_FMRA_DCE: u8 = 50u8;
pub const THOST_FTDC_FMRA_CZCE: u8 = 51u8;
pub const THOST_FTDC_HTAA_Base: u8 = 49u8;
pub const THOST_FTDC_HTAA_DCE: u8 = 50u8;
pub const THOST_FTDC_HTAA_CZCE: u8 = 51u8;
pub const THOST_FTDC_PST_Order: u8 = 49u8;
pub const THOST_FTDC_PST_Open: u8 = 50u8;
pub const THOST_FTDC_PST_Fund: u8 = 51u8;
pub const THOST_FTDC_PST_Settlement: u8 = 52u8;
pub const THOST_FTDC_PST_Company: u8 = 53u8;
pub const THOST_FTDC_PST_Corporation: u8 = 54u8;
pub const THOST_FTDC_PST_LinkMan: u8 = 55u8;
pub const THOST_FTDC_PST_Ledger: u8 = 56u8;
pub const THOST_FTDC_PST_Trustee: u8 = 57u8;
pub const THOST_FTDC_PST_TrusteeCorporation: u8 = 65u8;
pub const THOST_FTDC_PST_TrusteeOpen: u8 = 66u8;
pub const THOST_FTDC_PST_TrusteeContact: u8 = 67u8;
pub const THOST_FTDC_PST_ForeignerRefer: u8 = 68u8;
pub const THOST_FTDC_PST_CorporationRefer: u8 = 69u8;
pub const THOST_FTDC_QIR_All: u8 = 49u8;
pub const THOST_FTDC_QIR_Group: u8 = 50u8;
pub const THOST_FTDC_QIR_Single: u8 = 51u8;
pub const THOST_FTDC_IRS_Normal: u8 = 49u8;
pub const THOST_FTDC_IRS_Warn: u8 = 50u8;
pub const THOST_FTDC_IRS_Call: u8 = 51u8;
pub const THOST_FTDC_IRS_Force: u8 = 52u8;
pub const THOST_FTDC_IRS_Exception: u8 = 53u8;
pub const THOST_FTDC_UET_Login: u8 = 49u8;
pub const THOST_FTDC_UET_Logout: u8 = 50u8;
pub const THOST_FTDC_UET_Trading: u8 = 51u8;
pub const THOST_FTDC_UET_TradingError: u8 = 52u8;
pub const THOST_FTDC_UET_UpdatePassword: u8 = 53u8;
pub const THOST_FTDC_UET_Authenticate: u8 = 54u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_UET_SubmitSysInfo: u8 = 55u8;
pub const THOST_FTDC_UET_Transfer: u8 = 56u8;
}}
pub const THOST_FTDC_UET_Other: u8 = 57u8;
pub const THOST_FTDC_ICS_Close: u8 = 48u8;
pub const THOST_FTDC_ICS_CloseToday: u8 = 49u8;
pub const THOST_FTDC_SM_Non: u8 = 48u8;
pub const THOST_FTDC_SM_Instrument: u8 = 49u8;
pub const THOST_FTDC_SM_Product: u8 = 50u8;
pub const THOST_FTDC_SM_Investor: u8 = 51u8;
pub const THOST_FTDC_PAOS_NotSend: u8 = 49u8;
pub const THOST_FTDC_PAOS_Send: u8 = 50u8;
pub const THOST_FTDC_PAOS_Deleted: u8 = 51u8;
pub const THOST_FTDC_VDS_Dealing: u8 = 49u8;
pub const THOST_FTDC_VDS_DeaclSucceed: u8 = 50u8;
pub const THOST_FTDC_ORGS_Standard: u8 = 48u8;
pub const THOST_FTDC_ORGS_ESunny: u8 = 49u8;
pub const THOST_FTDC_ORGS_KingStarV6: u8 = 50u8;
pub const THOST_FTDC_VTS_NaturalDeal: u8 = 48u8;
pub const THOST_FTDC_VTS_SucceedEnd: u8 = 49u8;
pub const THOST_FTDC_VTS_FailedEND: u8 = 50u8;
pub const THOST_FTDC_VTS_Exception: u8 = 51u8;
pub const THOST_FTDC_VTS_ManualDeal: u8 = 52u8;
pub const THOST_FTDC_VTS_MesException: u8 = 53u8;
pub const THOST_FTDC_VTS_SysException: u8 = 54u8;
pub const THOST_FTDC_VBAT_BankBook: u8 = 49u8;
pub const THOST_FTDC_VBAT_BankCard: u8 = 50u8;
pub const THOST_FTDC_VBAT_CreditCard: u8 = 51u8;
pub const THOST_FTDC_VMS_Natural: u8 = 48u8;
pub const THOST_FTDC_VMS_Canceled: u8 = 57u8;
pub const THOST_FTDC_VAA_NoAvailAbility: u8 = 48u8;
pub const THOST_FTDC_VAA_AvailAbility: u8 = 49u8;
pub const THOST_FTDC_VAA_Repeal: u8 = 50u8;
pub const THOST_FTDC_GEN_Program: u8 = 48u8;
pub const THOST_FTDC_GEN_HandWork: u8 = 49u8;
pub const THOST_FTDC_CFMMCKK_REQUEST: u8 = 82u8;
pub const THOST_FTDC_CFMMCKK_AUTO: u8 = 65u8;
pub const THOST_FTDC_CFMMCKK_MANUAL: u8 = 77u8;
pub const THOST_FTDC_CFT_IDCard: u8 = 48u8;
pub const THOST_FTDC_CFT_Passport: u8 = 49u8;
pub const THOST_FTDC_CFT_OfficerIDCard: u8 = 50u8;
pub const THOST_FTDC_CFT_SoldierIDCard: u8 = 51u8;
pub const THOST_FTDC_CFT_HomeComingCard: u8 = 52u8;
pub const THOST_FTDC_CFT_HouseholdRegister: u8 = 53u8;
pub const THOST_FTDC_CFT_LicenseNo: u8 = 54u8;
pub const THOST_FTDC_CFT_InstitutionCodeCard: u8 = 55u8;
pub const THOST_FTDC_CFT_TempLicenseNo: u8 = 56u8;
pub const THOST_FTDC_CFT_NoEnterpriseLicenseNo: u8 = 57u8;
pub const THOST_FTDC_CFT_OtherCard: u8 = 120u8;
pub const THOST_FTDC_CFT_SuperDepAgree: u8 = 97u8;
pub const THOST_FTDC_FBC_Others: u8 = 48u8;
pub const THOST_FTDC_FBC_TransferDetails: u8 = 49u8;
pub const THOST_FTDC_FBC_CustAccStatus: u8 = 50u8;
pub const THOST_FTDC_FBC_AccountTradeDetails: u8 = 51u8;
pub const THOST_FTDC_FBC_FutureAccountChangeInfoDetails: u8 = 52u8;
pub const THOST_FTDC_FBC_CustMoneyDetail: u8 = 53u8;
pub const THOST_FTDC_FBC_CustCancelAccountInfo: u8 = 54u8;
pub const THOST_FTDC_FBC_CustMoneyResult: u8 = 55u8;
pub const THOST_FTDC_FBC_OthersExceptionResult: u8 = 56u8;
pub const THOST_FTDC_FBC_CustInterestNetMoneyDetails: u8 = 57u8;
pub const THOST_FTDC_FBC_CustMoneySendAndReceiveDetails: u8 = 97u8;
pub const THOST_FTDC_FBC_CorporationMoneyTotal: u8 = 98u8;
pub const THOST_FTDC_FBC_MainbodyMoneyTotal: u8 = 99u8;
pub const THOST_FTDC_FBC_MainPartMonitorData: u8 = 100u8;
pub const THOST_FTDC_FBC_PreparationMoney: u8 = 101u8;
pub const THOST_FTDC_FBC_BankMoneyMonitorData: u8 = 102u8;
pub const THOST_FTDC_CEC_Exchange: u8 = 49u8;
pub const THOST_FTDC_CEC_Cash: u8 = 50u8;
pub const THOST_FTDC_YNI_Yes: u8 = 48u8;
pub const THOST_FTDC_YNI_No: u8 = 49u8;
pub const THOST_FTDC_BLT_CurrentMoney: u8 = 48u8;
pub const THOST_FTDC_BLT_UsableMoney: u8 = 49u8;
pub const THOST_FTDC_BLT_FetchableMoney: u8 = 50u8;
pub const THOST_FTDC_BLT_FreezeMoney: u8 = 51u8;
pub const THOST_FTDC_GD_Unknown: u8 = 48u8;
pub const THOST_FTDC_GD_Male: u8 = 49u8;
pub const THOST_FTDC_GD_Female: u8 = 50u8;
pub const THOST_FTDC_FPF_BEN: u8 = 48u8;
pub const THOST_FTDC_FPF_OUR: u8 = 49u8;
pub const THOST_FTDC_FPF_SHA: u8 = 50u8;
pub const THOST_FTDC_PWKT_ExchangeKey: u8 = 48u8;
pub const THOST_FTDC_PWKT_PassWordKey: u8 = 49u8;
pub const THOST_FTDC_PWKT_MACKey: u8 = 50u8;
pub const THOST_FTDC_PWKT_MessageKey: u8 = 51u8;
pub const THOST_FTDC_PWT_Query: u8 = 48u8;
pub const THOST_FTDC_PWT_Fetch: u8 = 49u8;
pub const THOST_FTDC_PWT_Transfer: u8 = 50u8;
pub const THOST_FTDC_PWT_Trade: u8 = 51u8;
pub const THOST_FTDC_EM_NoEncry: u8 = 48u8;
pub const THOST_FTDC_EM_DES: u8 = 49u8;
pub const THOST_FTDC_EM_3DES: u8 = 50u8;
pub const THOST_FTDC_BRF_BankNotNeedRepeal: u8 = 48u8;
pub const THOST_FTDC_BRF_BankWaitingRepeal: u8 = 49u8;
pub const THOST_FTDC_BRF_BankBeenRepealed: u8 = 50u8;
pub const THOST_FTDC_BRORF_BrokerNotNeedRepeal: u8 = 48u8;
pub const THOST_FTDC_BRORF_BrokerWaitingRepeal: u8 = 49u8;
pub const THOST_FTDC_BRORF_BrokerBeenRepealed: u8 = 50u8;
pub const THOST_FTDC_TS_Bank: u8 = 48u8;
pub const THOST_FTDC_TS_Future: u8 = 49u8;
pub const THOST_FTDC_TS_Store: u8 = 50u8;
pub const THOST_FTDC_LF_Yes: u8 = 48u8;
pub const THOST_FTDC_LF_No: u8 = 49u8;
pub const THOST_FTDC_BAS_Normal: u8 = 48u8;
pub const THOST_FTDC_BAS_Freeze: u8 = 49u8;
pub const THOST_FTDC_BAS_ReportLoss: u8 = 50u8;
pub const THOST_FTDC_MAS_Normal: u8 = 48u8;
pub const THOST_FTDC_MAS_Cancel: u8 = 49u8;
pub const THOST_FTDC_MSS_Point: u8 = 48u8;
pub const THOST_FTDC_MSS_PrePoint: u8 = 49u8;
pub const THOST_FTDC_MSS_CancelPoint: u8 = 50u8;
pub const THOST_FTDC_SYT_FutureBankTransfer: u8 = 48u8;
pub const THOST_FTDC_SYT_StockBankTransfer: u8 = 49u8;
pub const THOST_FTDC_SYT_TheThirdPartStore: u8 = 50u8;
pub const THOST_FTDC_TEF_NormalProcessing: u8 = 48u8;
pub const THOST_FTDC_TEF_Success: u8 = 49u8;
pub const THOST_FTDC_TEF_Failed: u8 = 50u8;
pub const THOST_FTDC_TEF_Abnormal: u8 = 51u8;
pub const THOST_FTDC_TEF_ManualProcessedForException: u8 = 52u8;
pub const THOST_FTDC_TEF_CommuFailedNeedManualProcess: u8 = 53u8;
pub const THOST_FTDC_TEF_SysErrorNeedManualProcess: u8 = 54u8;
pub const THOST_FTDC_PSS_NotProcess: u8 = 48u8;
pub const THOST_FTDC_PSS_StartProcess: u8 = 49u8;
pub const THOST_FTDC_PSS_Finished: u8 = 50u8;
pub const THOST_FTDC_CUSTT_Person: u8 = 48u8;
pub const THOST_FTDC_CUSTT_Institution: u8 = 49u8;
pub const THOST_FTDC_FBTTD_FromBankToFuture: u8 = 49u8;
pub const THOST_FTDC_FBTTD_FromFutureToBank: u8 = 50u8;
pub const THOST_FTDC_OOD_Open: u8 = 49u8;
pub const THOST_FTDC_OOD_Destroy: u8 = 48u8;
pub const THOST_FTDC_AVAF_Invalid: u8 = 48u8;
pub const THOST_FTDC_AVAF_Valid: u8 = 49u8;
pub const THOST_FTDC_AVAF_Repeal: u8 = 50u8;
pub const THOST_FTDC_OT_Bank: u8 = 49u8;
pub const THOST_FTDC_OT_Future: u8 = 50u8;
pub const THOST_FTDC_OT_PlateForm: u8 = 57u8;
pub const THOST_FTDC_OL_HeadQuarters: u8 = 49u8;
pub const THOST_FTDC_OL_Branch: u8 = 50u8;
pub const THOST_FTDC_PID_FutureProtocal: u8 = 48u8;
pub const THOST_FTDC_PID_ICBCProtocal: u8 = 49u8;
pub const THOST_FTDC_PID_ABCProtocal: u8 = 50u8;
pub const THOST_FTDC_PID_CBCProtocal: u8 = 51u8;
pub const THOST_FTDC_PID_CCBProtocal: u8 = 52u8;
pub const THOST_FTDC_PID_BOCOMProtocal: u8 = 53u8;
pub const THOST_FTDC_PID_FBTPlateFormProtocal: u8 = 88u8;
pub const THOST_FTDC_CM_ShortConnect: u8 = 48u8;
pub const THOST_FTDC_CM_LongConnect: u8 = 49u8;
pub const THOST_FTDC_SRM_ASync: u8 = 48u8;
pub const THOST_FTDC_SRM_Sync: u8 = 49u8;
pub const THOST_FTDC_BAT_BankBook: u8 = 49u8;
pub const THOST_FTDC_BAT_SavingCard: u8 = 50u8;
pub const THOST_FTDC_BAT_CreditCard: u8 = 51u8;
pub const THOST_FTDC_FAT_BankBook: u8 = 49u8;
pub const THOST_FTDC_FAT_SavingCard: u8 = 50u8;
pub const THOST_FTDC_FAT_CreditCard: u8 = 51u8;
pub const THOST_FTDC_OS_Ready: u8 = 48u8;
pub const THOST_FTDC_OS_CheckIn: u8 = 49u8;
pub const THOST_FTDC_OS_CheckOut: u8 = 50u8;
pub const THOST_FTDC_OS_CheckFileArrived: u8 = 51u8;
pub const THOST_FTDC_OS_CheckDetail: u8 = 52u8;
pub const THOST_FTDC_OS_DayEndClean: u8 = 53u8;
pub const THOST_FTDC_OS_Invalid: u8 = 57u8;
pub const THOST_FTDC_CCBFM_ByAmount: u8 = 49u8;
pub const THOST_FTDC_CCBFM_ByMonth: u8 = 50u8;
pub const THOST_FTDC_CAPIT_Client: u8 = 49u8;
pub const THOST_FTDC_CAPIT_Server: u8 = 50u8;
pub const THOST_FTDC_CAPIT_UserApi: u8 = 51u8;
pub const THOST_FTDC_LS_Connected: u8 = 49u8;
pub const THOST_FTDC_LS_Disconnected: u8 = 50u8;
pub const THOST_FTDC_BPWDF_NoCheck: u8 = 48u8;
pub const THOST_FTDC_BPWDF_BlankCheck: u8 = 49u8;
pub const THOST_FTDC_BPWDF_EncryptCheck: u8 = 50u8;
pub const THOST_FTDC_SAT_AccountID: u8 = 49u8;
pub const THOST_FTDC_SAT_CardID: u8 = 50u8;
pub const THOST_FTDC_SAT_SHStockholderID: u8 = 51u8;
pub const THOST_FTDC_SAT_SZStockholderID: u8 = 52u8;
pub const THOST_FTDC_TRFS_Normal: u8 = 48u8;
pub const THOST_FTDC_TRFS_Repealed: u8 = 49u8;
pub const THOST_FTDC_SPTYPE_Broker: u8 = 48u8;
pub const THOST_FTDC_SPTYPE_Bank: u8 = 49u8;
pub const THOST_FTDC_REQRSP_Request: u8 = 48u8;
pub const THOST_FTDC_REQRSP_Response: u8 = 49u8;
pub const THOST_FTDC_FBTUET_SignIn: u8 = 48u8;
pub const THOST_FTDC_FBTUET_FromBankToFuture: u8 = 49u8;
pub const THOST_FTDC_FBTUET_FromFutureToBank: u8 = 50u8;
pub const THOST_FTDC_FBTUET_OpenAccount: u8 = 51u8;
pub const THOST_FTDC_FBTUET_CancelAccount: u8 = 52u8;
pub const THOST_FTDC_FBTUET_ChangeAccount: u8 = 53u8;
pub const THOST_FTDC_FBTUET_RepealFromBankToFuture: u8 = 54u8;
pub const THOST_FTDC_FBTUET_RepealFromFutureToBank: u8 = 55u8;
pub const THOST_FTDC_FBTUET_QueryBankAccount: u8 = 56u8;
pub const THOST_FTDC_FBTUET_QueryFutureAccount: u8 = 57u8;
pub const THOST_FTDC_FBTUET_SignOut: u8 = 65u8;
pub const THOST_FTDC_FBTUET_SyncKey: u8 = 66u8;
pub const THOST_FTDC_FBTUET_ReserveOpenAccount: u8 = 67u8;
pub const THOST_FTDC_FBTUET_CancelReserveOpenAccount: u8 = 68u8;
pub const THOST_FTDC_FBTUET_ReserveOpenAccountConfirm: u8 = 69u8;
pub const THOST_FTDC_FBTUET_Other: u8 = 90u8;
pub const THOST_FTDC_DBOP_Insert: u8 = 48u8;
pub const THOST_FTDC_DBOP_Update: u8 = 49u8;
pub const THOST_FTDC_DBOP_Delete: u8 = 50u8;
pub const THOST_FTDC_SYNF_Yes: u8 = 48u8;
pub const THOST_FTDC_SYNF_No: u8 = 49u8;
pub const THOST_FTDC_SYNT_OneOffSync: u8 = 48u8;
pub const THOST_FTDC_SYNT_TimerSync: u8 = 49u8;
pub const THOST_FTDC_SYNT_TimerFullSync: u8 = 50u8;
pub const THOST_FTDC_FBEDIR_Settlement: u8 = 48u8;
pub const THOST_FTDC_FBEDIR_Sale: u8 = 49u8;
pub const THOST_FTDC_FBERES_Success: u8 = 48u8;
pub const THOST_FTDC_FBERES_InsufficientBalance: u8 = 49u8;
pub const THOST_FTDC_FBERES_UnknownTrading: u8 = 56u8;
pub const THOST_FTDC_FBERES_Fail: u8 = 120u8;
pub const THOST_FTDC_FBEES_Normal: u8 = 48u8;
pub const THOST_FTDC_FBEES_ReExchange: u8 = 49u8;
pub const THOST_FTDC_FBEFG_DataPackage: u8 = 48u8;
pub const THOST_FTDC_FBEFG_File: u8 = 49u8;
pub const THOST_FTDC_FBEAT_NotTrade: u8 = 48u8;
pub const THOST_FTDC_FBEAT_Trade: u8 = 49u8;
pub const THOST_FTDC_FBEUET_SignIn: u8 = 48u8;
pub const THOST_FTDC_FBEUET_Exchange: u8 = 49u8;
pub const THOST_FTDC_FBEUET_ReExchange: u8 = 50u8;
pub const THOST_FTDC_FBEUET_QueryBankAccount: u8 = 51u8;
pub const THOST_FTDC_FBEUET_QueryExchDetial: u8 = 52u8;
pub const THOST_FTDC_FBEUET_QueryExchSummary: u8 = 53u8;
pub const THOST_FTDC_FBEUET_QueryExchRate: u8 = 54u8;
pub const THOST_FTDC_FBEUET_CheckBankAccount: u8 = 55u8;
pub const THOST_FTDC_FBEUET_SignOut: u8 = 56u8;
pub const THOST_FTDC_FBEUET_Other: u8 = 90u8;
pub const THOST_FTDC_FBERF_UnProcessed: u8 = 48u8;
pub const THOST_FTDC_FBERF_WaitSend: u8 = 49u8;
pub const THOST_FTDC_FBERF_SendSuccess: u8 = 50u8;
pub const THOST_FTDC_FBERF_SendFailed: u8 = 51u8;
pub const THOST_FTDC_FBERF_WaitReSend: u8 = 52u8;
pub const THOST_FTDC_NC_NOERROR: u8 = 48u8;
pub const THOST_FTDC_NC_Warn: u8 = 49u8;
pub const THOST_FTDC_NC_Call: u8 = 50u8;
pub const THOST_FTDC_NC_Force: u8 = 51u8;
pub const THOST_FTDC_NC_CHUANCANG: u8 = 52u8;
pub const THOST_FTDC_NC_Exception: u8 = 53u8;
pub const THOST_FTDC_FCT_Manual: u8 = 48u8;
pub const THOST_FTDC_FCT_Single: u8 = 49u8;
pub const THOST_FTDC_FCT_Group: u8 = 50u8;
pub const THOST_FTDC_RNM_System: u8 = 48u8;
pub const THOST_FTDC_RNM_SMS: u8 = 49u8;
pub const THOST_FTDC_RNM_EMail: u8 = 50u8;
pub const THOST_FTDC_RNM_Manual: u8 = 51u8;
pub const THOST_FTDC_RNS_NotGen: u8 = 48u8;
pub const THOST_FTDC_RNS_Generated: u8 = 49u8;
pub const THOST_FTDC_RNS_SendError: u8 = 50u8;
pub const THOST_FTDC_RNS_SendOk: u8 = 51u8;
pub const THOST_FTDC_RNS_Received: u8 = 52u8;
pub const THOST_FTDC_RNS_Confirmed: u8 = 53u8;
pub const THOST_FTDC_RUE_ExportData: u8 = 48u8;
pub const THOST_FTDC_COST_LastPriceAsc: u8 = 48u8;
pub const THOST_FTDC_COST_LastPriceDesc: u8 = 49u8;
pub const THOST_FTDC_COST_AskPriceAsc: u8 = 50u8;
pub const THOST_FTDC_COST_AskPriceDesc: u8 = 51u8;
pub const THOST_FTDC_COST_BidPriceAsc: u8 = 52u8;
pub const THOST_FTDC_COST_BidPriceDesc: u8 = 53u8;
pub const THOST_FTDC_UOAST_NoSend: u8 = 48u8;
pub const THOST_FTDC_UOAST_Sended: u8 = 49u8;
pub const THOST_FTDC_UOAST_Generated: u8 = 50u8;
pub const THOST_FTDC_UOAST_SendFail: u8 = 51u8;
pub const THOST_FTDC_UOAST_Success: u8 = 52u8;
pub const THOST_FTDC_UOAST_Fail: u8 = 53u8;
pub const THOST_FTDC_UOAST_Cancel: u8 = 54u8;
pub const THOST_FTDC_UOACS_NoApply: u8 = 49u8;
pub const THOST_FTDC_UOACS_Submited: u8 = 50u8;
pub const THOST_FTDC_UOACS_Sended: u8 = 51u8;
pub const THOST_FTDC_UOACS_Success: u8 = 52u8;
pub const THOST_FTDC_UOACS_Refuse: u8 = 53u8;
pub const THOST_FTDC_UOACS_Cancel: u8 = 54u8;
pub const THOST_FTDC_QT_Radio: u8 = 49u8;
pub const THOST_FTDC_QT_Option: u8 = 50u8;
pub const THOST_FTDC_QT_Blank: u8 = 51u8;
pub const THOST_FTDC_BT_Request: u8 = 49u8;
pub const THOST_FTDC_BT_Response: u8 = 50u8;
pub const THOST_FTDC_BT_Notice: u8 = 51u8;
pub const THOST_FTDC_CRC_Success: u8 = 48u8;
pub const THOST_FTDC_CRC_Working: u8 = 49u8;
pub const THOST_FTDC_CRC_InfoFail: u8 = 50u8;
pub const THOST_FTDC_CRC_IDCardFail: u8 = 51u8;
pub const THOST_FTDC_CRC_OtherFail: u8 = 52u8;
pub const THOST_FTDC_CfMMCCT_All: u8 = 48u8;
pub const THOST_FTDC_CfMMCCT_Person: u8 = 49u8;
pub const THOST_FTDC_CfMMCCT_Company: u8 = 50u8;
pub const THOST_FTDC_CfMMCCT_Other: u8 = 51u8;
pub const THOST_FTDC_CfMMCCT_SpecialOrgan: u8 = 52u8;
pub const THOST_FTDC_CfMMCCT_Asset: u8 = 53u8;
pub const THOST_FTDC_EIDT_SHFE: u8 = 83u8;
pub const THOST_FTDC_EIDT_CZCE: u8 = 90u8;
pub const THOST_FTDC_EIDT_DCE: u8 = 68u8;
pub const THOST_FTDC_EIDT_CFFEX: u8 = 74u8;
pub const THOST_FTDC_EIDT_INE: u8 = 78u8;
pub const THOST_FTDC_ECIDT_Hedge: u8 = 49u8;
pub const THOST_FTDC_ECIDT_Arbitrage: u8 = 50u8;
pub const THOST_FTDC_ECIDT_Speculation: u8 = 51u8;
pub const THOST_FTDC_UF_NoUpdate: u8 = 48u8;
pub const THOST_FTDC_UF_Success: u8 = 49u8;
pub const THOST_FTDC_UF_Fail: u8 = 50u8;
pub const THOST_FTDC_UF_TCSuccess: u8 = 51u8;
pub const THOST_FTDC_UF_TCFail: u8 = 52u8;
pub const THOST_FTDC_UF_Cancel: u8 = 53u8;
pub const THOST_FTDC_AOID_OpenInvestor: u8 = 49u8;
pub const THOST_FTDC_AOID_ModifyIDCard: u8 = 50u8;
pub const THOST_FTDC_AOID_ModifyNoIDCard: u8 = 51u8;
pub const THOST_FTDC_AOID_ApplyTradingCode: u8 = 52u8;
pub const THOST_FTDC_AOID_CancelTradingCode: u8 = 53u8;
pub const THOST_FTDC_AOID_CancelInvestor: u8 = 54u8;
pub const THOST_FTDC_AOID_FreezeAccount: u8 = 56u8;
pub const THOST_FTDC_AOID_ActiveFreezeAccount: u8 = 57u8;
pub const THOST_FTDC_ASID_NoComplete: u8 = 49u8;
pub const THOST_FTDC_ASID_Submited: u8 = 50u8;
pub const THOST_FTDC_ASID_Checked: u8 = 51u8;
pub const THOST_FTDC_ASID_Refused: u8 = 52u8;
pub const THOST_FTDC_ASID_Deleted: u8 = 53u8;
pub const THOST_FTDC_UOASM_ByAPI: u8 = 49u8;
pub const THOST_FTDC_UOASM_ByFile: u8 = 50u8;
pub const THOST_FTDC_EvM_ADD: u8 = 49u8;
pub const THOST_FTDC_EvM_UPDATE: u8 = 50u8;
pub const THOST_FTDC_EvM_DELETE: u8 = 51u8;
pub const THOST_FTDC_EvM_CHECK: u8 = 52u8;
pub const THOST_FTDC_EvM_COPY: u8 = 53u8;
pub const THOST_FTDC_EvM_CANCEL: u8 = 54u8;
pub const THOST_FTDC_EvM_Reverse: u8 = 55u8;
pub const THOST_FTDC_UOAA_ASR: u8 = 49u8;
pub const THOST_FTDC_UOAA_ASNR: u8 = 50u8;
pub const THOST_FTDC_UOAA_NSAR: u8 = 51u8;
pub const THOST_FTDC_UOAA_NSR: u8 = 52u8;
pub const THOST_FTDC_EvM_InvestorGroupFlow: u8 = 49u8;
pub const THOST_FTDC_EvM_InvestorRate: u8 = 50u8;
pub const THOST_FTDC_EvM_InvestorCommRateModel: u8 = 51u8;
pub const THOST_FTDC_CL_Zero: u8 = 48u8;
pub const THOST_FTDC_CL_One: u8 = 49u8;
pub const THOST_FTDC_CL_Two: u8 = 50u8;
pub const THOST_FTDC_CHS_Init: u8 = 48u8;
pub const THOST_FTDC_CHS_Checking: u8 = 49u8;
pub const THOST_FTDC_CHS_Checked: u8 = 50u8;
pub const THOST_FTDC_CHS_Refuse: u8 = 51u8;
pub const THOST_FTDC_CHS_Cancel: u8 = 52u8;
pub const THOST_FTDC_CHU_Unused: u8 = 48u8;
pub const THOST_FTDC_CHU_Used: u8 = 49u8;
pub const THOST_FTDC_CHU_Fail: u8 = 50u8;
pub const THOST_FTDC_BAO_ByAccProperty: u8 = 48u8;
pub const THOST_FTDC_BAO_ByFBTransfer: u8 = 49u8;
pub const THOST_FTDC_MBTS_ByInstrument: u8 = 48u8;
pub const THOST_FTDC_MBTS_ByDayInsPrc: u8 = 49u8;
pub const THOST_FTDC_MBTS_ByDayIns: u8 = 50u8;
pub const THOST_FTDC_OTP_NONE: u8 = 48u8;
pub const THOST_FTDC_OTP_TOTP: u8 = 49u8;
pub const THOST_FTDC_OTPS_Unused: u8 = 48u8;
pub const THOST_FTDC_OTPS_Used: u8 = 49u8;
pub const THOST_FTDC_OTPS_Disuse: u8 = 50u8;
pub const THOST_FTDC_BUT_Investor: u8 = 49u8;
pub const THOST_FTDC_BUT_BrokerUser: u8 = 50u8;
pub const THOST_FTDC_FUTT_Commodity: u8 = 49u8;
pub const THOST_FTDC_FUTT_Financial: u8 = 50u8;
pub const THOST_FTDC_FET_Restriction: u8 = 48u8;
pub const THOST_FTDC_FET_TodayRestriction: u8 = 49u8;
pub const THOST_FTDC_FET_Transfer: u8 = 50u8;
pub const THOST_FTDC_FET_Credit: u8 = 51u8;
pub const THOST_FTDC_FET_InvestorWithdrawAlm: u8 = 52u8;
pub const THOST_FTDC_FET_BankRestriction: u8 = 53u8;
pub const THOST_FTDC_FET_Accountregister: u8 = 54u8;
pub const THOST_FTDC_FET_ExchangeFundIO: u8 = 55u8;
pub const THOST_FTDC_FET_InvestorFundIO: u8 = 56u8;
pub const THOST_FTDC_AST_FBTransfer: u8 = 48u8;
pub const THOST_FTDC_AST_ManualEntry: u8 = 49u8;
pub const THOST_FTDC_CST_UnifyAccount: u8 = 48u8;
pub const THOST_FTDC_CST_ManualEntry: u8 = 49u8;
pub const THOST_FTDC_UR_All: u8 = 48u8;
pub const THOST_FTDC_UR_Single: u8 = 49u8;
pub const THOST_FTDC_BG_Investor: u8 = 50u8;
pub const THOST_FTDC_BG_Group: u8 = 49u8;
pub const THOST_FTDC_TSSM_Instrument: u8 = 49u8;
pub const THOST_FTDC_TSSM_Product: u8 = 50u8;
pub const THOST_FTDC_TSSM_Exchange: u8 = 51u8;
pub const THOST_FTDC_ESM_Relative: u8 = 49u8;
pub const THOST_FTDC_ESM_Typical: u8 = 50u8;
pub const THOST_FTDC_RIR_All: u8 = 49u8;
pub const THOST_FTDC_RIR_Model: u8 = 50u8;
pub const THOST_FTDC_RIR_Single: u8 = 51u8;
pub const THOST_FTDC_SDS_Initialize: u8 = 48u8;
pub const THOST_FTDC_SDS_Settlementing: u8 = 49u8;
pub const THOST_FTDC_SDS_Settlemented: u8 = 50u8;
pub const THOST_FTDC_TSRC_NORMAL: u8 = 48u8;
pub const THOST_FTDC_TSRC_QUERY: u8 = 49u8;
pub const THOST_FTDC_FSM_Product: u8 = 49u8;
pub const THOST_FTDC_FSM_Exchange: u8 = 50u8;
pub const THOST_FTDC_FSM_All: u8 = 51u8;
pub const THOST_FTDC_BIR_Property: u8 = 49u8;
pub const THOST_FTDC_BIR_All: u8 = 50u8;
pub const THOST_FTDC_PIR_All: u8 = 49u8;
pub const THOST_FTDC_PIR_Property: u8 = 50u8;
pub const THOST_FTDC_PIR_Single: u8 = 51u8;
pub const THOST_FTDC_FIS_NoCreate: u8 = 48u8;
pub const THOST_FTDC_FIS_Created: u8 = 49u8;
pub const THOST_FTDC_FIS_Failed: u8 = 50u8;
pub const THOST_FTDC_FGS_FileTransmit: u8 = 48u8;
pub const THOST_FTDC_FGS_FileGen: u8 = 49u8;
pub const THOST_FTDC_SoM_Add: u8 = 49u8;
pub const THOST_FTDC_SoM_Update: u8 = 50u8;
pub const THOST_FTDC_SoM_Delete: u8 = 51u8;
pub const THOST_FTDC_SoM_Copy: u8 = 52u8;
pub const THOST_FTDC_SoM_AcTive: u8 = 53u8;
pub const THOST_FTDC_SoM_CanCel: u8 = 54u8;
pub const THOST_FTDC_SoM_ReSet: u8 = 55u8;
pub const THOST_FTDC_SoT_UpdatePassword: u8 = 48u8;
pub const THOST_FTDC_SoT_UserDepartment: u8 = 49u8;
pub const THOST_FTDC_SoT_RoleManager: u8 = 50u8;
pub const THOST_FTDC_SoT_RoleFunction: u8 = 51u8;
pub const THOST_FTDC_SoT_BaseParam: u8 = 52u8;
pub const THOST_FTDC_SoT_SetUserID: u8 = 53u8;
pub const THOST_FTDC_SoT_SetUserRole: u8 = 54u8;
pub const THOST_FTDC_SoT_UserIpRestriction: u8 = 55u8;
pub const THOST_FTDC_SoT_DepartmentManager: u8 = 56u8;
pub const THOST_FTDC_SoT_DepartmentCopy: u8 = 57u8;
pub const THOST_FTDC_SoT_Tradingcode: u8 = 65u8;
pub const THOST_FTDC_SoT_InvestorStatus: u8 = 66u8;
pub const THOST_FTDC_SoT_InvestorAuthority: u8 = 67u8;
pub const THOST_FTDC_SoT_PropertySet: u8 = 68u8;
pub const THOST_FTDC_SoT_ReSetInvestorPasswd: u8 = 69u8;
pub const THOST_FTDC_SoT_InvestorPersonalityInfo: u8 = 70u8;
pub const THOST_FTDC_CSRCQ_Current: u8 = 48u8;
pub const THOST_FTDC_CSRCQ_History: u8 = 49u8;
pub const THOST_FTDC_FRS_Normal: u8 = 49u8;
pub const THOST_FTDC_FRS_Freeze: u8 = 48u8;
pub const THOST_FTDC_STST_Standard: u8 = 48u8;
pub const THOST_FTDC_STST_NonStandard: u8 = 49u8;
pub const THOST_FTDC_RPT_Freeze: u8 = 49u8;
pub const THOST_FTDC_RPT_FreezeActive: u8 = 50u8;
pub const THOST_FTDC_RPT_OpenLimit: u8 = 51u8;
pub const THOST_FTDC_RPT_RelieveOpenLimit: u8 = 52u8;
pub const THOST_FTDC_AMLDS_Normal: u8 = 48u8;
pub const THOST_FTDC_AMLDS_Deleted: u8 = 49u8;
pub const THOST_FTDC_AMLCHS_Init: u8 = 48u8;
pub const THOST_FTDC_AMLCHS_Checking: u8 = 49u8;
pub const THOST_FTDC_AMLCHS_Checked: u8 = 50u8;
pub const THOST_FTDC_AMLCHS_RefuseReport: u8 = 51u8;
pub const THOST_FTDC_AMLDT_DrawDay: u8 = 48u8;
pub const THOST_FTDC_AMLDT_TouchDay: u8 = 49u8;
pub const THOST_FTDC_AMLCL_CheckLevel0: u8 = 48u8;
pub const THOST_FTDC_AMLCL_CheckLevel1: u8 = 49u8;
pub const THOST_FTDC_AMLCL_CheckLevel2: u8 = 50u8;
pub const THOST_FTDC_AMLCL_CheckLevel3: u8 = 51u8;
pub const THOST_FTDC_EFT_CSV: u8 = 48u8;
pub const THOST_FTDC_EFT_EXCEL: u8 = 49u8;
pub const THOST_FTDC_EFT_DBF: u8 = 50u8;
pub const THOST_FTDC_SMT_Before: u8 = 49u8;
pub const THOST_FTDC_SMT_Settlement: u8 = 50u8;
pub const THOST_FTDC_SMT_After: u8 = 51u8;
pub const THOST_FTDC_SMT_Settlemented: u8 = 52u8;
pub const THOST_FTDC_SML_Must: u8 = 49u8;
pub const THOST_FTDC_SML_Alarm: u8 = 50u8;
pub const THOST_FTDC_SML_Prompt: u8 = 51u8;
pub const THOST_FTDC_SML_Ignore: u8 = 52u8;
pub const THOST_FTDC_SMG_Exhcange: u8 = 49u8;
pub const THOST_FTDC_SMG_ASP: u8 = 50u8;
pub const THOST_FTDC_SMG_CSRC: u8 = 51u8;
pub const THOST_FTDC_LUT_Repeatable: u8 = 49u8;
pub const THOST_FTDC_LUT_Unrepeatable: u8 = 50u8;
pub const THOST_FTDC_DAR_Settle: u8 = 49u8;
pub const THOST_FTDC_DAR_Exchange: u8 = 50u8;
pub const THOST_FTDC_DAR_CSRC: u8 = 51u8;
pub const THOST_FTDC_MGT_ExchMarginRate: u8 = 48u8;
pub const THOST_FTDC_MGT_InstrMarginRate: u8 = 49u8;
pub const THOST_FTDC_MGT_InstrMarginRateTrade: u8 = 50u8;
pub const THOST_FTDC_ACT_Intraday: u8 = 49u8;
pub const THOST_FTDC_ACT_Long: u8 = 50u8;
pub const THOST_FTDC_MRT_Exchange: u8 = 49u8;
pub const THOST_FTDC_MRT_Investor: u8 = 50u8;
pub const THOST_FTDC_MRT_InvestorTrade: u8 = 51u8;
pub const THOST_FTDC_BUS_UnBak: u8 = 48u8;
pub const THOST_FTDC_BUS_BakUp: u8 = 49u8;
pub const THOST_FTDC_BUS_BakUped: u8 = 50u8;
pub const THOST_FTDC_BUS_BakFail: u8 = 51u8;
pub const THOST_FTDC_SIS_UnInitialize: u8 = 48u8;
pub const THOST_FTDC_SIS_Initialize: u8 = 49u8;
pub const THOST_FTDC_SIS_Initialized: u8 = 50u8;
pub const THOST_FTDC_SRS_NoCreate: u8 = 48u8;
pub const THOST_FTDC_SRS_Create: u8 = 49u8;
pub const THOST_FTDC_SRS_Created: u8 = 50u8;
pub const THOST_FTDC_SRS_CreateFail: u8 = 51u8;
pub const THOST_FTDC_SSS_UnSaveData: u8 = 48u8;
pub const THOST_FTDC_SSS_SaveDatad: u8 = 49u8;
pub const THOST_FTDC_SAS_UnArchived: u8 = 48u8;
pub const THOST_FTDC_SAS_Archiving: u8 = 49u8;
pub const THOST_FTDC_SAS_Archived: u8 = 50u8;
pub const THOST_FTDC_SAS_ArchiveFail: u8 = 51u8;
pub const THOST_FTDC_CTPT_Unkown: u8 = 48u8;
pub const THOST_FTDC_CTPT_MainCenter: u8 = 49u8;
pub const THOST_FTDC_CTPT_BackUp: u8 = 50u8;
pub const THOST_FTDC_CDT_Normal: u8 = 48u8;
pub const THOST_FTDC_CDT_SpecFirst: u8 = 49u8;
pub const THOST_FTDC_MFUR_None: u8 = 48u8;
pub const THOST_FTDC_MFUR_Margin: u8 = 49u8;
pub const THOST_FTDC_MFUR_All: u8 = 50u8;
pub const THOST_FTDC_MFUR_CNY3: u8 = 51u8;
pub const THOST_FTDC_SPT_CzceHedge: u8 = 49u8;
pub const THOST_FTDC_SPT_IneForeignCurrency: u8 = 50u8;
pub const THOST_FTDC_SPT_DceOpenClose: u8 = 51u8;
pub const THOST_FTDC_FMT_Mortgage: u8 = 49u8;
pub const THOST_FTDC_FMT_Redemption: u8 = 50u8;
pub const THOST_FTDC_ASPI_BaseMargin: u8 = 49u8;
pub const THOST_FTDC_ASPI_LowestInterest: u8 = 50u8;
pub const THOST_FTDC_FMD_In: u8 = 49u8;
pub const THOST_FTDC_FMD_Out: u8 = 50u8;
pub const THOST_FTDC_BT_Profit: u8 = 48u8;
pub const THOST_FTDC_BT_Loss: u8 = 49u8;
pub const THOST_FTDC_BT_Other: u8 = 90u8;
pub const THOST_FTDC_SST_Manual: u8 = 48u8;
pub const THOST_FTDC_SST_Automatic: u8 = 49u8;
pub const THOST_FTDC_CED_Settlement: u8 = 48u8;
pub const THOST_FTDC_CED_Sale: u8 = 49u8;
pub const THOST_FTDC_CSS_Entry: u8 = 49u8;
pub const THOST_FTDC_CSS_Approve: u8 = 50u8;
pub const THOST_FTDC_CSS_Refuse: u8 = 51u8;
pub const THOST_FTDC_CSS_Revoke: u8 = 52u8;
pub const THOST_FTDC_CSS_Send: u8 = 53u8;
pub const THOST_FTDC_CSS_Success: u8 = 54u8;
pub const THOST_FTDC_CSS_Failure: u8 = 55u8;
pub const THOST_FTDC_REQF_NoSend: u8 = 48u8;
pub const THOST_FTDC_REQF_SendSuccess: u8 = 49u8;
pub const THOST_FTDC_REQF_SendFailed: u8 = 50u8;
pub const THOST_FTDC_REQF_WaitReSend: u8 = 51u8;
pub const THOST_FTDC_RESF_Success: u8 = 48u8;
pub const THOST_FTDC_RESF_InsuffiCient: u8 = 49u8;
pub const THOST_FTDC_RESF_UnKnown: u8 = 56u8;
pub const THOST_FTDC_EXS_Before: u8 = 48u8;
pub const THOST_FTDC_EXS_After: u8 = 49u8;
pub const THOST_FTDC_CR_Domestic: u8 = 49u8;
pub const THOST_FTDC_CR_GMT: u8 = 50u8;
pub const THOST_FTDC_CR_Foreign: u8 = 51u8;
pub const THOST_FTDC_HB_No: u8 = 48u8;
pub const THOST_FTDC_HB_Yes: u8 = 49u8;
pub const THOST_FTDC_SM_Normal: u8 = 49u8;
pub const THOST_FTDC_SM_Emerge: u8 = 50u8;
pub const THOST_FTDC_SM_Restore: u8 = 51u8;
pub const THOST_FTDC_TPT_Full: u8 = 49u8;
pub const THOST_FTDC_TPT_Increment: u8 = 50u8;
pub const THOST_FTDC_TPT_BackUp: u8 = 51u8;
pub const THOST_FTDC_LM_Trade: u8 = 48u8;
pub const THOST_FTDC_LM_Transfer: u8 = 49u8;
pub const THOST_FTDC_CPT_Instrument: u8 = 49u8;
pub const THOST_FTDC_CPT_Margin: u8 = 50u8;
pub const THOST_FTDC_HT_Yes: u8 = 49u8;
pub const THOST_FTDC_HT_No: u8 = 48u8;
pub const THOST_FTDC_AMT_Bank: u8 = 49u8;
pub const THOST_FTDC_AMT_Securities: u8 = 50u8;
pub const THOST_FTDC_AMT_Fund: u8 = 51u8;
pub const THOST_FTDC_AMT_Insurance: u8 = 52u8;
pub const THOST_FTDC_AMT_Trust: u8 = 53u8;
pub const THOST_FTDC_AMT_Other: u8 = 57u8;
pub const THOST_FTDC_CFIOT_FundIO: u8 = 48u8;
pub const THOST_FTDC_CFIOT_SwapCurrency: u8 = 49u8;
pub const THOST_FTDC_CAT_Futures: u8 = 49u8;
pub const THOST_FTDC_CAT_AssetmgrFuture: u8 = 50u8;
pub const THOST_FTDC_CAT_AssetmgrTrustee: u8 = 51u8;
pub const THOST_FTDC_CAT_AssetmgrTransfer: u8 = 52u8;
pub const THOST_FTDC_LT_Chinese: u8 = 49u8;
pub const THOST_FTDC_LT_English: u8 = 50u8;
pub const THOST_FTDC_AMCT_Person: u8 = 49u8;
pub const THOST_FTDC_AMCT_Organ: u8 = 50u8;
pub const THOST_FTDC_AMCT_SpecialOrgan: u8 = 52u8;
pub const THOST_FTDC_ASST_Futures: u8 = 51u8;
pub const THOST_FTDC_ASST_SpecialOrgan: u8 = 52u8;
pub const THOST_FTDC_CIT_HasExch: u8 = 48u8;
pub const THOST_FTDC_CIT_HasATP: u8 = 49u8;
pub const THOST_FTDC_CIT_HasDiff: u8 = 50u8;
pub const THOST_FTDC_DT_HandDeliv: u8 = 49u8;
pub const THOST_FTDC_DT_PersonDeliv: u8 = 50u8;
pub const THOST_FTDC_MMSA_NO: u8 = 48u8;
pub const THOST_FTDC_MMSA_YES: u8 = 49u8;
pub const THOST_FTDC_CACT_Person: u8 = 48u8;
pub const THOST_FTDC_CACT_Company: u8 = 49u8;
pub const THOST_FTDC_CACT_Other: u8 = 50u8;
pub const THOST_FTDC_UOAAT_Futures: u8 = 49u8;
pub const THOST_FTDC_UOAAT_SpecialOrgan: u8 = 50u8;
pub const THOST_FTDC_DEN_Buy: u8 = 48u8;
pub const THOST_FTDC_DEN_Sell: u8 = 49u8;
pub const THOST_FTDC_OFEN_Open: u8 = 48u8;
pub const THOST_FTDC_OFEN_Close: u8 = 49u8;
pub const THOST_FTDC_OFEN_ForceClose: u8 = 50u8;
pub const THOST_FTDC_OFEN_CloseToday: u8 = 51u8;
pub const THOST_FTDC_OFEN_CloseYesterday: u8 = 52u8;
pub const THOST_FTDC_OFEN_ForceOff: u8 = 53u8;
pub const THOST_FTDC_OFEN_LocalForceClose: u8 = 54u8;
pub const THOST_FTDC_HFEN_Speculation: u8 = 49u8;
pub const THOST_FTDC_HFEN_Arbitrage: u8 = 50u8;
pub const THOST_FTDC_HFEN_Hedge: u8 = 51u8;
pub const THOST_FTDC_FIOTEN_FundIO: u8 = 49u8;
pub const THOST_FTDC_FIOTEN_Transfer: u8 = 50u8;
pub const THOST_FTDC_FIOTEN_SwapCurrency: u8 = 51u8;
pub const THOST_FTDC_FTEN_Deposite: u8 = 49u8;
pub const THOST_FTDC_FTEN_ItemFund: u8 = 50u8;
pub const THOST_FTDC_FTEN_Company: u8 = 51u8;
pub const THOST_FTDC_FTEN_InnerTransfer: u8 = 52u8;
pub const THOST_FTDC_FDEN_In: u8 = 49u8;
pub const THOST_FTDC_FDEN_Out: u8 = 50u8;
pub const THOST_FTDC_FMDEN_In: u8 = 49u8;
pub const THOST_FTDC_FMDEN_Out: u8 = 50u8;
pub const THOST_FTDC_CP_CallOptions: u8 = 49u8;
pub const THOST_FTDC_CP_PutOptions: u8 = 50u8;
pub const THOST_FTDC_STM_Continental: u8 = 48u8;
pub const THOST_FTDC_STM_American: u8 = 49u8;
pub const THOST_FTDC_STM_Bermuda: u8 = 50u8;
pub const THOST_FTDC_STT_Hedge: u8 = 48u8;
pub const THOST_FTDC_STT_Match: u8 = 49u8;
pub const THOST_FTDC_APPT_NotStrikeNum: u8 = 52u8;
pub const THOST_FTDC_GUDS_Gen: u8 = 48u8;
pub const THOST_FTDC_GUDS_Hand: u8 = 49u8;
pub const THOST_FTDC_OER_NoExec: u8 = 110u8;
pub const THOST_FTDC_OER_Canceled: u8 = 99u8;
pub const THOST_FTDC_OER_OK: u8 = 48u8;
pub const THOST_FTDC_OER_NoPosition: u8 = 49u8;
pub const THOST_FTDC_OER_NoDeposit: u8 = 50u8;
pub const THOST_FTDC_OER_NoParticipant: u8 = 51u8;
pub const THOST_FTDC_OER_NoClient: u8 = 52u8;
pub const THOST_FTDC_OER_NoInstrument: u8 = 54u8;
pub const THOST_FTDC_OER_NoRight: u8 = 55u8;
pub const THOST_FTDC_OER_InvalidVolume: u8 = 56u8;
pub const THOST_FTDC_OER_NoEnoughHistoryTrade: u8 = 57u8;
pub const THOST_FTDC_OER_Unknown: u8 = 97u8;
pub const THOST_FTDC_COMBT_Future: u8 = 48u8;
pub const THOST_FTDC_COMBT_BUL: u8 = 49u8;
pub const THOST_FTDC_COMBT_BER: u8 = 50u8;
pub const THOST_FTDC_COMBT_STD: u8 = 51u8;
pub const THOST_FTDC_COMBT_STG: u8 = 52u8;
pub const THOST_FTDC_COMBT_PRT: u8 = 53u8;
pub const THOST_FTDC_COMBT_CLD: u8 = 54u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_COMBT_OPL: u8 = 55u8;
pub const THOST_FTDC_COMBT_BFO: u8 = 56u8;
}}
cfg_if::cfg_if! {if #[cfg(not(feature = "v6_3_13"))] {
pub const THOST_FTDC_DCECOMBT_SPL: u8 = 48u8;
pub const THOST_FTDC_DCECOMBT_OPL: u8 = 49u8;
pub const THOST_FTDC_DCECOMBT_SP: u8 = 50u8;
pub const THOST_FTDC_DCECOMBT_SPC: u8 = 51u8;
pub const THOST_FTDC_DCECOMBT_BLS: u8 = 52u8;
pub const THOST_FTDC_DCECOMBT_BES: u8 = 53u8;
pub const THOST_FTDC_DCECOMBT_CAS: u8 = 54u8;
pub const THOST_FTDC_DCECOMBT_STD: u8 = 55u8;
pub const THOST_FTDC_DCECOMBT_STG: u8 = 56u8;
pub const THOST_FTDC_DCECOMBT_BFO: u8 = 57u8;
pub const THOST_FTDC_DCECOMBT_SFO: u8 = 97u8;
}}
pub const THOST_FTDC_ORPT_PreSettlementPrice: u8 = 49u8;
pub const THOST_FTDC_ORPT_OpenPrice: u8 = 52u8;
pub const THOST_FTDC_ORPT_MaxPreSettlementPrice: u8 = 53u8;
pub const THOST_FTDC_BLAG_Default: u8 = 49u8;
pub const THOST_FTDC_BLAG_IncludeOptValLost: u8 = 50u8;
pub const THOST_FTDC_ACTP_Exec: u8 = 49u8;
pub const THOST_FTDC_ACTP_Abandon: u8 = 50u8;
pub const THOST_FTDC_FQST_Submitted: u8 = 97u8;
pub const THOST_FTDC_FQST_Accepted: u8 = 98u8;
pub const THOST_FTDC_FQST_Rejected: u8 = 99u8;
pub const THOST_FTDC_VM_Absolute: u8 = 48u8;
pub const THOST_FTDC_VM_Ratio: u8 = 49u8;
pub const THOST_FTDC_EOPF_Reserve: u8 = 48u8;
pub const THOST_FTDC_EOPF_UnReserve: u8 = 49u8;
pub const THOST_FTDC_EOCF_AutoClose: u8 = 48u8;
pub const THOST_FTDC_EOCF_NotToClose: u8 = 49u8;
pub const THOST_FTDC_PTE_Futures: u8 = 49u8;
pub const THOST_FTDC_PTE_Options: u8 = 50u8;
pub const THOST_FTDC_CUFN_CUFN_O: u8 = 79u8;
pub const THOST_FTDC_CUFN_CUFN_T: u8 = 84u8;
pub const THOST_FTDC_CUFN_CUFN_P: u8 = 80u8;
pub const THOST_FTDC_CUFN_CUFN_N: u8 = 78u8;
pub const THOST_FTDC_CUFN_CUFN_L: u8 = 76u8;
pub const THOST_FTDC_CUFN_CUFN_F: u8 = 70u8;
pub const THOST_FTDC_CUFN_CUFN_C: u8 = 67u8;
pub const THOST_FTDC_CUFN_CUFN_M: u8 = 77u8;
pub const THOST_FTDC_DUFN_DUFN_O: u8 = 79u8;
pub const THOST_FTDC_DUFN_DUFN_T: u8 = 84u8;
pub const THOST_FTDC_DUFN_DUFN_P: u8 = 80u8;
pub const THOST_FTDC_DUFN_DUFN_F: u8 = 70u8;
pub const THOST_FTDC_DUFN_DUFN_C: u8 = 67u8;
pub const THOST_FTDC_DUFN_DUFN_D: u8 = 68u8;
pub const THOST_FTDC_DUFN_DUFN_M: u8 = 77u8;
pub const THOST_FTDC_DUFN_DUFN_S: u8 = 83u8;
pub const THOST_FTDC_SUFN_SUFN_O: u8 = 79u8;
pub const THOST_FTDC_SUFN_SUFN_T: u8 = 84u8;
pub const THOST_FTDC_SUFN_SUFN_P: u8 = 80u8;
pub const THOST_FTDC_SUFN_SUFN_F: u8 = 70u8;
pub const THOST_FTDC_CFUFN_SUFN_T: u8 = 84u8;
pub const THOST_FTDC_CFUFN_SUFN_P: u8 = 80u8;
pub const THOST_FTDC_CFUFN_SUFN_F: u8 = 70u8;
pub const THOST_FTDC_CFUFN_SUFN_S: u8 = 83u8;
pub const THOST_FTDC_CMDR_Comb: u8 = 48u8;
pub const THOST_FTDC_CMDR_UnComb: u8 = 49u8;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub const THOST_FTDC_CMDR_DelComb: u8 = 50u8;
}}
pub const THOST_FTDC_STOV_RealValue: u8 = 49u8;
pub const THOST_FTDC_STOV_ProfitValue: u8 = 50u8;
pub const THOST_FTDC_STOV_RealRatio: u8 = 51u8;
pub const THOST_FTDC_STOV_ProfitRatio: u8 = 52u8;
pub const THOST_FTDC_ROAST_Processing: u8 = 48u8;
pub const THOST_FTDC_ROAST_Cancelled: u8 = 49u8;
pub const THOST_FTDC_ROAST_Opened: u8 = 50u8;
pub const THOST_FTDC_ROAST_Invalid: u8 = 51u8;
pub const THOST_FTDC_WPSR_Lib: u8 = 49u8;
pub const THOST_FTDC_WPSR_Manual: u8 = 50u8;
pub const THOST_FTDC_OSCF_CloseSelfOptionPosition: u8 = 49u8;
pub const THOST_FTDC_OSCF_ReserveOptionPosition: u8 = 50u8;
pub const THOST_FTDC_OSCF_SellCloseSelfFuturePosition: u8 = 51u8;
pub const THOST_FTDC_OSCF_ReserveFuturePosition: u8 = 52u8;
pub const THOST_FTDC_BZTP_Future: u8 = 49u8;
pub const THOST_FTDC_BZTP_Stock: u8 = 50u8;
pub const THOST_FTDC_APP_TYPE_Investor: u8 = 49u8;
pub const THOST_FTDC_APP_TYPE_InvestorRelay: u8 = 50u8;
pub const THOST_FTDC_APP_TYPE_OperatorRelay: u8 = 51u8;
pub const THOST_FTDC_APP_TYPE_UnKnown: u8 = 52u8;
pub const THOST_FTDC_RV_Right: u8 = 48u8;
pub const THOST_FTDC_RV_Refuse: u8 = 49u8;
pub const THOST_FTDC_OTC_TRDT_Block: u8 = 48u8;
pub const THOST_FTDC_OTC_TRDT_EFP: u8 = 49u8;
pub const THOST_FTDC_OTC_MT_DV01: u8 = 49u8;
pub const THOST_FTDC_OTC_MT_ParValue: u8 = 50u8;
pub const THOST_TE_RESUME_TYPE_THOST_TERT_RESTART: THOST_TE_RESUME_TYPE = 0;
pub const THOST_TE_RESUME_TYPE_THOST_TERT_RESUME: THOST_TE_RESUME_TYPE = 1;
pub const THOST_TE_RESUME_TYPE_THOST_TERT_QUICK: THOST_TE_RESUME_TYPE = 2;
pub type THOST_TE_RESUME_TYPE = u32;
#[doc = ""]
#[doc = "TFtdcTraderIDType是一个交易所交易员代码类型"]
#[doc = ""]
pub type TThostFtdcTraderIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcInvestorIDType是一个投资者代码类型"]
#[doc = ""]
pub type TThostFtdcInvestorIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcBrokerIDType是一个经纪公司代码类型"]
#[doc = ""]
pub type TThostFtdcBrokerIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcBrokerAbbrType是一个经纪公司简称类型"]
#[doc = ""]
pub type TThostFtdcBrokerAbbrType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcBrokerNameType是一个经纪公司名称类型"]
#[doc = ""]
pub type TThostFtdcBrokerNameType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcExchangeInstIDType是一个合约在交易所的代码类型"]
#[doc = ""]
pub type TThostFtdcExchangeInstIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcOrderRefType是一个报单引用类型"]
#[doc = ""]
pub type TThostFtdcOrderRefType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcParticipantIDType是一个会员代码类型"]
#[doc = ""]
pub type TThostFtdcParticipantIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcUserIDType是一个用户代码类型"]
#[doc = ""]
pub type TThostFtdcUserIDType = [u8; 16usize];
#[doc = ""]
#[doc = "TFtdcPasswordType是一个密码类型"]
#[doc = ""]
pub type TThostFtdcPasswordType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcClientIDType是一个交易编码类型"]
#[doc = ""]
pub type TThostFtdcClientIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcInstrumentIDType是一个合约代码类型"]
#[doc = ""]
pub type TThostFtdcInstrumentIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcInstrumentCodeType是一个合约标识码类型"]
#[doc = ""]
pub type TThostFtdcInstrumentCodeType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcMarketIDType是一个市场代码类型"]
#[doc = ""]
pub type TThostFtdcMarketIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcProductNameType是一个产品名称类型"]
#[doc = ""]
pub type TThostFtdcProductNameType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcExchangeIDType是一个交易所代码类型"]
#[doc = ""]
pub type TThostFtdcExchangeIDType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcExchangeNameType是一个交易所名称类型"]
#[doc = ""]
pub type TThostFtdcExchangeNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcExchangeAbbrType是一个交易所简称类型"]
#[doc = ""]
pub type TThostFtdcExchangeAbbrType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcExchangeFlagType是一个交易所标志类型"]
#[doc = ""]
pub type TThostFtdcExchangeFlagType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcMacAddressType是一个Mac地址类型"]
#[doc = ""]
pub type TThostFtdcMacAddressType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcSystemIDType是一个系统编号类型"]
#[doc = ""]
pub type TThostFtdcSystemIDType = [u8; 21usize];
pub type TThostFtdcExchangePropertyType = u8;
#[doc = ""]
#[doc = "TFtdcDateType是一个日期类型"]
#[doc = ""]
pub type TThostFtdcDateType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcTimeType是一个时间类型"]
#[doc = ""]
pub type TThostFtdcTimeType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcLongTimeType是一个长时间类型"]
#[doc = ""]
pub type TThostFtdcLongTimeType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcInstrumentNameType是一个合约名称类型"]
#[doc = ""]
pub type TThostFtdcInstrumentNameType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcSettlementGroupIDType是一个结算组代码类型"]
#[doc = ""]
pub type TThostFtdcSettlementGroupIDType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcOrderSysIDType是一个报单编号类型"]
#[doc = ""]
pub type TThostFtdcOrderSysIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcTradeIDType是一个成交编号类型"]
#[doc = ""]
pub type TThostFtdcTradeIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcCommandTypeType是一个DB命令类型类型"]
#[doc = ""]
pub type TThostFtdcCommandTypeType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcIPAddressType是一个IP地址类型"]
#[doc = ""]
pub type TThostFtdcIPAddressType = [u8; 16usize];
#[doc = ""]
#[doc = "TFtdcIPPortType是一个IP端口类型"]
#[doc = ""]
pub type TThostFtdcIPPortType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcProductInfoType是一个产品信息类型"]
#[doc = ""]
pub type TThostFtdcProductInfoType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcProtocolInfoType是一个协议信息类型"]
#[doc = ""]
pub type TThostFtdcProtocolInfoType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcBusinessUnitType是一个业务单元类型"]
#[doc = ""]
pub type TThostFtdcBusinessUnitType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcDepositSeqNoType是一个出入金流水号类型"]
#[doc = ""]
pub type TThostFtdcDepositSeqNoType = [u8; 15usize];
#[doc = ""]
#[doc = "TFtdcIdentifiedCardNoType是一个证件号码类型"]
#[doc = ""]
pub type TThostFtdcIdentifiedCardNoType = [u8; 51usize];
pub type TThostFtdcIdCardTypeType = u8;
#[doc = ""]
#[doc = "TFtdcOrderLocalIDType是一个本地报单编号类型"]
#[doc = ""]
pub type TThostFtdcOrderLocalIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcUserNameType是一个用户名称类型"]
#[doc = ""]
pub type TThostFtdcUserNameType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcPartyNameType是一个参与人名称类型"]
#[doc = ""]
pub type TThostFtdcPartyNameType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcErrorMsgType是一个错误信息类型"]
#[doc = ""]
pub type TThostFtdcErrorMsgType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcFieldNameType是一个字段名类型"]
#[doc = ""]
pub type TThostFtdcFieldNameType = [u8; 2049usize];
#[doc = ""]
#[doc = "TFtdcFieldContentType是一个字段内容类型"]
#[doc = ""]
pub type TThostFtdcFieldContentType = [u8; 2049usize];
#[doc = ""]
#[doc = "TFtdcSystemNameType是一个系统名称类型"]
#[doc = ""]
pub type TThostFtdcSystemNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcContentType是一个消息正文类型"]
#[doc = ""]
pub type TThostFtdcContentType = [u8; 501usize];
pub type TThostFtdcInvestorRangeType = InvestorRangeType;
pub type TThostFtdcDepartmentRangeType = u8;
pub type TThostFtdcDataSyncStatusType = u8;
pub type TThostFtdcBrokerDataSyncStatusType = u8;
pub type TThostFtdcExchangeConnectStatusType = u8;
pub type TThostFtdcTraderConnectStatusType = u8;
pub type TThostFtdcFunctionCodeType = u8;
pub type TThostFtdcBrokerFunctionCodeType = u8;
pub type TThostFtdcOrderActionStatusType = u8;
pub type TThostFtdcOrderStatusType = OrderStatus;
pub type TThostFtdcOrderSubmitStatusType = OrderSubmitStatus;
pub type TThostFtdcPositionDateType = PositionDate;
pub type TThostFtdcPositionDateTypeType = PositionDateType;
pub type TThostFtdcTradingRoleType = TradingRoleType;
pub type TThostFtdcProductClassType = ProductClass;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub type TThostFtdcAPIProductClassType = u8;
}}
pub type TThostFtdcInstLifePhaseType = InstLifePhase;
pub type TThostFtdcDirectionType = OrderDirection;
pub type TThostFtdcPositionTypeType = PositionType;
pub type TThostFtdcPosiDirectionType = PositionDirection;
pub type TThostFtdcSysSettlementStatusType = u8;
pub type TThostFtdcRatioAttrType = u8;
pub type TThostFtdcHedgeFlagType = OrderHedgeFlag;
pub type TThostFtdcBillHedgeFlagType = u8;
pub type TThostFtdcClientIDTypeType = u8;
pub type TThostFtdcOrderPriceTypeType = OrderPriceType;
pub type TThostFtdcOffsetFlagType = OrderOffsetFlag;
pub type TThostFtdcForceCloseReasonType = u8;
pub type TThostFtdcOrderTypeType = u8;
pub type TThostFtdcTimeConditionType = OrderTimeCondition;
pub type TThostFtdcVolumeConditionType = OrderVolumeCondition;
pub type TThostFtdcContingentConditionType = u8;
pub type TThostFtdcActionFlagType = u8;
pub type TThostFtdcTradingRightType = u8;
pub type TThostFtdcOrderSourceType = u8;
pub type TThostFtdcTradeTypeType = OrderTradeType;
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
pub type TThostFtdcSpecPosiTypeType = u8;
}}
pub type TThostFtdcPriceSourceType = OrderPriceSourceType;
pub type TThostFtdcInstrumentStatusType = InstrumentStatus;
pub type TThostFtdcInstStatusEnterReasonType = InstrumentStatusReason;
#[doc = ""]
#[doc = "TFtdcOrderActionRefType是一个报单操作引用类型"]
#[doc = ""]
pub type TThostFtdcOrderActionRefType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcInstallCountType是一个安装数量类型"]
#[doc = ""]
pub type TThostFtdcInstallCountType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcInstallIDType是一个安装编号类型"]
#[doc = ""]
pub type TThostFtdcInstallIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcErrorIDType是一个错误代码类型"]
#[doc = ""]
pub type TThostFtdcErrorIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcSettlementIDType是一个结算编号类型"]
#[doc = ""]
pub type TThostFtdcSettlementIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcVolumeType是一个数量类型"]
#[doc = ""]
pub type TThostFtdcVolumeType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcFrontIDType是一个前置编号类型"]
#[doc = ""]
pub type TThostFtdcFrontIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcSessionIDType是一个会话编号类型"]
#[doc = ""]
pub type TThostFtdcSessionIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcSequenceNoType是一个序号类型"]
#[doc = ""]
pub type TThostFtdcSequenceNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCommandNoType是一个DB命令序号类型"]
#[doc = ""]
pub type TThostFtdcCommandNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcMillisecType是一个时间（毫秒）类型"]
#[doc = ""]
pub type TThostFtdcMillisecType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcVolumeMultipleType是一个合约数量乘数类型"]
#[doc = ""]
pub type TThostFtdcVolumeMultipleType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTradingSegmentSNType是一个交易阶段编号类型"]
#[doc = ""]
pub type TThostFtdcTradingSegmentSNType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcRequestIDType是一个请求编号类型"]
#[doc = ""]
pub type TThostFtdcRequestIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcYearType是一个年份类型"]
#[doc = ""]
pub type TThostFtdcYearType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcMonthType是一个月份类型"]
#[doc = ""]
pub type TThostFtdcMonthType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcBoolType是一个布尔型类型"]
#[doc = ""]
pub type TThostFtdcBoolType = BoolType;
#[doc = ""]
#[doc = "TFtdcPriceType是一个价格类型"]
#[doc = ""]
pub type TThostFtdcPriceType = f64;
#[doc = ""]
#[doc = "TFtdcCombOffsetFlagType是一个组合开平标志类型"]
#[doc = ""]
pub type TThostFtdcCombOffsetFlagType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcCombHedgeFlagType是一个组合投机套保标志类型"]
#[doc = ""]
pub type TThostFtdcCombHedgeFlagType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcRatioType是一个比率类型"]
#[doc = ""]
pub type TThostFtdcRatioType = f64;
#[doc = ""]
#[doc = "TFtdcMoneyType是一个资金类型"]
#[doc = ""]
pub type TThostFtdcMoneyType = f64;
#[doc = ""]
#[doc = "TFtdcLargeVolumeType是一个大额数量类型"]
#[doc = ""]
pub type TThostFtdcLargeVolumeType = f64;
#[doc = ""]
#[doc = "TFtdcSequenceSeriesType是一个序列系列号类型"]
#[doc = ""]
pub type TThostFtdcSequenceSeriesType = ::std::os::raw::c_short;
#[doc = ""]
#[doc = "TFtdcCommPhaseNoType是一个通讯时段编号类型"]
#[doc = ""]
pub type TThostFtdcCommPhaseNoType = ::std::os::raw::c_short;
#[doc = ""]
#[doc = "TFtdcSequenceLabelType是一个序列编号类型"]
#[doc = ""]
pub type TThostFtdcSequenceLabelType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcUnderlyingMultipleType是一个基础商品乘数类型"]
#[doc = ""]
pub type TThostFtdcUnderlyingMultipleType = f64;
#[doc = ""]
#[doc = "TFtdcPriorityType是一个优先级类型"]
#[doc = ""]
pub type TThostFtdcPriorityType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcContractCodeType是一个合同编号类型"]
#[doc = ""]
pub type TThostFtdcContractCodeType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcCityType是一个市类型"]
#[doc = ""]
pub type TThostFtdcCityType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcIsStockType是一个是否股民类型"]
#[doc = ""]
pub type TThostFtdcIsStockType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcChannelType是一个渠道类型"]
#[doc = ""]
pub type TThostFtdcChannelType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcAddressType是一个通讯地址类型"]
#[doc = ""]
pub type TThostFtdcAddressType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcZipCodeType是一个邮政编码类型"]
#[doc = ""]
pub type TThostFtdcZipCodeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcTelephoneType是一个联系电话类型"]
#[doc = ""]
pub type TThostFtdcTelephoneType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcFaxType是一个传真类型"]
#[doc = ""]
pub type TThostFtdcFaxType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcMobileType是一个手机类型"]
#[doc = ""]
pub type TThostFtdcMobileType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcEMailType是一个电子邮件类型"]
#[doc = ""]
pub type TThostFtdcEMailType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcMemoType是一个备注类型"]
#[doc = ""]
pub type TThostFtdcMemoType = [u8; 161usize];
#[doc = ""]
#[doc = "TFtdcCompanyCodeType是一个企业代码类型"]
#[doc = ""]
pub type TThostFtdcCompanyCodeType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcWebsiteType是一个网站地址类型"]
#[doc = ""]
pub type TThostFtdcWebsiteType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcTaxNoType是一个税务登记号类型"]
#[doc = ""]
pub type TThostFtdcTaxNoType = [u8; 31usize];
pub type TThostFtdcBatchStatusType = u8;
#[doc = ""]
#[doc = "TFtdcPropertyIDType是一个属性代码类型"]
#[doc = ""]
pub type TThostFtdcPropertyIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcPropertyNameType是一个属性名称类型"]
#[doc = ""]
pub type TThostFtdcPropertyNameType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcLicenseNoType是一个营业执照号类型"]
#[doc = ""]
pub type TThostFtdcLicenseNoType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcAgentIDType是一个经纪人代码类型"]
#[doc = ""]
pub type TThostFtdcAgentIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcAgentNameType是一个经纪人名称类型"]
#[doc = ""]
pub type TThostFtdcAgentNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcAgentGroupIDType是一个经纪人组代码类型"]
#[doc = ""]
pub type TThostFtdcAgentGroupIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcAgentGroupNameType是一个经纪人组名称类型"]
#[doc = ""]
pub type TThostFtdcAgentGroupNameType = [u8; 41usize];
pub type TThostFtdcReturnStyleType = u8;
pub type TThostFtdcReturnPatternType = u8;
pub type TThostFtdcReturnLevelType = u8;
pub type TThostFtdcReturnStandardType = u8;
pub type TThostFtdcMortgageTypeType = u8;
pub type TThostFtdcInvestorSettlementParamIDType = u8;
pub type TThostFtdcExchangeSettlementParamIDType = u8;
pub type TThostFtdcSystemParamIDType = u8;
pub type TThostFtdcTradeParamIDType = u8;
#[doc = ""]
#[doc = "TFtdcSettlementParamValueType是一个参数代码值类型"]
#[doc = ""]
pub type TThostFtdcSettlementParamValueType = [u8; 256usize];
#[doc = ""]
#[doc = "TFtdcCounterIDType是一个计数器代码类型"]
#[doc = ""]
pub type TThostFtdcCounterIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcInvestorGroupNameType是一个投资者分组名称类型"]
#[doc = ""]
pub type TThostFtdcInvestorGroupNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcBrandCodeType是一个牌号类型"]
#[doc = ""]
pub type TThostFtdcBrandCodeType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcWarehouseType是一个仓库类型"]
#[doc = ""]
pub type TThostFtdcWarehouseType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcProductDateType是一个产期类型"]
#[doc = ""]
pub type TThostFtdcProductDateType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcGradeType是一个等级类型"]
#[doc = ""]
pub type TThostFtdcGradeType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcClassifyType是一个类别类型"]
#[doc = ""]
pub type TThostFtdcClassifyType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcPositionType是一个货位类型"]
#[doc = ""]
pub type TThostFtdcPositionType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcYieldlyType是一个产地类型"]
#[doc = ""]
pub type TThostFtdcYieldlyType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcWeightType是一个公定重量类型"]
#[doc = ""]
pub type TThostFtdcWeightType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcSubEntryFundNoType是一个分项资金流水号类型"]
#[doc = ""]
pub type TThostFtdcSubEntryFundNoType = ::std::os::raw::c_int;
pub type TThostFtdcFileIDType = u8;
#[doc = ""]
#[doc = "TFtdcFileNameType是一个文件名称类型"]
#[doc = ""]
pub type TThostFtdcFileNameType = [u8; 257usize];
pub type TThostFtdcFileTypeType = u8;
pub type TThostFtdcFileFormatType = u8;
pub type TThostFtdcFileUploadStatusType = u8;
pub type TThostFtdcTransferDirectionType = u8;
#[doc = ""]
#[doc = "TFtdcUploadModeType是一个上传文件类型类型"]
#[doc = ""]
pub type TThostFtdcUploadModeType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcAccountIDType是一个投资者帐号类型"]
#[doc = ""]
pub type TThostFtdcAccountIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcBankFlagType是一个银行统一标识类型类型"]
#[doc = ""]
pub type TThostFtdcBankFlagType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcBankAccountType是一个银行账户类型"]
#[doc = ""]
pub type TThostFtdcBankAccountType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcOpenNameType是一个银行账户的开户人名称类型"]
#[doc = ""]
pub type TThostFtdcOpenNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcOpenBankType是一个银行账户的开户行类型"]
#[doc = ""]
pub type TThostFtdcOpenBankType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcBankNameType是一个银行名称类型"]
#[doc = ""]
pub type TThostFtdcBankNameType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcPublishPathType是一个发布路径类型"]
#[doc = ""]
pub type TThostFtdcPublishPathType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcOperatorIDType是一个操作员代码类型"]
#[doc = ""]
pub type TThostFtdcOperatorIDType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcMonthCountType是一个月份数量类型"]
#[doc = ""]
pub type TThostFtdcMonthCountType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcAdvanceMonthArrayType是一个月份提前数组类型"]
#[doc = ""]
pub type TThostFtdcAdvanceMonthArrayType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcDateExprType是一个日期表达式类型"]
#[doc = ""]
pub type TThostFtdcDateExprType = [u8; 1025usize];
#[doc = ""]
#[doc = "TFtdcInstrumentIDExprType是一个合约代码表达式类型"]
#[doc = ""]
pub type TThostFtdcInstrumentIDExprType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcInstrumentNameExprType是一个合约名称表达式类型"]
#[doc = ""]
pub type TThostFtdcInstrumentNameExprType = [u8; 41usize];
pub type TThostFtdcSpecialCreateRuleType = u8;
pub type TThostFtdcBasisPriceTypeType = u8;
pub type TThostFtdcProductLifePhaseType = u8;
pub type TThostFtdcDeliveryModeType = u8;
#[doc = ""]
#[doc = "TFtdcLogLevelType是一个日志级别类型"]
#[doc = ""]
pub type TThostFtdcLogLevelType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcProcessNameType是一个存储过程名称类型"]
#[doc = ""]
pub type TThostFtdcProcessNameType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcOperationMemoType是一个操作摘要类型"]
#[doc = ""]
pub type TThostFtdcOperationMemoType = [u8; 1025usize];
pub type TThostFtdcFundIOTypeType = u8;
pub type TThostFtdcFundTypeType = u8;
pub type TThostFtdcFundDirectionType = u8;
pub type TThostFtdcFundStatusType = u8;
#[doc = ""]
#[doc = "TFtdcBillNoType是一个票据号类型"]
#[doc = ""]
pub type TThostFtdcBillNoType = [u8; 15usize];
#[doc = ""]
#[doc = "TFtdcBillNameType是一个票据名称类型"]
#[doc = ""]
pub type TThostFtdcBillNameType = [u8; 33usize];
pub type TThostFtdcPublishStatusType = u8;
#[doc = ""]
#[doc = "TFtdcEnumValueIDType是一个枚举值代码类型"]
#[doc = ""]
pub type TThostFtdcEnumValueIDType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcEnumValueTypeType是一个枚举值类型类型"]
#[doc = ""]
pub type TThostFtdcEnumValueTypeType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcEnumValueLabelType是一个枚举值名称类型"]
#[doc = ""]
pub type TThostFtdcEnumValueLabelType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcEnumValueResultType是一个枚举值结果类型"]
#[doc = ""]
pub type TThostFtdcEnumValueResultType = [u8; 33usize];
pub type TThostFtdcSystemStatusType = u8;
pub type TThostFtdcSettlementStatusType = u8;
#[doc = ""]
#[doc = "TFtdcRangeIntTypeType是一个限定值类型类型"]
#[doc = ""]
pub type TThostFtdcRangeIntTypeType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcRangeIntFromType是一个限定值下限类型"]
#[doc = ""]
pub type TThostFtdcRangeIntFromType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcRangeIntToType是一个限定值上限类型"]
#[doc = ""]
pub type TThostFtdcRangeIntToType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcFunctionIDType是一个功能代码类型"]
#[doc = ""]
pub type TThostFtdcFunctionIDType = [u8; 25usize];
#[doc = ""]
#[doc = "TFtdcFunctionValueCodeType是一个功能编码类型"]
#[doc = ""]
pub type TThostFtdcFunctionValueCodeType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcFunctionNameType是一个功能名称类型"]
#[doc = ""]
pub type TThostFtdcFunctionNameType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcRoleIDType是一个角色编号类型"]
#[doc = ""]
pub type TThostFtdcRoleIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcRoleNameType是一个角色名称类型"]
#[doc = ""]
pub type TThostFtdcRoleNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcDescriptionType是一个描述类型"]
#[doc = ""]
pub type TThostFtdcDescriptionType = [u8; 401usize];
#[doc = ""]
#[doc = "TFtdcCombineIDType是一个组合编号类型"]
#[doc = ""]
pub type TThostFtdcCombineIDType = [u8; 25usize];
#[doc = ""]
#[doc = "TFtdcCombineTypeType是一个组合类型类型"]
#[doc = ""]
pub type TThostFtdcCombineTypeType = [u8; 25usize];
pub type TThostFtdcInvestorTypeType = u8;
pub type TThostFtdcBrokerTypeType = u8;
pub type TThostFtdcRiskLevelType = u8;
pub type TThostFtdcFeeAcceptStyleType = u8;
pub type TThostFtdcPasswordTypeType = u8;
pub type TThostFtdcAlgorithmType = u8;
pub type TThostFtdcIncludeCloseProfitType = u8;
pub type TThostFtdcAllWithoutTradeType = u8;
#[doc = ""]
#[doc = "TFtdcCommentType是一个盈亏算法说明类型"]
#[doc = ""]
pub type TThostFtdcCommentType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcVersionType是一个版本号类型"]
#[doc = ""]
pub type TThostFtdcVersionType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcTradeCodeType是一个交易代码类型"]
#[doc = ""]
pub type TThostFtdcTradeCodeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcTradeDateType是一个交易日期类型"]
#[doc = ""]
pub type TThostFtdcTradeDateType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcTradeTimeType是一个交易时间类型"]
#[doc = ""]
pub type TThostFtdcTradeTimeType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcTradeSerialType是一个发起方流水号类型"]
#[doc = ""]
pub type TThostFtdcTradeSerialType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcTradeSerialNoType是一个发起方流水号类型"]
#[doc = ""]
pub type TThostFtdcTradeSerialNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcFutureIDType是一个期货公司代码类型"]
#[doc = ""]
pub type TThostFtdcFutureIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcBankIDType是一个银行代码类型"]
#[doc = ""]
pub type TThostFtdcBankIDType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcBankBrchIDType是一个银行分中心代码类型"]
#[doc = ""]
pub type TThostFtdcBankBrchIDType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcBankBranchIDType是一个分中心代码类型"]
#[doc = ""]
pub type TThostFtdcBankBranchIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcOperNoType是一个交易柜员类型"]
#[doc = ""]
pub type TThostFtdcOperNoType = [u8; 17usize];
#[doc = ""]
#[doc = "TFtdcDeviceIDType是一个渠道标志类型"]
#[doc = ""]
pub type TThostFtdcDeviceIDType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcRecordNumType是一个记录数类型"]
#[doc = ""]
pub type TThostFtdcRecordNumType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcFutureAccountType是一个期货资金账号类型"]
#[doc = ""]
pub type TThostFtdcFutureAccountType = [u8; 22usize];
pub type TThostFtdcFuturePwdFlagType = u8;
pub type TThostFtdcTransferTypeType = u8;
#[doc = ""]
#[doc = "TFtdcFutureAccPwdType是一个期货资金密码类型"]
#[doc = ""]
pub type TThostFtdcFutureAccPwdType = [u8; 17usize];
#[doc = ""]
#[doc = "TFtdcCurrencyCodeType是一个币种类型"]
#[doc = ""]
pub type TThostFtdcCurrencyCodeType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcRetCodeType是一个响应代码类型"]
#[doc = ""]
pub type TThostFtdcRetCodeType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcRetInfoType是一个响应信息类型"]
#[doc = ""]
pub type TThostFtdcRetInfoType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcTradeAmtType是一个银行总余额类型"]
#[doc = ""]
pub type TThostFtdcTradeAmtType = [u8; 20usize];
#[doc = ""]
#[doc = "TFtdcUseAmtType是一个银行可用余额类型"]
#[doc = ""]
pub type TThostFtdcUseAmtType = [u8; 20usize];
#[doc = ""]
#[doc = "TFtdcFetchAmtType是一个银行可取余额类型"]
#[doc = ""]
pub type TThostFtdcFetchAmtType = [u8; 20usize];
pub type TThostFtdcTransferValidFlagType = u8;
#[doc = ""]
#[doc = "TFtdcCertCodeType是一个证件号码类型"]
#[doc = ""]
pub type TThostFtdcCertCodeType = [u8; 21usize];
pub type TThostFtdcReasonType = u8;
#[doc = ""]
#[doc = "TFtdcFundProjectIDType是一个资金项目编号类型"]
#[doc = ""]
pub type TThostFtdcFundProjectIDType = [u8; 5usize];
pub type TThostFtdcSexType = u8;
#[doc = ""]
#[doc = "TFtdcProfessionType是一个职业类型"]
#[doc = ""]
pub type TThostFtdcProfessionType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcNationalType是一个国籍类型"]
#[doc = ""]
pub type TThostFtdcNationalType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcProvinceType是一个省类型"]
#[doc = ""]
pub type TThostFtdcProvinceType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcRegionType是一个区类型"]
#[doc = ""]
pub type TThostFtdcRegionType = [u8; 16usize];
#[doc = ""]
#[doc = "TFtdcCountryType是一个国家类型"]
#[doc = ""]
pub type TThostFtdcCountryType = [u8; 16usize];
#[doc = ""]
#[doc = "TFtdcLicenseNOType是一个营业执照类型"]
#[doc = ""]
pub type TThostFtdcLicenseNOType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcCompanyTypeType是一个企业性质类型"]
#[doc = ""]
pub type TThostFtdcCompanyTypeType = [u8; 16usize];
#[doc = ""]
#[doc = "TFtdcBusinessScopeType是一个经营范围类型"]
#[doc = ""]
pub type TThostFtdcBusinessScopeType = [u8; 1001usize];
#[doc = ""]
#[doc = "TFtdcCapitalCurrencyType是一个注册资本币种类型"]
#[doc = ""]
pub type TThostFtdcCapitalCurrencyType = [u8; 4usize];
pub type TThostFtdcUserTypeType = u8;
#[doc = ""]
#[doc = "TFtdcBranchIDType是一个营业部编号类型"]
#[doc = ""]
pub type TThostFtdcBranchIDType = [u8; 9usize];
pub type TThostFtdcRateTypeType = u8;
pub type TThostFtdcNoteTypeType = u8;
pub type TThostFtdcSettlementStyleType = u8;
#[doc = ""]
#[doc = "TFtdcBrokerDNSType是一个域名类型"]
#[doc = ""]
pub type TThostFtdcBrokerDNSType = [u8; 256usize];
#[doc = ""]
#[doc = "TFtdcSentenceType是一个语句类型"]
#[doc = ""]
pub type TThostFtdcSentenceType = [u8; 501usize];
pub type TThostFtdcSettlementBillTypeType = u8;
pub type TThostFtdcUserRightTypeType = u8;
pub type TThostFtdcMarginPriceTypeType = u8;
pub type TThostFtdcBillGenStatusType = u8;
pub type TThostFtdcAlgoTypeType = u8;
pub type TThostFtdcHandlePositionAlgoIDType = u8;
pub type TThostFtdcFindMarginRateAlgoIDType = u8;
pub type TThostFtdcHandleTradingAccountAlgoIDType = u8;
pub type TThostFtdcPersonTypeType = u8;
pub type TThostFtdcQueryInvestorRangeType = u8;
pub type TThostFtdcInvestorRiskStatusType = u8;
#[doc = ""]
#[doc = "TFtdcLegIDType是一个单腿编号类型"]
#[doc = ""]
pub type TThostFtdcLegIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcLegMultipleType是一个单腿乘数类型"]
#[doc = ""]
pub type TThostFtdcLegMultipleType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcImplyLevelType是一个派生层数类型"]
#[doc = ""]
pub type TThostFtdcImplyLevelType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcClearAccountType是一个结算账户类型"]
#[doc = ""]
pub type TThostFtdcClearAccountType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcOrganNOType是一个结算账户类型"]
#[doc = ""]
pub type TThostFtdcOrganNOType = [u8; 6usize];
#[doc = ""]
#[doc = "TFtdcClearbarchIDType是一个结算账户联行号类型"]
#[doc = ""]
pub type TThostFtdcClearbarchIDType = [u8; 6usize];
pub type TThostFtdcUserEventTypeType = u8;
#[doc = ""]
#[doc = "TFtdcUserEventInfoType是一个用户事件信息类型"]
#[doc = ""]
pub type TThostFtdcUserEventInfoType = [u8; 1025usize];
pub type TThostFtdcCloseStyleType = u8;
pub type TThostFtdcStatModeType = u8;
pub type TThostFtdcParkedOrderStatusType = u8;
#[doc = ""]
#[doc = "TFtdcParkedOrderIDType是一个预埋报单编号类型"]
#[doc = ""]
pub type TThostFtdcParkedOrderIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcParkedOrderActionIDType是一个预埋撤单编号类型"]
#[doc = ""]
pub type TThostFtdcParkedOrderActionIDType = [u8; 13usize];
pub type TThostFtdcVirDealStatusType = u8;
pub type TThostFtdcOrgSystemIDType = u8;
pub type TThostFtdcVirTradeStatusType = u8;
pub type TThostFtdcVirBankAccTypeType = u8;
pub type TThostFtdcVirementStatusType = u8;
pub type TThostFtdcVirementAvailAbilityType = u8;
pub type TThostFtdcVirementTradeCodeType = u8;
#[doc = ""]
#[doc = "TFtdcPhotoTypeNameType是一个影像类型名称类型"]
#[doc = ""]
pub type TThostFtdcPhotoTypeNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcSysVersionType是一个系统版本类型"]
#[doc = ""]
pub type TThostFtdcSysVersionType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcPhotoTypeIDType是一个影像类型代码类型"]
#[doc = ""]
pub type TThostFtdcPhotoTypeIDType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcPhotoNameType是一个影像名称类型"]
#[doc = ""]
pub type TThostFtdcPhotoNameType = [u8; 161usize];
#[doc = ""]
#[doc = "TFtdcTopicIDType是一个主题代码类型"]
#[doc = ""]
pub type TThostFtdcTopicIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcReportTypeIDType是一个交易报告类型标识类型"]
#[doc = ""]
pub type TThostFtdcReportTypeIDType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcCharacterIDType是一个交易特征代码类型"]
#[doc = ""]
pub type TThostFtdcCharacterIDType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcAMLParamIDType是一个参数代码类型"]
#[doc = ""]
pub type TThostFtdcAMLParamIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcAMLInvestorTypeType是一个投资者类型类型"]
#[doc = ""]
pub type TThostFtdcAMLInvestorTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLIdCardTypeType是一个证件类型类型"]
#[doc = ""]
pub type TThostFtdcAMLIdCardTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLTradeDirectType是一个资金进出方向类型"]
#[doc = ""]
pub type TThostFtdcAMLTradeDirectType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLTradeModelType是一个资金进出方式类型"]
#[doc = ""]
pub type TThostFtdcAMLTradeModelType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLOpParamValueType是一个业务参数代码值类型"]
#[doc = ""]
pub type TThostFtdcAMLOpParamValueType = f64;
#[doc = ""]
#[doc = "TFtdcAMLCustomerCardTypeType是一个客户身份证件/证明文件类型类型"]
#[doc = ""]
pub type TThostFtdcAMLCustomerCardTypeType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcAMLInstitutionNameType是一个金融机构网点名称类型"]
#[doc = ""]
pub type TThostFtdcAMLInstitutionNameType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcAMLDistrictIDType是一个金融机构网点所在地区行政区划代码类型"]
#[doc = ""]
pub type TThostFtdcAMLDistrictIDType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcAMLRelationShipType是一个金融机构网点与大额交易的关系类型"]
#[doc = ""]
pub type TThostFtdcAMLRelationShipType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLInstitutionTypeType是一个金融机构网点代码类型类型"]
#[doc = ""]
pub type TThostFtdcAMLInstitutionTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLInstitutionIDType是一个金融机构网点代码类型"]
#[doc = ""]
pub type TThostFtdcAMLInstitutionIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcAMLAccountTypeType是一个账户类型类型"]
#[doc = ""]
pub type TThostFtdcAMLAccountTypeType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcAMLTradingTypeType是一个交易方式类型"]
#[doc = ""]
pub type TThostFtdcAMLTradingTypeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcAMLTransactClassType是一个涉外收支交易分类与代码类型"]
#[doc = ""]
pub type TThostFtdcAMLTransactClassType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcAMLCapitalIOType是一个资金收付标识类型"]
#[doc = ""]
pub type TThostFtdcAMLCapitalIOType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcAMLSiteType是一个交易地点类型"]
#[doc = ""]
pub type TThostFtdcAMLSiteType = [u8; 10usize];
#[doc = ""]
#[doc = "TFtdcAMLCapitalPurposeType是一个资金用途类型"]
#[doc = ""]
pub type TThostFtdcAMLCapitalPurposeType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcAMLReportTypeType是一个报文类型类型"]
#[doc = ""]
pub type TThostFtdcAMLReportTypeType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcAMLSerialNoType是一个编号类型"]
#[doc = ""]
pub type TThostFtdcAMLSerialNoType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcAMLStatusType是一个状态类型"]
#[doc = ""]
pub type TThostFtdcAMLStatusType = [u8; 2usize];
pub type TThostFtdcAMLGenStatusType = u8;
#[doc = ""]
#[doc = "TFtdcAMLSeqCodeType是一个业务标识号类型"]
#[doc = ""]
pub type TThostFtdcAMLSeqCodeType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcAMLFileNameType是一个AML文件名类型"]
#[doc = ""]
pub type TThostFtdcAMLFileNameType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcAMLMoneyType是一个反洗钱资金类型"]
#[doc = ""]
pub type TThostFtdcAMLMoneyType = f64;
#[doc = ""]
#[doc = "TFtdcAMLFileAmountType是一个反洗钱资金类型"]
#[doc = ""]
pub type TThostFtdcAMLFileAmountType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCFMMCKeyType是一个密钥类型(保证金监管)类型"]
#[doc = ""]
pub type TThostFtdcCFMMCKeyType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcCFMMCTokenType是一个令牌类型(保证金监管)类型"]
#[doc = ""]
pub type TThostFtdcCFMMCTokenType = [u8; 21usize];
pub type TThostFtdcCFMMCKeyKindType = u8;
#[doc = ""]
#[doc = "TFtdcAMLReportNameType是一个报文名称类型"]
#[doc = ""]
pub type TThostFtdcAMLReportNameType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcIndividualNameType是一个个人姓名类型"]
#[doc = ""]
pub type TThostFtdcIndividualNameType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcCurrencyIDType是一个币种代码类型"]
#[doc = ""]
pub type TThostFtdcCurrencyIDType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcCustNumberType是一个客户编号类型"]
#[doc = ""]
pub type TThostFtdcCustNumberType = [u8; 36usize];
#[doc = ""]
#[doc = "TFtdcOrganCodeType是一个机构编码类型"]
#[doc = ""]
pub type TThostFtdcOrganCodeType = [u8; 36usize];
#[doc = ""]
#[doc = "TFtdcOrganNameType是一个机构名称类型"]
#[doc = ""]
pub type TThostFtdcOrganNameType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcSuperOrganCodeType是一个上级机构编码,即期货公司总部、银行总行类型"]
#[doc = ""]
pub type TThostFtdcSuperOrganCodeType = [u8; 12usize];
#[doc = ""]
#[doc = "TFtdcSubBranchIDType是一个分支机构类型"]
#[doc = ""]
pub type TThostFtdcSubBranchIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcSubBranchNameType是一个分支机构名称类型"]
#[doc = ""]
pub type TThostFtdcSubBranchNameType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcBranchNetCodeType是一个机构网点号类型"]
#[doc = ""]
pub type TThostFtdcBranchNetCodeType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcBranchNetNameType是一个机构网点名称类型"]
#[doc = ""]
pub type TThostFtdcBranchNetNameType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcOrganFlagType是一个机构标识类型"]
#[doc = ""]
pub type TThostFtdcOrganFlagType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcBankCodingForFutureType是一个银行对期货公司的编码类型"]
#[doc = ""]
pub type TThostFtdcBankCodingForFutureType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcBankReturnCodeType是一个银行对返回码的定义类型"]
#[doc = ""]
pub type TThostFtdcBankReturnCodeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcPlateReturnCodeType是一个银期转帐平台对返回码的定义类型"]
#[doc = ""]
pub type TThostFtdcPlateReturnCodeType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcBankSubBranchIDType是一个银行分支机构编码类型"]
#[doc = ""]
pub type TThostFtdcBankSubBranchIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcFutureBranchIDType是一个期货分支机构编码类型"]
#[doc = ""]
pub type TThostFtdcFutureBranchIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcReturnCodeType是一个返回代码类型"]
#[doc = ""]
pub type TThostFtdcReturnCodeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcOperatorCodeType是一个操作员类型"]
#[doc = ""]
pub type TThostFtdcOperatorCodeType = [u8; 17usize];
#[doc = ""]
#[doc = "TFtdcClearDepIDType是一个机构结算帐户机构号类型"]
#[doc = ""]
pub type TThostFtdcClearDepIDType = [u8; 6usize];
#[doc = ""]
#[doc = "TFtdcClearBrchIDType是一个机构结算帐户联行号类型"]
#[doc = ""]
pub type TThostFtdcClearBrchIDType = [u8; 6usize];
#[doc = ""]
#[doc = "TFtdcClearNameType是一个机构结算帐户名称类型"]
#[doc = ""]
pub type TThostFtdcClearNameType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcBankAccountNameType是一个银行帐户名称类型"]
#[doc = ""]
pub type TThostFtdcBankAccountNameType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcInvDepIDType是一个机构投资人账号机构号类型"]
#[doc = ""]
pub type TThostFtdcInvDepIDType = [u8; 6usize];
#[doc = ""]
#[doc = "TFtdcInvBrchIDType是一个机构投资人联行号类型"]
#[doc = ""]
pub type TThostFtdcInvBrchIDType = [u8; 6usize];
#[doc = ""]
#[doc = "TFtdcMessageFormatVersionType是一个信息格式版本类型"]
#[doc = ""]
pub type TThostFtdcMessageFormatVersionType = [u8; 36usize];
#[doc = ""]
#[doc = "TFtdcDigestType是一个摘要类型"]
#[doc = ""]
pub type TThostFtdcDigestType = [u8; 36usize];
#[doc = ""]
#[doc = "TFtdcAuthenticDataType是一个认证数据类型"]
#[doc = ""]
pub type TThostFtdcAuthenticDataType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcPasswordKeyType是一个密钥类型"]
#[doc = ""]
pub type TThostFtdcPasswordKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcFutureAccountNameType是一个期货帐户名称类型"]
#[doc = ""]
pub type TThostFtdcFutureAccountNameType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcMobilePhoneType是一个手机类型"]
#[doc = ""]
pub type TThostFtdcMobilePhoneType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcFutureMainKeyType是一个期货公司主密钥类型"]
#[doc = ""]
pub type TThostFtdcFutureMainKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcFutureWorkKeyType是一个期货公司工作密钥类型"]
#[doc = ""]
pub type TThostFtdcFutureWorkKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcFutureTransKeyType是一个期货公司传输密钥类型"]
#[doc = ""]
pub type TThostFtdcFutureTransKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcBankMainKeyType是一个银行主密钥类型"]
#[doc = ""]
pub type TThostFtdcBankMainKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcBankWorkKeyType是一个银行工作密钥类型"]
#[doc = ""]
pub type TThostFtdcBankWorkKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcBankTransKeyType是一个银行传输密钥类型"]
#[doc = ""]
pub type TThostFtdcBankTransKeyType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcBankServerDescriptionType是一个银行服务器描述信息类型"]
#[doc = ""]
pub type TThostFtdcBankServerDescriptionType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcAddInfoType是一个附加信息类型"]
#[doc = ""]
pub type TThostFtdcAddInfoType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcDescrInfoForReturnCodeType是一个返回码描述类型"]
#[doc = ""]
pub type TThostFtdcDescrInfoForReturnCodeType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcCountryCodeType是一个国家代码类型"]
#[doc = ""]
pub type TThostFtdcCountryCodeType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcSerialType是一个流水号类型"]
#[doc = ""]
pub type TThostFtdcSerialType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcPlateSerialType是一个平台流水号类型"]
#[doc = ""]
pub type TThostFtdcPlateSerialType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcBankSerialType是一个银行流水号类型"]
#[doc = ""]
pub type TThostFtdcBankSerialType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcCorrectSerialType是一个被冲正交易流水号类型"]
#[doc = ""]
pub type TThostFtdcCorrectSerialType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcFutureSerialType是一个期货公司流水号类型"]
#[doc = ""]
pub type TThostFtdcFutureSerialType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcApplicationIDType是一个应用标识类型"]
#[doc = ""]
pub type TThostFtdcApplicationIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcBankProxyIDType是一个银行代理标识类型"]
#[doc = ""]
pub type TThostFtdcBankProxyIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcFBTCoreIDType是一个银期转帐核心系统标识类型"]
#[doc = ""]
pub type TThostFtdcFBTCoreIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcServerPortType是一个服务端口号类型"]
#[doc = ""]
pub type TThostFtdcServerPortType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcRepealedTimesType是一个已经冲正次数类型"]
#[doc = ""]
pub type TThostFtdcRepealedTimesType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcRepealTimeIntervalType是一个冲正时间间隔类型"]
#[doc = ""]
pub type TThostFtdcRepealTimeIntervalType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTotalTimesType是一个每日累计转帐次数类型"]
#[doc = ""]
pub type TThostFtdcTotalTimesType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcFBTRequestIDType是一个请求ID类型"]
#[doc = ""]
pub type TThostFtdcFBTRequestIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTIDType是一个交易ID类型"]
#[doc = ""]
pub type TThostFtdcTIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTradeAmountType是一个交易金额（元）类型"]
#[doc = ""]
pub type TThostFtdcTradeAmountType = f64;
#[doc = ""]
#[doc = "TFtdcCustFeeType是一个应收客户费用（元）类型"]
#[doc = ""]
pub type TThostFtdcCustFeeType = f64;
#[doc = ""]
#[doc = "TFtdcFutureFeeType是一个应收期货公司费用（元）类型"]
#[doc = ""]
pub type TThostFtdcFutureFeeType = f64;
#[doc = ""]
#[doc = "TFtdcSingleMaxAmtType是一个单笔最高限额类型"]
#[doc = ""]
pub type TThostFtdcSingleMaxAmtType = f64;
#[doc = ""]
#[doc = "TFtdcSingleMinAmtType是一个单笔最低限额类型"]
#[doc = ""]
pub type TThostFtdcSingleMinAmtType = f64;
#[doc = ""]
#[doc = "TFtdcTotalAmtType是一个每日累计转帐额度类型"]
#[doc = ""]
pub type TThostFtdcTotalAmtType = f64;
pub type TThostFtdcCertificationTypeType = u8;
pub type TThostFtdcFileBusinessCodeType = u8;
pub type TThostFtdcCashExchangeCodeType = u8;
pub type TThostFtdcYesNoIndicatorType = u8;
pub type TThostFtdcBanlanceTypeType = u8;
pub type TThostFtdcGenderType = u8;
pub type TThostFtdcFeePayFlagType = u8;
pub type TThostFtdcPassWordKeyTypeType = u8;
pub type TThostFtdcFBTPassWordTypeType = u8;
pub type TThostFtdcFBTEncryModeType = u8;
pub type TThostFtdcBankRepealFlagType = u8;
pub type TThostFtdcBrokerRepealFlagType = u8;
pub type TThostFtdcInstitutionTypeType = u8;
pub type TThostFtdcLastFragmentType = u8;
pub type TThostFtdcBankAccStatusType = u8;
pub type TThostFtdcMoneyAccountStatusType = u8;
pub type TThostFtdcManageStatusType = u8;
pub type TThostFtdcSystemTypeType = u8;
pub type TThostFtdcTxnEndFlagType = u8;
pub type TThostFtdcProcessStatusType = u8;
pub type TThostFtdcCustTypeType = u8;
pub type TThostFtdcFBTTransferDirectionType = u8;
pub type TThostFtdcOpenOrDestroyType = u8;
pub type TThostFtdcAvailabilityFlagType = u8;
pub type TThostFtdcOrganTypeType = u8;
pub type TThostFtdcOrganLevelType = u8;
pub type TThostFtdcProtocalIDType = u8;
pub type TThostFtdcConnectModeType = u8;
pub type TThostFtdcSyncModeType = u8;
pub type TThostFtdcBankAccTypeType = u8;
pub type TThostFtdcFutureAccTypeType = u8;
pub type TThostFtdcOrganStatusType = u8;
pub type TThostFtdcCCBFeeModeType = u8;
pub type TThostFtdcCommApiTypeType = u8;
#[doc = ""]
#[doc = "TFtdcServiceIDType是一个服务编号类型"]
#[doc = ""]
pub type TThostFtdcServiceIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcServiceLineNoType是一个服务线路编号类型"]
#[doc = ""]
pub type TThostFtdcServiceLineNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcServiceNameType是一个服务名类型"]
#[doc = ""]
pub type TThostFtdcServiceNameType = [u8; 61usize];
pub type TThostFtdcLinkStatusType = u8;
#[doc = ""]
#[doc = "TFtdcCommApiPointerType是一个通讯API指针类型"]
#[doc = ""]
pub type TThostFtdcCommApiPointerType = ::std::os::raw::c_int;
pub type TThostFtdcPwdFlagType = u8;
pub type TThostFtdcSecuAccTypeType = u8;
pub type TThostFtdcTransferStatusType = TransferStatusType;
pub type TThostFtdcSponsorTypeType = u8;
pub type TThostFtdcReqRspTypeType = u8;
pub type TThostFtdcFBTUserEventTypeType = u8;
#[doc = ""]
#[doc = "TFtdcBankIDByBankType是一个银行自己的编码类型"]
#[doc = ""]
pub type TThostFtdcBankIDByBankType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcBankOperNoType是一个银行操作员号类型"]
#[doc = ""]
pub type TThostFtdcBankOperNoType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcBankCustNoType是一个银行客户号类型"]
#[doc = ""]
pub type TThostFtdcBankCustNoType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcDBOPSeqNoType是一个递增的序列号类型"]
#[doc = ""]
pub type TThostFtdcDBOPSeqNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTableNameType是一个FBT表名类型"]
#[doc = ""]
pub type TThostFtdcTableNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcPKNameType是一个FBT表操作主键名类型"]
#[doc = ""]
pub type TThostFtdcPKNameType = [u8; 201usize];
#[doc = ""]
#[doc = "TFtdcPKValueType是一个FBT表操作主键值类型"]
#[doc = ""]
pub type TThostFtdcPKValueType = [u8; 501usize];
pub type TThostFtdcDBOperationType = u8;
pub type TThostFtdcSyncFlagType = u8;
#[doc = ""]
#[doc = "TFtdcTargetIDType是一个同步目标编号类型"]
#[doc = ""]
pub type TThostFtdcTargetIDType = [u8; 4usize];
pub type TThostFtdcSyncTypeType = u8;
#[doc = ""]
#[doc = "TFtdcFBETimeType是一个各种换汇时间类型"]
#[doc = ""]
pub type TThostFtdcFBETimeType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcFBEBankNoType是一个换汇银行行号类型"]
#[doc = ""]
pub type TThostFtdcFBEBankNoType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcFBECertNoType是一个换汇凭证号类型"]
#[doc = ""]
pub type TThostFtdcFBECertNoType = [u8; 13usize];
pub type TThostFtdcExDirectionType = u8;
#[doc = ""]
#[doc = "TFtdcFBEBankAccountType是一个换汇银行账户类型"]
#[doc = ""]
pub type TThostFtdcFBEBankAccountType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcFBEBankAccountNameType是一个换汇银行账户名类型"]
#[doc = ""]
pub type TThostFtdcFBEBankAccountNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcFBEAmtType是一个各种换汇金额类型"]
#[doc = ""]
pub type TThostFtdcFBEAmtType = f64;
#[doc = ""]
#[doc = "TFtdcFBEBusinessTypeType是一个换汇业务类型类型"]
#[doc = ""]
pub type TThostFtdcFBEBusinessTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcFBEPostScriptType是一个换汇附言类型"]
#[doc = ""]
pub type TThostFtdcFBEPostScriptType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcFBERemarkType是一个换汇备注类型"]
#[doc = ""]
pub type TThostFtdcFBERemarkType = [u8; 71usize];
#[doc = ""]
#[doc = "TFtdcExRateType是一个换汇汇率类型"]
#[doc = ""]
pub type TThostFtdcExRateType = f64;
pub type TThostFtdcFBEResultFlagType = u8;
#[doc = ""]
#[doc = "TFtdcFBERtnMsgType是一个换汇返回信息类型"]
#[doc = ""]
pub type TThostFtdcFBERtnMsgType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcFBEExtendMsgType是一个换汇扩展信息类型"]
#[doc = ""]
pub type TThostFtdcFBEExtendMsgType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcFBEBusinessSerialType是一个换汇记账流水号类型"]
#[doc = ""]
pub type TThostFtdcFBEBusinessSerialType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcFBESystemSerialType是一个换汇流水号类型"]
#[doc = ""]
pub type TThostFtdcFBESystemSerialType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcFBETotalExCntType是一个换汇交易总笔数类型"]
#[doc = ""]
pub type TThostFtdcFBETotalExCntType = ::std::os::raw::c_int;
pub type TThostFtdcFBEExchStatusType = u8;
pub type TThostFtdcFBEFileFlagType = u8;
pub type TThostFtdcFBEAlreadyTradeType = u8;
#[doc = ""]
#[doc = "TFtdcFBEOpenBankType是一个换汇账户开户行类型"]
#[doc = ""]
pub type TThostFtdcFBEOpenBankType = [u8; 61usize];
pub type TThostFtdcFBEUserEventTypeType = u8;
#[doc = ""]
#[doc = "TFtdcFBEFileNameType是一个换汇相关文件名类型"]
#[doc = ""]
pub type TThostFtdcFBEFileNameType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcFBEBatchSerialType是一个换汇批次号类型"]
#[doc = ""]
pub type TThostFtdcFBEBatchSerialType = [u8; 21usize];
pub type TThostFtdcFBEReqFlagType = u8;
pub type TThostFtdcNotifyClassType = u8;
#[doc = ""]
#[doc = "TFtdcRiskNofityInfoType是一个客户风险通知消息类型"]
#[doc = ""]
pub type TThostFtdcRiskNofityInfoType = [u8; 257usize];
#[doc = ""]
#[doc = "TFtdcForceCloseSceneIdType是一个强平场景编号类型"]
#[doc = ""]
pub type TThostFtdcForceCloseSceneIdType = [u8; 24usize];
pub type TThostFtdcForceCloseTypeType = u8;
#[doc = ""]
#[doc = "TFtdcInstrumentIDsType是一个多个产品代码,用+分隔,如cu+zn类型"]
#[doc = ""]
pub type TThostFtdcInstrumentIDsType = [u8; 101usize];
pub type TThostFtdcRiskNotifyMethodType = u8;
pub type TThostFtdcRiskNotifyStatusType = u8;
pub type TThostFtdcRiskUserEventType = u8;
#[doc = ""]
#[doc = "TFtdcParamIDType是一个参数代码类型"]
#[doc = ""]
pub type TThostFtdcParamIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcParamNameType是一个参数名类型"]
#[doc = ""]
pub type TThostFtdcParamNameType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcParamValueType是一个参数值类型"]
#[doc = ""]
pub type TThostFtdcParamValueType = [u8; 41usize];
pub type TThostFtdcConditionalOrderSortTypeType = u8;
pub type TThostFtdcSendTypeType = u8;
pub type TThostFtdcClientIDStatusType = u8;
#[doc = ""]
#[doc = "TFtdcIndustryIDType是一个行业编码类型"]
#[doc = ""]
pub type TThostFtdcIndustryIDType = [u8; 17usize];
#[doc = ""]
#[doc = "TFtdcQuestionIDType是一个特有信息编号类型"]
#[doc = ""]
pub type TThostFtdcQuestionIDType = [u8; 5usize];
#[doc = ""]
#[doc = "TFtdcQuestionContentType是一个特有信息说明类型"]
#[doc = ""]
pub type TThostFtdcQuestionContentType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcOptionIDType是一个选项编号类型"]
#[doc = ""]
pub type TThostFtdcOptionIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcOptionContentType是一个选项说明类型"]
#[doc = ""]
pub type TThostFtdcOptionContentType = [u8; 61usize];
pub type TThostFtdcQuestionTypeType = u8;
#[doc = ""]
#[doc = "TFtdcProcessIDType是一个业务流水号类型"]
#[doc = ""]
pub type TThostFtdcProcessIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcSeqNoType是一个流水号类型"]
#[doc = ""]
pub type TThostFtdcSeqNoType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcUOAProcessStatusType是一个流程状态类型"]
#[doc = ""]
pub type TThostFtdcUOAProcessStatusType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcProcessTypeType是一个流程功能类型类型"]
#[doc = ""]
pub type TThostFtdcProcessTypeType = [u8; 3usize];
pub type TThostFtdcBusinessTypeType = u8;
pub type TThostFtdcCfmmcReturnCodeType = u8;
#[doc = ""]
#[doc = "TFtdcExReturnCodeType是一个交易所返回码类型"]
#[doc = ""]
pub type TThostFtdcExReturnCodeType = ::std::os::raw::c_int;
pub type TThostFtdcClientTypeType = u8;
pub type TThostFtdcExchangeIDTypeType = u8;
pub type TThostFtdcExClientIDTypeType = u8;
#[doc = ""]
#[doc = "TFtdcClientClassifyType是一个客户分类码类型"]
#[doc = ""]
pub type TThostFtdcClientClassifyType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcUOAOrganTypeType是一个单位性质类型"]
#[doc = ""]
pub type TThostFtdcUOAOrganTypeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcUOACountryCodeType是一个国家代码类型"]
#[doc = ""]
pub type TThostFtdcUOACountryCodeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcAreaCodeType是一个区号类型"]
#[doc = ""]
pub type TThostFtdcAreaCodeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcFuturesIDType是一个监控中心为客户分配的代码类型"]
#[doc = ""]
pub type TThostFtdcFuturesIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcCffmcDateType是一个日期类型"]
#[doc = ""]
pub type TThostFtdcCffmcDateType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcCffmcTimeType是一个时间类型"]
#[doc = ""]
pub type TThostFtdcCffmcTimeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcNocIDType是一个组织机构代码类型"]
#[doc = ""]
pub type TThostFtdcNocIDType = [u8; 21usize];
pub type TThostFtdcUpdateFlagType = u8;
pub type TThostFtdcApplyOperateIDType = u8;
pub type TThostFtdcApplyStatusIDType = u8;
pub type TThostFtdcSendMethodType = u8;
#[doc = ""]
#[doc = "TFtdcEventTypeType是一个业务操作类型类型"]
#[doc = ""]
pub type TThostFtdcEventTypeType = [u8; 33usize];
pub type TThostFtdcEventModeType = u8;
pub type TThostFtdcUOAAutoSendType = u8;
#[doc = ""]
#[doc = "TFtdcQueryDepthType是一个查询深度类型"]
#[doc = ""]
pub type TThostFtdcQueryDepthType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcDataCenterIDType是一个数据中心代码类型"]
#[doc = ""]
pub type TThostFtdcDataCenterIDType = ::std::os::raw::c_int;
pub type TThostFtdcFlowIDType = u8;
pub type TThostFtdcCheckLevelType = u8;
#[doc = ""]
#[doc = "TFtdcCheckNoType是一个操作次数类型"]
#[doc = ""]
pub type TThostFtdcCheckNoType = ::std::os::raw::c_int;
pub type TThostFtdcCheckStatusType = u8;
pub type TThostFtdcUsedStatusType = u8;
#[doc = ""]
#[doc = "TFtdcRateTemplateNameType是一个模型名称类型"]
#[doc = ""]
pub type TThostFtdcRateTemplateNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcPropertyStringType是一个用于查询的投资属性字段类型"]
#[doc = ""]
pub type TThostFtdcPropertyStringType = [u8; 2049usize];
pub type TThostFtdcBankAcountOriginType = u8;
pub type TThostFtdcMonthBillTradeSumType = u8;
pub type TThostFtdcFBTTradeCodeEnumType = u8;
#[doc = ""]
#[doc = "TFtdcRateTemplateIDType是一个模型代码类型"]
#[doc = ""]
pub type TThostFtdcRateTemplateIDType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcRiskRateType是一个风险度类型"]
#[doc = ""]
pub type TThostFtdcRiskRateType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcTimestampType是一个时间戳类型"]
#[doc = ""]
pub type TThostFtdcTimestampType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcInvestorIDRuleNameType是一个号段规则名称类型"]
#[doc = ""]
pub type TThostFtdcInvestorIDRuleNameType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcInvestorIDRuleExprType是一个号段规则表达式类型"]
#[doc = ""]
pub type TThostFtdcInvestorIDRuleExprType = [u8; 513usize];
#[doc = ""]
#[doc = "TFtdcLastDriftType是一个上次OTP漂移值类型"]
#[doc = ""]
pub type TThostFtdcLastDriftType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcLastSuccessType是一个上次OTP成功值类型"]
#[doc = ""]
pub type TThostFtdcLastSuccessType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcAuthKeyType是一个令牌密钥类型"]
#[doc = ""]
pub type TThostFtdcAuthKeyType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcSerialNumberType是一个序列号类型"]
#[doc = ""]
pub type TThostFtdcSerialNumberType = [u8; 17usize];
pub type TThostFtdcOTPTypeType = u8;
#[doc = ""]
#[doc = "TFtdcOTPVendorsIDType是一个动态令牌提供商类型"]
#[doc = ""]
pub type TThostFtdcOTPVendorsIDType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcOTPVendorsNameType是一个动态令牌提供商名称类型"]
#[doc = ""]
pub type TThostFtdcOTPVendorsNameType = [u8; 61usize];
pub type TThostFtdcOTPStatusType = u8;
pub type TThostFtdcBrokerUserTypeType = u8;
pub type TThostFtdcFutureTypeType = u8;
pub type TThostFtdcFundEventTypeType = u8;
pub type TThostFtdcAccountSourceTypeType = u8;
pub type TThostFtdcCodeSourceTypeType = u8;
pub type TThostFtdcUserRangeType = u8;
#[doc = ""]
#[doc = "TFtdcTimeSpanType是一个时间跨度类型"]
#[doc = ""]
pub type TThostFtdcTimeSpanType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcImportSequenceIDType是一个动态令牌导入批次编号类型"]
#[doc = ""]
pub type TThostFtdcImportSequenceIDType = [u8; 17usize];
pub type TThostFtdcByGroupType = u8;
pub type TThostFtdcTradeSumStatModeType = u8;
#[doc = ""]
#[doc = "TFtdcComTypeType是一个组合成交类型类型"]
#[doc = ""]
pub type TThostFtdcComTypeType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcUserProductIDType是一个产品标识类型"]
#[doc = ""]
pub type TThostFtdcUserProductIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcUserProductNameType是一个产品名称类型"]
#[doc = ""]
pub type TThostFtdcUserProductNameType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcUserProductMemoType是一个产品说明类型"]
#[doc = ""]
pub type TThostFtdcUserProductMemoType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcCSRCCancelFlagType是一个新增或变更标志类型"]
#[doc = ""]
pub type TThostFtdcCSRCCancelFlagType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcCSRCDateType是一个日期类型"]
#[doc = ""]
pub type TThostFtdcCSRCDateType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcCSRCInvestorNameType是一个客户名称类型"]
#[doc = ""]
pub type TThostFtdcCSRCInvestorNameType = [u8; 201usize];
#[doc = ""]
#[doc = "TFtdcCSRCOpenInvestorNameType是一个客户名称类型"]
#[doc = ""]
pub type TThostFtdcCSRCOpenInvestorNameType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcCSRCInvestorIDType是一个客户代码类型"]
#[doc = ""]
pub type TThostFtdcCSRCInvestorIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcCSRCIdentifiedCardNoType是一个证件号码类型"]
#[doc = ""]
pub type TThostFtdcCSRCIdentifiedCardNoType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcCSRCClientIDType是一个交易编码类型"]
#[doc = ""]
pub type TThostFtdcCSRCClientIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcCSRCBankFlagType是一个银行标识类型"]
#[doc = ""]
pub type TThostFtdcCSRCBankFlagType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcCSRCBankAccountType是一个银行账户类型"]
#[doc = ""]
pub type TThostFtdcCSRCBankAccountType = [u8; 23usize];
#[doc = ""]
#[doc = "TFtdcCSRCOpenNameType是一个开户人类型"]
#[doc = ""]
pub type TThostFtdcCSRCOpenNameType = [u8; 401usize];
#[doc = ""]
#[doc = "TFtdcCSRCMemoType是一个说明类型"]
#[doc = ""]
pub type TThostFtdcCSRCMemoType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcCSRCTimeType是一个时间类型"]
#[doc = ""]
pub type TThostFtdcCSRCTimeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcCSRCTradeIDType是一个成交流水号类型"]
#[doc = ""]
pub type TThostFtdcCSRCTradeIDType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcCSRCExchangeInstIDType是一个合约代码类型"]
#[doc = ""]
pub type TThostFtdcCSRCExchangeInstIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcCSRCMortgageNameType是一个质押品名称类型"]
#[doc = ""]
pub type TThostFtdcCSRCMortgageNameType = [u8; 7usize];
#[doc = ""]
#[doc = "TFtdcCSRCReasonType是一个事由类型"]
#[doc = ""]
pub type TThostFtdcCSRCReasonType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcIsSettlementType是一个是否为非结算会员类型"]
#[doc = ""]
pub type TThostFtdcIsSettlementType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcCSRCMoneyType是一个资金类型"]
#[doc = ""]
pub type TThostFtdcCSRCMoneyType = f64;
#[doc = ""]
#[doc = "TFtdcCSRCPriceType是一个价格类型"]
#[doc = ""]
pub type TThostFtdcCSRCPriceType = f64;
#[doc = ""]
#[doc = "TFtdcCSRCOptionsTypeType是一个期权类型类型"]
#[doc = ""]
pub type TThostFtdcCSRCOptionsTypeType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcCSRCStrikePriceType是一个执行价类型"]
#[doc = ""]
pub type TThostFtdcCSRCStrikePriceType = f64;
#[doc = ""]
#[doc = "TFtdcCSRCTargetProductIDType是一个标的品种类型"]
#[doc = ""]
pub type TThostFtdcCSRCTargetProductIDType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcCSRCTargetInstrIDType是一个标的合约类型"]
#[doc = ""]
pub type TThostFtdcCSRCTargetInstrIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcCommModelNameType是一个手续费率模板名称类型"]
#[doc = ""]
pub type TThostFtdcCommModelNameType = [u8; 161usize];
#[doc = ""]
#[doc = "TFtdcCommModelMemoType是一个手续费率模板备注类型"]
#[doc = ""]
pub type TThostFtdcCommModelMemoType = [u8; 1025usize];
pub type TThostFtdcExprSetModeType = u8;
pub type TThostFtdcRateInvestorRangeType = u8;
#[doc = ""]
#[doc = "TFtdcAgentBrokerIDType是一个代理经纪公司代码类型"]
#[doc = ""]
pub type TThostFtdcAgentBrokerIDType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcDRIdentityIDType是一个交易中心代码类型"]
#[doc = ""]
pub type TThostFtdcDRIdentityIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcDRIdentityNameType是一个交易中心名称类型"]
#[doc = ""]
pub type TThostFtdcDRIdentityNameType = [u8; 65usize];
#[doc = ""]
#[doc = "TFtdcDBLinkIDType是一个DBLink标识号类型"]
#[doc = ""]
pub type TThostFtdcDBLinkIDType = [u8; 31usize];
pub type TThostFtdcSyncDataStatusType = u8;
pub type TThostFtdcTradeSourceType = OrderTradeSourceType;
pub type TThostFtdcFlexStatModeType = u8;
pub type TThostFtdcByInvestorRangeType = u8;
#[doc = ""]
#[doc = "TFtdcSRiskRateType是一个风险度类型"]
#[doc = ""]
pub type TThostFtdcSRiskRateType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcSequenceNo12Type是一个序号类型"]
#[doc = ""]
pub type TThostFtdcSequenceNo12Type = ::std::os::raw::c_int;
pub type TThostFtdcPropertyInvestorRangeType = u8;
pub type TThostFtdcFileStatusType = u8;
pub type TThostFtdcFileGenStyleType = u8;
pub type TThostFtdcSysOperModeType = u8;
pub type TThostFtdcSysOperTypeType = u8;
pub type TThostFtdcCSRCDataQueyTypeType = u8;
pub type TThostFtdcFreezeStatusType = u8;
pub type TThostFtdcStandardStatusType = u8;
#[doc = ""]
#[doc = "TFtdcCSRCFreezeStatusType是一个休眠状态类型"]
#[doc = ""]
pub type TThostFtdcCSRCFreezeStatusType = [u8; 2usize];
pub type TThostFtdcRightParamTypeType = u8;
#[doc = ""]
#[doc = "TFtdcRightTemplateIDType是一个模板代码类型"]
#[doc = ""]
pub type TThostFtdcRightTemplateIDType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcRightTemplateNameType是一个模板名称类型"]
#[doc = ""]
pub type TThostFtdcRightTemplateNameType = [u8; 61usize];
pub type TThostFtdcDataStatusType = u8;
pub type TThostFtdcAMLCheckStatusType = u8;
pub type TThostFtdcAmlDateTypeType = u8;
pub type TThostFtdcAmlCheckLevelType = u8;
#[doc = ""]
#[doc = "TFtdcAmlCheckFlowType是一个反洗钱数据抽取审核流程类型"]
#[doc = ""]
pub type TThostFtdcAmlCheckFlowType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcDataTypeType是一个数据类型类型"]
#[doc = ""]
pub type TThostFtdcDataTypeType = [u8; 129usize];
pub type TThostFtdcExportFileTypeType = u8;
pub type TThostFtdcSettleManagerTypeType = u8;
#[doc = ""]
#[doc = "TFtdcSettleManagerIDType是一个结算配置代码类型"]
#[doc = ""]
pub type TThostFtdcSettleManagerIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcSettleManagerNameType是一个结算配置名称类型"]
#[doc = ""]
pub type TThostFtdcSettleManagerNameType = [u8; 129usize];
pub type TThostFtdcSettleManagerLevelType = u8;
pub type TThostFtdcSettleManagerGroupType = u8;
#[doc = ""]
#[doc = "TFtdcCheckResultMemoType是一个核对结果说明类型"]
#[doc = ""]
pub type TThostFtdcCheckResultMemoType = [u8; 1025usize];
#[doc = ""]
#[doc = "TFtdcFunctionUrlType是一个功能链接类型"]
#[doc = ""]
pub type TThostFtdcFunctionUrlType = [u8; 1025usize];
#[doc = ""]
#[doc = "TFtdcAuthInfoType是一个客户端认证信息类型"]
#[doc = ""]
pub type TThostFtdcAuthInfoType = [u8; 129usize];
#[doc = ""]
#[doc = "TFtdcAuthCodeType是一个客户端认证码类型"]
#[doc = ""]
pub type TThostFtdcAuthCodeType = [u8; 17usize];
pub type TThostFtdcLimitUseTypeType = u8;
pub type TThostFtdcDataResourceType = u8;
pub type TThostFtdcMarginTypeType = u8;
pub type TThostFtdcActiveTypeType = u8;
pub type TThostFtdcMarginRateTypeType = u8;
pub type TThostFtdcBackUpStatusType = u8;
pub type TThostFtdcInitSettlementType = u8;
pub type TThostFtdcReportStatusType = u8;
pub type TThostFtdcSaveStatusType = u8;
pub type TThostFtdcSettArchiveStatusType = u8;
pub type TThostFtdcCTPTypeType = u8;
#[doc = ""]
#[doc = "TFtdcToolIDType是一个工具代码类型"]
#[doc = ""]
pub type TThostFtdcToolIDType = [u8; 9usize];
#[doc = ""]
#[doc = "TFtdcToolNameType是一个工具名称类型"]
#[doc = ""]
pub type TThostFtdcToolNameType = [u8; 81usize];
pub type TThostFtdcCloseDealTypeType = u8;
pub type TThostFtdcMortgageFundUseRangeType = u8;
#[doc = ""]
#[doc = "TFtdcCurrencyUnitType是一个币种单位数量类型"]
#[doc = ""]
pub type TThostFtdcCurrencyUnitType = f64;
#[doc = ""]
#[doc = "TFtdcExchangeRateType是一个汇率类型"]
#[doc = ""]
pub type TThostFtdcExchangeRateType = f64;
pub type TThostFtdcSpecProductTypeType = u8;
pub type TThostFtdcFundMortgageTypeType = u8;
pub type TThostFtdcAccountSettlementParamIDType = u8;
#[doc = ""]
#[doc = "TFtdcCurrencyNameType是一个币种名称类型"]
#[doc = ""]
pub type TThostFtdcCurrencyNameType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcCurrencySignType是一个币种符号类型"]
#[doc = ""]
pub type TThostFtdcCurrencySignType = [u8; 4usize];
pub type TThostFtdcFundMortDirectionType = u8;
pub type TThostFtdcBusinessClassType = u8;
pub type TThostFtdcSwapSourceTypeType = u8;
pub type TThostFtdcCurrExDirectionType = u8;
pub type TThostFtdcCurrencySwapStatusType = u8;
#[doc = ""]
#[doc = "TFtdcCurrExchCertNoType是一个凭证号类型"]
#[doc = ""]
pub type TThostFtdcCurrExchCertNoType = [u8; 13usize];
#[doc = ""]
#[doc = "TFtdcBatchSerialNoType是一个批次号类型"]
#[doc = ""]
pub type TThostFtdcBatchSerialNoType = [u8; 21usize];
pub type TThostFtdcReqFlagType = u8;
pub type TThostFtdcResFlagType = u8;
#[doc = ""]
#[doc = "TFtdcPageControlType是一个换汇页面控制类型"]
#[doc = ""]
pub type TThostFtdcPageControlType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcRecordCountType是一个记录数类型"]
#[doc = ""]
pub type TThostFtdcRecordCountType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCurrencySwapMemoType是一个换汇需确认信息类型"]
#[doc = ""]
pub type TThostFtdcCurrencySwapMemoType = [u8; 101usize];
pub type TThostFtdcExStatusType = u8;
pub type TThostFtdcClientRegionType = u8;
#[doc = ""]
#[doc = "TFtdcWorkPlaceType是一个工作单位类型"]
#[doc = ""]
pub type TThostFtdcWorkPlaceType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcBusinessPeriodType是一个经营期限类型"]
#[doc = ""]
pub type TThostFtdcBusinessPeriodType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcWebSiteType是一个网址类型"]
#[doc = ""]
pub type TThostFtdcWebSiteType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcUOAIdCardTypeType是一个统一开户证件类型类型"]
#[doc = ""]
pub type TThostFtdcUOAIdCardTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcClientModeType是一个开户模式类型"]
#[doc = ""]
pub type TThostFtdcClientModeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcInvestorFullNameType是一个投资者全称类型"]
#[doc = ""]
pub type TThostFtdcInvestorFullNameType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcUOABrokerIDType是一个境外中介机构ID类型"]
#[doc = ""]
pub type TThostFtdcUOABrokerIDType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcUOAZipCodeType是一个邮政编码类型"]
#[doc = ""]
pub type TThostFtdcUOAZipCodeType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcUOAEMailType是一个电子邮箱类型"]
#[doc = ""]
pub type TThostFtdcUOAEMailType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcOldCityType是一个城市类型"]
#[doc = ""]
pub type TThostFtdcOldCityType = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcCorporateIdentifiedCardNoType是一个法人代表证件号码类型"]
#[doc = ""]
pub type TThostFtdcCorporateIdentifiedCardNoType = [u8; 101usize];
pub type TThostFtdcHasBoardType = u8;
pub type TThostFtdcStartModeType = u8;
pub type TThostFtdcTemplateTypeType = u8;
pub type TThostFtdcLoginModeType = u8;
pub type TThostFtdcPromptTypeType = u8;
#[doc = ""]
#[doc = "TFtdcLedgerManageIDType是一个分户管理资产编码类型"]
#[doc = ""]
pub type TThostFtdcLedgerManageIDType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcInvestVarietyType是一个投资品种类型"]
#[doc = ""]
pub type TThostFtdcInvestVarietyType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcBankAccountTypeType是一个账户类别类型"]
#[doc = ""]
pub type TThostFtdcBankAccountTypeType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcLedgerManageBankType是一个开户银行类型"]
#[doc = ""]
pub type TThostFtdcLedgerManageBankType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcCffexDepartmentNameType是一个开户营业部类型"]
#[doc = ""]
pub type TThostFtdcCffexDepartmentNameType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcCffexDepartmentCodeType是一个营业部代码类型"]
#[doc = ""]
pub type TThostFtdcCffexDepartmentCodeType = [u8; 9usize];
pub type TThostFtdcHasTrusteeType = u8;
#[doc = ""]
#[doc = "TFtdcCSRCMemo1Type是一个说明类型"]
#[doc = ""]
pub type TThostFtdcCSRCMemo1Type = [u8; 41usize];
#[doc = ""]
#[doc = "TFtdcAssetmgrCFullNameType是一个代理资产管理业务的期货公司全称类型"]
#[doc = ""]
pub type TThostFtdcAssetmgrCFullNameType = [u8; 101usize];
#[doc = ""]
#[doc = "TFtdcAssetmgrApprovalNOType是一个资产管理业务批文号类型"]
#[doc = ""]
pub type TThostFtdcAssetmgrApprovalNOType = [u8; 51usize];
#[doc = ""]
#[doc = "TFtdcAssetmgrMgrNameType是一个资产管理业务负责人姓名类型"]
#[doc = ""]
pub type TThostFtdcAssetmgrMgrNameType = [u8; 401usize];
pub type TThostFtdcAmTypeType = u8;
#[doc = ""]
#[doc = "TFtdcCSRCAmTypeType是一个机构类型类型"]
#[doc = ""]
pub type TThostFtdcCSRCAmTypeType = [u8; 5usize];
pub type TThostFtdcCSRCFundIOTypeType = u8;
pub type TThostFtdcCusAccountTypeType = u8;
#[doc = ""]
#[doc = "TFtdcCSRCNationalType是一个国籍类型"]
#[doc = ""]
pub type TThostFtdcCSRCNationalType = [u8; 4usize];
#[doc = ""]
#[doc = "TFtdcCSRCSecAgentIDType是一个二级代理ID类型"]
#[doc = ""]
pub type TThostFtdcCSRCSecAgentIDType = [u8; 11usize];
pub type TThostFtdcLanguageTypeType = u8;
#[doc = ""]
#[doc = "TFtdcAmAccountType是一个投资账户类型"]
#[doc = ""]
pub type TThostFtdcAmAccountType = [u8; 23usize];
pub type TThostFtdcAssetmgrClientTypeType = u8;
pub type TThostFtdcAssetmgrTypeType = u8;
#[doc = ""]
#[doc = "TFtdcUOMType是一个计量单位类型"]
#[doc = ""]
pub type TThostFtdcUOMType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcSHFEInstLifePhaseType是一个上期所合约生命周期状态类型"]
#[doc = ""]
pub type TThostFtdcSHFEInstLifePhaseType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcSHFEProductClassType是一个产品类型类型"]
#[doc = ""]
pub type TThostFtdcSHFEProductClassType = [u8; 11usize];
#[doc = ""]
#[doc = "TFtdcPriceDecimalType是一个价格小数位类型"]
#[doc = ""]
pub type TThostFtdcPriceDecimalType = [u8; 2usize];
#[doc = ""]
#[doc = "TFtdcInTheMoneyFlagType是一个平值期权标志类型"]
#[doc = ""]
pub type TThostFtdcInTheMoneyFlagType = [u8; 2usize];
pub type TThostFtdcCheckInstrTypeType = u8;
pub type TThostFtdcDeliveryTypeType = u8;
#[doc = ""]
#[doc = "TFtdcBigMoneyType是一个资金类型"]
#[doc = ""]
pub type TThostFtdcBigMoneyType = f64;
pub type TThostFtdcMaxMarginSideAlgorithmType = MaxMarginSideAlgorithmType;
pub type TThostFtdcDAClientTypeType = u8;
#[doc = ""]
#[doc = "TFtdcCombinInstrIDType是一个套利合约代码类型"]
#[doc = ""]
pub type TThostFtdcCombinInstrIDType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcCombinSettlePriceType是一个各腿结算价类型"]
#[doc = ""]
pub type TThostFtdcCombinSettlePriceType = [u8; 61usize];
#[doc = ""]
#[doc = "TFtdcDCEPriorityType是一个优先级类型"]
#[doc = ""]
pub type TThostFtdcDCEPriorityType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcTradeGroupIDType是一个成交组号类型"]
#[doc = ""]
pub type TThostFtdcTradeGroupIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcIsCheckPrepaType是一个是否校验开户可用资金类型"]
#[doc = ""]
pub type TThostFtdcIsCheckPrepaType = ::std::os::raw::c_int;
pub type TThostFtdcUOAAssetmgrTypeType = u8;
pub type TThostFtdcDirectionEnType = u8;
pub type TThostFtdcOffsetFlagEnType = u8;
pub type TThostFtdcHedgeFlagEnType = u8;
pub type TThostFtdcFundIOTypeEnType = u8;
pub type TThostFtdcFundTypeEnType = u8;
pub type TThostFtdcFundDirectionEnType = u8;
pub type TThostFtdcFundMortDirectionEnType = u8;
#[doc = ""]
#[doc = "TFtdcSwapBusinessTypeType是一个换汇业务种类类型"]
#[doc = ""]
pub type TThostFtdcSwapBusinessTypeType = [u8; 3usize];
pub type TThostFtdcOptionsTypeType = OptionsType;
pub type TThostFtdcStrikeModeType = u8;
pub type TThostFtdcStrikeTypeType = u8;
pub type TThostFtdcApplyTypeType = u8;
pub type TThostFtdcGiveUpDataSourceType = u8;
#[doc = ""]
#[doc = "TFtdcExecOrderSysIDType是一个执行宣告系统编号类型"]
#[doc = ""]
pub type TThostFtdcExecOrderSysIDType = [u8; 21usize];
pub type TThostFtdcExecResultType = u8;
#[doc = ""]
#[doc = "TFtdcStrikeSequenceType是一个执行序号类型"]
#[doc = ""]
pub type TThostFtdcStrikeSequenceType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcStrikeTimeType是一个执行时间类型"]
#[doc = ""]
pub type TThostFtdcStrikeTimeType = [u8; 13usize];
pub type TThostFtdcCombinationTypeType = CombinationType;
cfg_if::cfg_if! {if #[cfg(not(feature = "v6_3_13"))] {
pub type TThostFtdcDceCombinationTypeType = u8;
}}
pub type TThostFtdcOptionRoyaltyPriceTypeType = u8;
pub type TThostFtdcBalanceAlgorithmType = u8;
pub type TThostFtdcActionTypeType = u8;
pub type TThostFtdcForQuoteStatusType = u8;
pub type TThostFtdcValueMethodType = u8;
pub type TThostFtdcExecOrderPositionFlagType = u8;
pub type TThostFtdcExecOrderCloseFlagType = u8;
pub type TThostFtdcProductTypeType = u8;
pub type TThostFtdcCZCEUploadFileNameType = u8;
pub type TThostFtdcDCEUploadFileNameType = u8;
pub type TThostFtdcSHFEUploadFileNameType = u8;
pub type TThostFtdcCFFEXUploadFileNameType = u8;
pub type TThostFtdcCombDirectionType = u8;
pub type TThostFtdcStrikeOffsetTypeType = u8;
pub type TThostFtdcReserveOpenAccStasType = u8;
#[doc = ""]
#[doc = "TFtdcLoginRemarkType是一个登录备注类型"]
#[doc = ""]
pub type TThostFtdcLoginRemarkType = [u8; 36usize];
#[doc = ""]
#[doc = "TFtdcInvestUnitIDType是一个投资单元代码类型"]
#[doc = ""]
pub type TThostFtdcInvestUnitIDType = [u8; 17usize];
#[doc = ""]
#[doc = "TFtdcBulletinIDType是一个公告编号类型"]
#[doc = ""]
pub type TThostFtdcBulletinIDType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcNewsTypeType是一个公告类型类型"]
#[doc = ""]
pub type TThostFtdcNewsTypeType = [u8; 3usize];
#[doc = ""]
#[doc = "TFtdcNewsUrgencyType是一个紧急程度类型"]
#[doc = ""]
pub type TThostFtdcNewsUrgencyType = u8;
#[doc = ""]
#[doc = "TFtdcAbstractType是一个消息摘要类型"]
#[doc = ""]
pub type TThostFtdcAbstractType = [u8; 81usize];
#[doc = ""]
#[doc = "TFtdcComeFromType是一个消息来源类型"]
#[doc = ""]
pub type TThostFtdcComeFromType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcURLLinkType是一个WEB地址类型"]
#[doc = ""]
pub type TThostFtdcURLLinkType = [u8; 201usize];
#[doc = ""]
#[doc = "TFtdcLongIndividualNameType是一个长个人姓名类型"]
#[doc = ""]
pub type TThostFtdcLongIndividualNameType = [u8; 161usize];
#[doc = ""]
#[doc = "TFtdcLongFBEBankAccountNameType是一个长换汇银行账户名类型"]
#[doc = ""]
pub type TThostFtdcLongFBEBankAccountNameType = [u8; 161usize];
#[doc = ""]
#[doc = "TFtdcDateTimeType是一个日期时间类型"]
#[doc = ""]
pub type TThostFtdcDateTimeType = [u8; 17usize];
pub type TThostFtdcWeakPasswordSourceType = u8;
#[doc = ""]
#[doc = "TFtdcRandomStringType是一个随机串类型"]
#[doc = ""]
pub type TThostFtdcRandomStringType = [u8; 17usize];
pub type TThostFtdcOptSelfCloseFlagType = u8;
pub type TThostFtdcBizTypeType = BizType;
pub type TThostFtdcAppTypeType = u8;
#[doc = ""]
#[doc = "TFtdcAppIDType是一个App代码类型"]
#[doc = ""]
pub type TThostFtdcAppIDType = [u8; 33usize];
#[doc = ""]
#[doc = "TFtdcSystemInfoLenType是一个系统信息长度类型"]
#[doc = ""]
pub type TThostFtdcSystemInfoLenType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcAdditionalInfoLenType是一个补充信息长度类型"]
#[doc = ""]
pub type TThostFtdcAdditionalInfoLenType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcClientSystemInfoType是一个交易终端系统信息类型"]
#[doc = ""]
pub type TThostFtdcClientSystemInfoType = [u8; 273usize];
#[doc = ""]
#[doc = "TFtdcAdditionalInfoType是一个系统外部信息类型"]
#[doc = ""]
pub type TThostFtdcAdditionalInfoType = [u8; 261usize];
#[doc = ""]
#[doc = "TFtdcBase64ClientSystemInfoType是一个base64交易终端系统信息类型"]
#[doc = ""]
pub type TThostFtdcBase64ClientSystemInfoType = [u8; 365usize];
#[doc = ""]
#[doc = "TFtdcBase64AdditionalInfoType是一个base64系统外部信息类型"]
#[doc = ""]
pub type TThostFtdcBase64AdditionalInfoType = [u8; 349usize];
#[doc = ""]
#[doc = "TFtdcCurrentAuthMethodType是一个当前可用的认证模式，0代表无需认证模式 A从低位开始最后一位代表图片验证码，倒数第二位代表动态口令，倒数第三位代表短信验证码类型"]
#[doc = ""]
pub type TThostFtdcCurrentAuthMethodType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCaptchaInfoLenType是一个图片验证信息长度类型"]
#[doc = ""]
pub type TThostFtdcCaptchaInfoLenType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCaptchaInfoType是一个图片验证信息类型"]
#[doc = ""]
pub type TThostFtdcCaptchaInfoType = [u8; 2561usize];
#[doc = ""]
#[doc = "TFtdcUserTextSeqType是一个用户短信验证码的编号类型"]
#[doc = ""]
pub type TThostFtdcUserTextSeqType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcHandshakeDataType是一个握手数据内容类型"]
#[doc = ""]
pub type TThostFtdcHandshakeDataType = [u8; 301usize];
#[doc = ""]
#[doc = "TFtdcHandshakeDataLenType是一个握手数据内容长度类型"]
#[doc = ""]
pub type TThostFtdcHandshakeDataLenType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcCryptoKeyVersionType是一个api与front通信密钥版本号类型"]
#[doc = ""]
pub type TThostFtdcCryptoKeyVersionType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcRsaKeyVersionType是一个公钥版本号类型"]
#[doc = ""]
pub type TThostFtdcRsaKeyVersionType = ::std::os::raw::c_int;
#[doc = ""]
#[doc = "TFtdcSoftwareProviderIDType是一个交易软件商ID类型"]
#[doc = ""]
pub type TThostFtdcSoftwareProviderIDType = [u8; 22usize];
#[doc = ""]
#[doc = "TFtdcCollectTimeType是一个信息采集时间类型"]
#[doc = ""]
pub type TThostFtdcCollectTimeType = [u8; 21usize];
#[doc = ""]
#[doc = "TFtdcQueryFreqType是一个查询频率类型"]
#[doc = ""]
pub type TThostFtdcQueryFreqType = ::std::os::raw::c_int;
pub type TThostFtdcResponseValueType = u8;
pub type TThostFtdcOTCTradeTypeType = u8;
pub type TThostFtdcMatchTypeType = u8;
#[doc = ""]
#[doc = "TFtdcOTCTraderIDType是一个OTC交易员代码类型"]
#[doc = ""]
pub type TThostFtdcOTCTraderIDType = [u8; 31usize];
#[doc = ""]
#[doc = "TFtdcRiskValueType是一个期货风险值类型"]
#[doc = ""]
pub type TThostFtdcRiskValueType = f64;
#[doc = ""]
#[doc = "TFtdcIDBNameType是一个握手数据内容类型"]
#[doc = ""]
pub type TThostFtdcIDBNameType = [u8; 100usize];
#[doc = "信息分发"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcDisseminationField {
    #[doc = "序列系列号"]
    pub SequenceSeries: TThostFtdcSequenceSeriesType,
    #[doc = "序列号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
}
#[doc = "用户登录请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqUserLoginField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
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
    #[serde(with = "BigArray")]
    pub OneTimePassword: TThostFtdcPasswordType,
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
    #[doc = "终端IP端口"]
    pub ClientIPPort: TThostFtdcIPPortType,
}
impl Default for CThostFtdcReqUserLoginField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户登录应答"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspUserLoginField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "登录成功时间"]
    pub LoginTime: TThostFtdcTimeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易系统名称"]
    #[serde(with = "BigArray")]
    pub SystemName: TThostFtdcSystemNameType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "最大报单引用"]
    pub MaxOrderRef: TThostFtdcOrderRefType,
    #[doc = "上期所时间"]
    pub SHFETime: TThostFtdcTimeType,
    #[doc = "大商所时间"]
    pub DCETime: TThostFtdcTimeType,
    #[doc = "郑商所时间"]
    pub CZCETime: TThostFtdcTimeType,
    #[doc = "中金所时间"]
    pub FFEXTime: TThostFtdcTimeType,
    #[doc = "能源中心时间"]
    pub INETime: TThostFtdcTimeType,
    #[cfg(feature = "v6_7_0")]
    #[doc = "后台版本信息"]
    #[serde(with = "BigArray")]
    pub SysVersion: TThostFtdcSysVersionType,
    #[cfg(feature = "v6_7_0")]
    #[doc = "广期所时间"]
    pub GFEXTime: TThostFtdcTimeType,
}
impl Default for CThostFtdcRspUserLoginField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户登出请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcUserLogoutField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "强制交易员退出"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcForceUserLogoutField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "客户端认证请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
    #[serde(with = "BigArray")]
    pub AppID: TThostFtdcAppIDType,
}
impl Default for CThostFtdcReqAuthenticateField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "客户端认证响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspAuthenticateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "App代码"]
    #[serde(with = "BigArray")]
    pub AppID: TThostFtdcAppIDType,
    #[doc = "App类型"]
    pub AppType: TThostFtdcAppTypeType,
}
impl Default for CThostFtdcRspAuthenticateField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "客户端认证信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcAuthenticationInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "认证信息"]
    #[serde(with = "BigArray")]
    pub AuthInfo: TThostFtdcAuthInfoType,
    #[doc = "是否为认证结果"]
    pub IsResult: TThostFtdcBoolType,
    #[doc = "App代码"]
    #[serde(with = "BigArray")]
    pub AppID: TThostFtdcAppIDType,
    #[doc = "App类型"]
    pub AppType: TThostFtdcAppTypeType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
}
impl Default for CThostFtdcAuthenticationInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户登录应答2"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspUserLogin2Field {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "登录成功时间"]
    pub LoginTime: TThostFtdcTimeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易系统名称"]
    #[serde(with = "BigArray")]
    pub SystemName: TThostFtdcSystemNameType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "最大报单引用"]
    pub MaxOrderRef: TThostFtdcOrderRefType,
    #[doc = "上期所时间"]
    pub SHFETime: TThostFtdcTimeType,
    #[doc = "大商所时间"]
    pub DCETime: TThostFtdcTimeType,
    #[doc = "郑商所时间"]
    pub CZCETime: TThostFtdcTimeType,
    #[doc = "中金所时间"]
    pub FFEXTime: TThostFtdcTimeType,
    #[doc = "能源中心时间"]
    pub INETime: TThostFtdcTimeType,
    #[doc = "随机串"]
    pub RandomString: TThostFtdcRandomStringType,
}
impl Default for CThostFtdcRspUserLogin2Field {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期转帐报文头"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTransferHeaderField {
    #[doc = "版本号，常量，1.0"]
    pub Version: TThostFtdcVersionType,
    #[doc = "交易代码，必填"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "交易日期，必填，格式：yyyymmdd"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间，必填，格式：hhmmss"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "发起方流水号，N/A"]
    pub TradeSerial: TThostFtdcTradeSerialType,
    #[doc = "期货公司代码，必填"]
    pub FutureID: TThostFtdcFutureIDType,
    #[doc = "银行代码，根据查询银行得到，必填"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码，根据查询银行得到，必填"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
    #[doc = "操作员，N/A"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "交易设备类型，N/A"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "记录数，N/A"]
    pub RecordNum: TThostFtdcRecordNumType,
    #[doc = "会话编号，N/A"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "请求编号，N/A"]
    pub RequestID: TThostFtdcRequestIDType,
}
#[doc = "银行资金转期货请求，TradeCode=202001"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTransferBankToFutureReqField {
    #[doc = "期货资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "密码标志"]
    pub FuturePwdFlag: TThostFtdcFuturePwdFlagType,
    #[doc = "密码"]
    pub FutureAccPwd: TThostFtdcFutureAccPwdType,
    #[doc = "转账金额"]
    pub TradeAmt: TThostFtdcMoneyType,
    #[doc = "客户手续费"]
    pub CustFee: TThostFtdcMoneyType,
    #[doc = "币种：RMB-人民币 USD-美圆 HKD-港元"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
#[doc = "银行资金转期货请求响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferBankToFutureRspField {
    #[doc = "响应代码"]
    pub RetCode: TThostFtdcRetCodeType,
    #[doc = "响应信息"]
    #[serde(with = "BigArray")]
    pub RetInfo: TThostFtdcRetInfoType,
    #[doc = "资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "转帐金额"]
    pub TradeAmt: TThostFtdcMoneyType,
    #[doc = "应收客户手续费"]
    pub CustFee: TThostFtdcMoneyType,
    #[doc = "币种"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
impl Default for CThostFtdcTransferBankToFutureRspField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期货资金转银行请求，TradeCode=202002"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTransferFutureToBankReqField {
    #[doc = "期货资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "密码标志"]
    pub FuturePwdFlag: TThostFtdcFuturePwdFlagType,
    #[doc = "密码"]
    pub FutureAccPwd: TThostFtdcFutureAccPwdType,
    #[doc = "转账金额"]
    pub TradeAmt: TThostFtdcMoneyType,
    #[doc = "客户手续费"]
    pub CustFee: TThostFtdcMoneyType,
    #[doc = "币种：RMB-人民币 USD-美圆 HKD-港元"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
#[doc = "期货资金转银行请求响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferFutureToBankRspField {
    #[doc = "响应代码"]
    pub RetCode: TThostFtdcRetCodeType,
    #[doc = "响应信息"]
    #[serde(with = "BigArray")]
    pub RetInfo: TThostFtdcRetInfoType,
    #[doc = "资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "转帐金额"]
    pub TradeAmt: TThostFtdcMoneyType,
    #[doc = "应收客户手续费"]
    pub CustFee: TThostFtdcMoneyType,
    #[doc = "币种"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
impl Default for CThostFtdcTransferFutureToBankRspField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询银行资金请求，TradeCode=204002"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTransferQryBankReqField {
    #[doc = "期货资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "密码标志"]
    pub FuturePwdFlag: TThostFtdcFuturePwdFlagType,
    #[doc = "密码"]
    pub FutureAccPwd: TThostFtdcFutureAccPwdType,
    #[doc = "币种：RMB-人民币 USD-美圆 HKD-港元"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
#[doc = "查询银行资金请求响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferQryBankRspField {
    #[doc = "响应代码"]
    pub RetCode: TThostFtdcRetCodeType,
    #[doc = "响应信息"]
    #[serde(with = "BigArray")]
    pub RetInfo: TThostFtdcRetInfoType,
    #[doc = "资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
    #[doc = "银行余额"]
    pub TradeAmt: TThostFtdcMoneyType,
    #[doc = "银行可用余额"]
    pub UseAmt: TThostFtdcMoneyType,
    #[doc = "银行可取余额"]
    pub FetchAmt: TThostFtdcMoneyType,
    #[doc = "币种"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
}
impl Default for CThostFtdcTransferQryBankRspField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询银行交易明细请求，TradeCode=204999"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTransferQryDetailReqField {
    #[doc = "期货资金账户"]
    pub FutureAccount: TThostFtdcAccountIDType,
}
#[doc = "查询银行交易明细请求响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferQryDetailRspField {
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "交易代码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "期货流水号"]
    pub FutureSerial: TThostFtdcTradeSerialNoType,
    #[doc = "期货公司代码"]
    pub FutureID: TThostFtdcFutureIDType,
    #[doc = "资金帐号"]
    pub FutureAccount: TThostFtdcFutureAccountType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcTradeSerialNoType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
    #[doc = "银行账号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "证件号码"]
    pub CertCode: TThostFtdcCertCodeType,
    #[doc = "货币代码"]
    pub CurrencyCode: TThostFtdcCurrencyCodeType,
    #[doc = "发生金额"]
    pub TxAmount: TThostFtdcMoneyType,
    #[doc = "有效标志"]
    pub Flag: TThostFtdcTransferValidFlagType,
}
impl Default for CThostFtdcTransferQryDetailRspField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "响应信息"]
#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspInfoField {
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl fmt::Display for CThostFtdcRspInfoField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.ErrorID, gbk(&self.ErrorMsg))
    }
}
impl fmt::Debug for CThostFtdcRspInfoField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.ErrorID, gbk(&self.ErrorMsg))
    }
}
impl Default for CThostFtdcRspInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcExchangeField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所名称"]
    #[serde(with = "BigArray")]
    pub ExchangeName: TThostFtdcExchangeNameType,
    #[doc = "交易所属性"]
    pub ExchangeProperty: TThostFtdcExchangePropertyType,
}
impl Default for CThostFtdcExchangeField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "产品"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcProductField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "产品名称"]
    pub ProductName: TThostFtdcProductNameType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "产品类型"]
    pub ProductClass: TThostFtdcProductClassType,
    #[doc = "合约数量乘数"]
    pub VolumeMultiple: TThostFtdcVolumeMultipleType,
    #[doc = "最小变动价位"]
    pub PriceTick: TThostFtdcPriceType,
    #[doc = "市价单最大下单量"]
    pub MaxMarketOrderVolume: TThostFtdcVolumeType,
    #[doc = "市价单最小下单量"]
    pub MinMarketOrderVolume: TThostFtdcVolumeType,
    #[doc = "限价单最大下单量"]
    pub MaxLimitOrderVolume: TThostFtdcVolumeType,
    #[doc = "限价单最小下单量"]
    pub MinLimitOrderVolume: TThostFtdcVolumeType,
    #[doc = "持仓类型"]
    pub PositionType: TThostFtdcPositionTypeType,
    #[doc = "持仓日期类型"]
    pub PositionDateType: TThostFtdcPositionDateTypeType,
    #[doc = "平仓处理类型"]
    pub CloseDealType: TThostFtdcCloseDealTypeType,
    #[doc = "交易币种类型"]
    pub TradeCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "质押资金可用范围"]
    pub MortgageFundUseRange: TThostFtdcMortgageFundUseRangeType,
    #[doc = "交易所产品代码"]
    pub ExchangeProductID: TThostFtdcInstrumentIDType,
    #[doc = "合约基础商品乘数"]
    pub UnderlyingMultiple: TThostFtdcUnderlyingMultipleType,
}
#[doc = "合约"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约名称"]
    pub InstrumentName: TThostFtdcInstrumentNameType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "产品类型"]
    pub ProductClass: TThostFtdcProductClassType,
    #[doc = "交割年份"]
    pub DeliveryYear: TThostFtdcYearType,
    #[doc = "交割月"]
    pub DeliveryMonth: TThostFtdcMonthType,
    #[doc = "市价单最大下单量"]
    pub MaxMarketOrderVolume: TThostFtdcVolumeType,
    #[doc = "市价单最小下单量"]
    pub MinMarketOrderVolume: TThostFtdcVolumeType,
    #[doc = "限价单最大下单量"]
    pub MaxLimitOrderVolume: TThostFtdcVolumeType,
    #[doc = "限价单最小下单量"]
    pub MinLimitOrderVolume: TThostFtdcVolumeType,
    #[doc = "合约数量乘数"]
    pub VolumeMultiple: TThostFtdcVolumeMultipleType,
    #[doc = "最小变动价位"]
    pub PriceTick: TThostFtdcPriceType,
    #[doc = "创建日"]
    pub CreateDate: TThostFtdcDateType,
    #[doc = "上市日"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "到期日"]
    pub ExpireDate: TThostFtdcDateType,
    #[doc = "开始交割日"]
    pub StartDelivDate: TThostFtdcDateType,
    #[doc = "结束交割日"]
    pub EndDelivDate: TThostFtdcDateType,
    #[doc = "合约生命周期状态"]
    pub InstLifePhase: TThostFtdcInstLifePhaseType,
    #[doc = "当前是否交易"]
    pub IsTrading: TThostFtdcBoolType,
    #[doc = "持仓类型"]
    pub PositionType: TThostFtdcPositionTypeType,
    #[doc = "持仓日期类型"]
    pub PositionDateType: TThostFtdcPositionDateTypeType,
    #[doc = "多头保证金率"]
    pub LongMarginRatio: TThostFtdcRatioType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatio: TThostFtdcRatioType,
    #[doc = "是否使用大额单边保证金算法"]
    pub MaxMarginSideAlgorithm: TThostFtdcMaxMarginSideAlgorithmType,
    #[doc = "基础商品代码"]
    pub UnderlyingInstrID: TThostFtdcInstrumentIDType,
    #[doc = "执行价"]
    pub StrikePrice: TThostFtdcPriceType,
    #[doc = "期权类型"]
    pub OptionsType: TThostFtdcOptionsTypeType,
    #[doc = "合约基础商品乘数"]
    pub UnderlyingMultiple: TThostFtdcUnderlyingMultipleType,
    #[doc = "组合类型"]
    pub CombinationType: TThostFtdcCombinationTypeType,
}
#[doc = "经纪公司"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBrokerField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "经纪公司简称"]
    pub BrokerAbbr: TThostFtdcBrokerAbbrType,
    #[doc = "经纪公司名称"]
    #[serde(with = "BigArray")]
    pub BrokerName: TThostFtdcBrokerNameType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
}
impl Default for CThostFtdcBrokerField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所交易员"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTraderField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装数量"]
    pub InstallCount: TThostFtdcInstallCountType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
impl Default for CThostFtdcTraderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "投资者"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInvestorField {
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者分组代码"]
    pub InvestorGroupID: TThostFtdcInvestorIDType,
    #[doc = "投资者名称"]
    #[serde(with = "BigArray")]
    pub InvestorName: TThostFtdcPartyNameType,
    #[doc = "证件类型"]
    pub IdentifiedCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "联系电话"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "通讯地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "开户日期"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "手机"]
    #[serde(with = "BigArray")]
    pub Mobile: TThostFtdcMobileType,
    #[doc = "手续费率模板代码"]
    pub CommModelID: TThostFtdcInvestorIDType,
    #[doc = "保证金率模板代码"]
    pub MarginModelID: TThostFtdcInvestorIDType,
}
impl Default for CThostFtdcInvestorField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易编码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTradingCodeField {
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "交易编码类型"]
    pub ClientIDType: TThostFtdcClientIDTypeType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "业务类型"]
    pub BizType: TThostFtdcBizTypeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "会员编码和经纪公司编码对照表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcPartBrokerField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
}
#[doc = "管理用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcSuperUserField {
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户名称"]
    #[serde(with = "BigArray")]
    pub UserName: TThostFtdcUserNameType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
}
impl Default for CThostFtdcSuperUserField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "管理用户功能权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSuperUserFunctionField {
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "功能代码"]
    pub FunctionCode: TThostFtdcFunctionCodeType,
}
#[doc = "投资者组"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInvestorGroupField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者分组代码"]
    pub InvestorGroupID: TThostFtdcInvestorIDType,
    #[doc = "投资者分组名称"]
    #[serde(with = "BigArray")]
    pub InvestorGroupName: TThostFtdcInvestorGroupNameType,
}
impl Default for CThostFtdcInvestorGroupField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "资金账户"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingAccountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "上次质押金额"]
    pub PreMortgage: TThostFtdcMoneyType,
    #[doc = "上次信用额度"]
    pub PreCredit: TThostFtdcMoneyType,
    #[doc = "上次存款额"]
    pub PreDeposit: TThostFtdcMoneyType,
    #[doc = "上次结算准备金"]
    pub PreBalance: TThostFtdcMoneyType,
    #[doc = "上次占用的保证金"]
    pub PreMargin: TThostFtdcMoneyType,
    #[doc = "利息基数"]
    pub InterestBase: TThostFtdcMoneyType,
    #[doc = "利息收入"]
    pub Interest: TThostFtdcMoneyType,
    #[doc = "入金金额"]
    pub Deposit: TThostFtdcMoneyType,
    #[doc = "出金金额"]
    pub Withdraw: TThostFtdcMoneyType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
    #[doc = "冻结的资金"]
    pub FrozenCash: TThostFtdcMoneyType,
    #[doc = "冻结的手续费"]
    pub FrozenCommission: TThostFtdcMoneyType,
    #[doc = "当前保证金总额"]
    pub CurrMargin: TThostFtdcMoneyType,
    #[doc = "资金差额"]
    pub CashIn: TThostFtdcMoneyType,
    #[doc = "手续费"]
    pub Commission: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "持仓盈亏"]
    pub PositionProfit: TThostFtdcMoneyType,
    #[doc = "期货结算准备金"]
    pub Balance: TThostFtdcMoneyType,
    #[doc = "可用资金"]
    pub Available: TThostFtdcMoneyType,
    #[doc = "可取资金"]
    pub WithdrawQuota: TThostFtdcMoneyType,
    #[doc = "基本准备金"]
    pub Reserve: TThostFtdcMoneyType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "信用额度"]
    pub Credit: TThostFtdcMoneyType,
    #[doc = "质押金额"]
    pub Mortgage: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchangeMargin: TThostFtdcMoneyType,
    #[doc = "投资者交割保证金"]
    pub DeliveryMargin: TThostFtdcMoneyType,
    #[doc = "交易所交割保证金"]
    pub ExchangeDeliveryMargin: TThostFtdcMoneyType,
    #[doc = "保底期货结算准备金"]
    pub ReserveBalance: TThostFtdcMoneyType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "上次货币质入金额"]
    pub PreFundMortgageIn: TThostFtdcMoneyType,
    #[doc = "上次货币质出金额"]
    pub PreFundMortgageOut: TThostFtdcMoneyType,
    #[doc = "货币质入金额"]
    pub FundMortgageIn: TThostFtdcMoneyType,
    #[doc = "货币质出金额"]
    pub FundMortgageOut: TThostFtdcMoneyType,
    #[doc = "货币质押余额"]
    pub FundMortgageAvailable: TThostFtdcMoneyType,
    #[doc = "可质押货币金额"]
    pub MortgageableFund: TThostFtdcMoneyType,
    #[doc = "特殊产品占用保证金"]
    pub SpecProductMargin: TThostFtdcMoneyType,
    #[doc = "特殊产品冻结保证金"]
    pub SpecProductFrozenMargin: TThostFtdcMoneyType,
    #[doc = "特殊产品手续费"]
    pub SpecProductCommission: TThostFtdcMoneyType,
    #[doc = "特殊产品冻结手续费"]
    pub SpecProductFrozenCommission: TThostFtdcMoneyType,
    #[doc = "特殊产品持仓盈亏"]
    pub SpecProductPositionProfit: TThostFtdcMoneyType,
    #[doc = "特殊产品平仓盈亏"]
    pub SpecProductCloseProfit: TThostFtdcMoneyType,
    #[doc = "根据持仓盈亏算法计算的特殊产品持仓盈亏"]
    pub SpecProductPositionProfitByAlg: TThostFtdcMoneyType,
    #[doc = "特殊产品交易所保证金"]
    pub SpecProductExchangeMargin: TThostFtdcMoneyType,
    #[doc = "业务类型"]
    pub BizType: TThostFtdcBizTypeType,
    #[doc = "延时换汇冻结金额"]
    pub FrozenSwap: TThostFtdcMoneyType,
    #[doc = "剩余换汇额度"]
    pub RemainSwap: TThostFtdcMoneyType,
}
#[doc = "投资者持仓"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorPositionField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "持仓多空方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "持仓日期"]
    pub PositionDate: TThostFtdcPositionDateType,
    #[doc = "上日持仓"]
    pub YdPosition: TThostFtdcVolumeType,
    #[doc = "今日持仓"]
    pub Position: TThostFtdcVolumeType,
    #[doc = "多头冻结"]
    pub LongFrozen: TThostFtdcVolumeType,
    #[doc = "空头冻结"]
    pub ShortFrozen: TThostFtdcVolumeType,
    #[doc = "开仓冻结金额"]
    pub LongFrozenAmount: TThostFtdcMoneyType,
    #[doc = "开仓冻结金额"]
    pub ShortFrozenAmount: TThostFtdcMoneyType,
    #[doc = "开仓量"]
    pub OpenVolume: TThostFtdcVolumeType,
    #[doc = "平仓量"]
    pub CloseVolume: TThostFtdcVolumeType,
    #[doc = "开仓金额"]
    pub OpenAmount: TThostFtdcMoneyType,
    #[doc = "平仓金额"]
    pub CloseAmount: TThostFtdcMoneyType,
    #[doc = "持仓成本"]
    pub PositionCost: TThostFtdcMoneyType,
    #[doc = "上次占用的保证金"]
    pub PreMargin: TThostFtdcMoneyType,
    #[doc = "占用的保证金"]
    pub UseMargin: TThostFtdcMoneyType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
    #[doc = "冻结的资金"]
    pub FrozenCash: TThostFtdcMoneyType,
    #[doc = "冻结的手续费"]
    pub FrozenCommission: TThostFtdcMoneyType,
    #[doc = "资金差额"]
    pub CashIn: TThostFtdcMoneyType,
    #[doc = "手续费"]
    pub Commission: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "持仓盈亏"]
    pub PositionProfit: TThostFtdcMoneyType,
    #[doc = "上次结算价"]
    pub PreSettlementPrice: TThostFtdcPriceType,
    #[doc = "本次结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "开仓成本"]
    pub OpenCost: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchangeMargin: TThostFtdcMoneyType,
    #[doc = "组合成交形成的持仓"]
    pub CombPosition: TThostFtdcVolumeType,
    #[doc = "组合多头冻结"]
    pub CombLongFrozen: TThostFtdcVolumeType,
    #[doc = "组合空头冻结"]
    pub CombShortFrozen: TThostFtdcVolumeType,
    #[doc = "逐日盯市平仓盈亏"]
    pub CloseProfitByDate: TThostFtdcMoneyType,
    #[doc = "逐笔对冲平仓盈亏"]
    pub CloseProfitByTrade: TThostFtdcMoneyType,
    #[doc = "今日持仓"]
    pub TodayPosition: TThostFtdcVolumeType,
    #[doc = "保证金率"]
    pub MarginRateByMoney: TThostFtdcRatioType,
    #[doc = "保证金率(按手数)"]
    pub MarginRateByVolume: TThostFtdcRatioType,
    #[doc = "执行冻结"]
    pub StrikeFrozen: TThostFtdcVolumeType,
    #[doc = "执行冻结金额"]
    pub StrikeFrozenAmount: TThostFtdcMoneyType,
    #[doc = "放弃执行冻结"]
    pub AbandonFrozen: TThostFtdcVolumeType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行冻结的昨仓"]
    pub YdStrikeFrozen: TThostFtdcVolumeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[cfg(not(feature = "v6_3_13"))]
    #[doc = "大商所持仓成本差值，只有大商所使用"]
    pub PositionCostOffset: TThostFtdcMoneyType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "tas持仓手数"]
    pub TasPosition: TThostFtdcVolumeType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "tas持仓成本"]
    pub TasPositionCost: TThostFtdcMoneyType,
}
#[doc = "合约保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentMarginRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "是否相对交易所收取"]
    pub IsRelative: TThostFtdcBoolType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "合约手续费率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentCommissionRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "开仓手续费率"]
    pub OpenRatioByMoney: TThostFtdcRatioType,
    #[doc = "开仓手续费"]
    pub OpenRatioByVolume: TThostFtdcRatioType,
    #[doc = "平仓手续费率"]
    pub CloseRatioByMoney: TThostFtdcRatioType,
    #[doc = "平仓手续费"]
    pub CloseRatioByVolume: TThostFtdcRatioType,
    #[doc = "平今手续费率"]
    pub CloseTodayRatioByMoney: TThostFtdcRatioType,
    #[doc = "平今手续费"]
    pub CloseTodayRatioByVolume: TThostFtdcRatioType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "业务类型"]
    pub BizType: TThostFtdcBizTypeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "深度行情"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcDepthMarketDataField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "最新价"]
    pub LastPrice: TThostFtdcPriceType,
    #[doc = "上次结算价"]
    pub PreSettlementPrice: TThostFtdcPriceType,
    #[doc = "昨收盘"]
    pub PreClosePrice: TThostFtdcPriceType,
    #[doc = "昨持仓量"]
    pub PreOpenInterest: TThostFtdcLargeVolumeType,
    #[doc = "今开盘"]
    pub OpenPrice: TThostFtdcPriceType,
    #[doc = "最高价"]
    pub HighestPrice: TThostFtdcPriceType,
    #[doc = "最低价"]
    pub LowestPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "成交金额"]
    pub Turnover: TThostFtdcMoneyType,
    #[doc = "持仓量"]
    pub OpenInterest: TThostFtdcLargeVolumeType,
    #[doc = "今收盘"]
    pub ClosePrice: TThostFtdcPriceType,
    #[doc = "本次结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "涨停板价"]
    pub UpperLimitPrice: TThostFtdcPriceType,
    #[doc = "跌停板价"]
    pub LowerLimitPrice: TThostFtdcPriceType,
    #[doc = "昨虚实度"]
    pub PreDelta: TThostFtdcRatioType,
    #[doc = "今虚实度"]
    pub CurrDelta: TThostFtdcRatioType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "最后修改毫秒"]
    pub UpdateMillisec: TThostFtdcMillisecType,
    #[doc = "申买价一"]
    pub BidPrice1: TThostFtdcPriceType,
    #[doc = "申买量一"]
    pub BidVolume1: TThostFtdcVolumeType,
    #[doc = "申卖价一"]
    pub AskPrice1: TThostFtdcPriceType,
    #[doc = "申卖量一"]
    pub AskVolume1: TThostFtdcVolumeType,
    #[doc = "申买价二"]
    pub BidPrice2: TThostFtdcPriceType,
    #[doc = "申买量二"]
    pub BidVolume2: TThostFtdcVolumeType,
    #[doc = "申卖价二"]
    pub AskPrice2: TThostFtdcPriceType,
    #[doc = "申卖量二"]
    pub AskVolume2: TThostFtdcVolumeType,
    #[doc = "申买价三"]
    pub BidPrice3: TThostFtdcPriceType,
    #[doc = "申买量三"]
    pub BidVolume3: TThostFtdcVolumeType,
    #[doc = "申卖价三"]
    pub AskPrice3: TThostFtdcPriceType,
    #[doc = "申卖量三"]
    pub AskVolume3: TThostFtdcVolumeType,
    #[doc = "申买价四"]
    pub BidPrice4: TThostFtdcPriceType,
    #[doc = "申买量四"]
    pub BidVolume4: TThostFtdcVolumeType,
    #[doc = "申卖价四"]
    pub AskPrice4: TThostFtdcPriceType,
    #[doc = "申卖量四"]
    pub AskVolume4: TThostFtdcVolumeType,
    #[doc = "申买价五"]
    pub BidPrice5: TThostFtdcPriceType,
    #[doc = "申买量五"]
    pub BidVolume5: TThostFtdcVolumeType,
    #[doc = "申卖价五"]
    pub AskPrice5: TThostFtdcPriceType,
    #[doc = "申卖量五"]
    pub AskVolume5: TThostFtdcVolumeType,
    #[doc = "当日均价"]
    pub AveragePrice: TThostFtdcPriceType,
    #[doc = "业务日期"]
    pub ActionDay: TThostFtdcDateType,
}
#[doc = "投资者合约交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentTradingRightField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易权限"]
    pub TradingRight: TThostFtdcTradingRightType,
}
#[doc = "经纪公司用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户名称"]
    #[serde(with = "BigArray")]
    pub UserName: TThostFtdcUserNameType,
    #[doc = "用户类型"]
    pub UserType: TThostFtdcUserTypeType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "是否使用令牌"]
    pub IsUsingOTP: TThostFtdcBoolType,
    #[doc = "是否强制终端认证"]
    pub IsAuthForce: TThostFtdcBoolType,
}
impl Default for CThostFtdcBrokerUserField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "经纪公司用户口令"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserPasswordField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "上次修改时间"]
    pub LastUpdateTime: TThostFtdcDateTimeType,
    #[doc = "上次登陆时间"]
    pub LastLoginTime: TThostFtdcDateTimeType,
    #[doc = "密码过期时间"]
    pub ExpireDate: TThostFtdcDateType,
    #[doc = "弱密码过期时间"]
    pub WeakExpireDate: TThostFtdcDateType,
}
impl Default for CThostFtdcBrokerUserPasswordField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "经纪公司用户功能权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserFunctionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "经纪公司功能代码"]
    pub BrokerFunctionCode: TThostFtdcBrokerFunctionCodeType,
}
#[doc = "交易所交易员报盘机"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTraderOfferField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所交易员连接状态"]
    pub TraderConnectStatus: TThostFtdcTraderConnectStatusType,
    #[doc = "发出连接请求的日期"]
    pub ConnectRequestDate: TThostFtdcDateType,
    #[doc = "发出连接请求的时间"]
    pub ConnectRequestTime: TThostFtdcTimeType,
    #[doc = "上次报告日期"]
    pub LastReportDate: TThostFtdcDateType,
    #[doc = "上次报告时间"]
    pub LastReportTime: TThostFtdcTimeType,
    #[doc = "完成连接日期"]
    pub ConnectDate: TThostFtdcDateType,
    #[doc = "完成连接时间"]
    pub ConnectTime: TThostFtdcTimeType,
    #[doc = "启动日期"]
    pub StartDate: TThostFtdcDateType,
    #[doc = "启动时间"]
    pub StartTime: TThostFtdcTimeType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "本席位最大成交编号"]
    pub MaxTradeID: TThostFtdcTradeIDType,
    #[doc = "本席位最大报单备拷"]
    pub MaxOrderMessageReference: TThostFtdcReturnCodeType,
}
impl Default for CThostFtdcTraderOfferField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "投资者结算结果"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcSettlementInfoField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "消息正文"]
    #[serde(with = "BigArray")]
    pub Content: TThostFtdcContentType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcSettlementInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "合约保证金率调整"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentMarginRateAdjustField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "是否相对交易所收取"]
    pub IsRelative: TThostFtdcBoolType,
}
#[doc = "交易所保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeMarginRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "交易所保证金率调整"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeMarginRateAdjustField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "跟随交易所投资者多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "跟随交易所投资者多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "跟随交易所投资者空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "跟随交易所投资者空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "交易所多头保证金率"]
    pub ExchLongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "交易所多头保证金费"]
    pub ExchLongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "交易所空头保证金率"]
    pub ExchShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "交易所空头保证金费"]
    pub ExchShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "不跟随交易所投资者多头保证金率"]
    pub NoLongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "不跟随交易所投资者多头保证金费"]
    pub NoLongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "不跟随交易所投资者空头保证金率"]
    pub NoShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "不跟随交易所投资者空头保证金费"]
    pub NoShortMarginRatioByVolume: TThostFtdcMoneyType,
}
#[doc = "汇率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "源币种"]
    pub FromCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "源币种单位数量"]
    pub FromCurrencyUnit: TThostFtdcCurrencyUnitType,
    #[doc = "目标币种"]
    pub ToCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇率"]
    pub ExchangeRate: TThostFtdcExchangeRateType,
}
#[doc = "结算引用"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSettlementRefField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
}
#[doc = "当前时间"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCurrentTimeField {
    #[doc = "当前日期"]
    pub CurrDate: TThostFtdcDateType,
    #[doc = "当前时间"]
    pub CurrTime: TThostFtdcTimeType,
    #[doc = "当前时间（毫秒）"]
    pub CurrMillisec: TThostFtdcMillisecType,
    #[doc = "业务日期"]
    pub ActionDay: TThostFtdcDateType,
}
#[doc = "通讯阶段"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCommPhaseField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "通讯时段编号"]
    pub CommPhaseNo: TThostFtdcCommPhaseNoType,
    #[doc = "系统编号"]
    pub SystemID: TThostFtdcSystemIDType,
}
#[doc = "登录信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcLoginInfoField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "登录日期"]
    pub LoginDate: TThostFtdcDateType,
    #[doc = "登录时间"]
    pub LoginTime: TThostFtdcTimeType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "系统名称"]
    #[serde(with = "BigArray")]
    pub SystemName: TThostFtdcSystemNameType,
    #[doc = "密码,已弃用"]
    #[serde(with = "BigArray")]
    pub PasswordDeprecated: TThostFtdcPasswordType,
    #[doc = "最大报单引用"]
    pub MaxOrderRef: TThostFtdcOrderRefType,
    #[doc = "上期所时间"]
    pub SHFETime: TThostFtdcTimeType,
    #[doc = "大商所时间"]
    pub DCETime: TThostFtdcTimeType,
    #[doc = "郑商所时间"]
    pub CZCETime: TThostFtdcTimeType,
    #[doc = "中金所时间"]
    pub FFEXTime: TThostFtdcTimeType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "动态密码"]
    #[serde(with = "BigArray")]
    pub OneTimePassword: TThostFtdcPasswordType,
    #[doc = "能源中心时间"]
    pub INETime: TThostFtdcTimeType,
    #[doc = "查询时是否需要流控"]
    pub IsQryControl: TThostFtdcBoolType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
}
impl Default for CThostFtdcLoginInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "登录信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcLogoutAllField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "系统名称"]
    #[serde(with = "BigArray")]
    pub SystemName: TThostFtdcSystemNameType,
}
impl Default for CThostFtdcLogoutAllField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "前置状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcFrontStatusField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "上次报告日期"]
    pub LastReportDate: TThostFtdcDateType,
    #[doc = "上次报告时间"]
    pub LastReportTime: TThostFtdcTimeType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
}
#[doc = "用户口令变更"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcUserPasswordUpdateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "原来的口令"]
    #[serde(with = "BigArray")]
    pub OldPassword: TThostFtdcPasswordType,
    #[doc = "新的口令"]
    #[serde(with = "BigArray")]
    pub NewPassword: TThostFtdcPasswordType,
}
impl Default for CThostFtdcUserPasswordUpdateField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "输入报单"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInputOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "用户强评标志"]
    pub UserForceClose: TThostFtdcBoolType,
    #[doc = "互换单标志"]
    pub IsSwapOrder: TThostFtdcBoolType,
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
#[doc = "报单"]
#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CThostFtdcOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报单提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单来源"]
    pub OrderSource: TThostFtdcOrderSourceType,
    #[doc = "报单状态"]
    pub OrderStatus: TThostFtdcOrderStatusType,
    #[doc = "报单类型"]
    pub OrderType: TThostFtdcOrderTypeType,
    #[doc = "今成交数量"]
    pub VolumeTraded: TThostFtdcVolumeType,
    #[doc = "剩余数量"]
    pub VolumeTotal: TThostFtdcVolumeType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "委托时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "激活时间"]
    pub ActiveTime: TThostFtdcTimeType,
    #[doc = "挂起时间"]
    pub SuspendTime: TThostFtdcTimeType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "最后修改交易所交易员代码"]
    pub ActiveTraderID: TThostFtdcTraderIDType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "用户强评标志"]
    pub UserForceClose: TThostFtdcBoolType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司报单编号"]
    pub BrokerOrderSeq: TThostFtdcSequenceNoType,
    #[doc = "相关报单"]
    pub RelativeOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "郑商所成交数量"]
    pub ZCETotalTradedVolume: TThostFtdcVolumeType,
    #[doc = "互换单标志"]
    pub IsSwapOrder: TThostFtdcBoolType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOrderField {
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报单提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单来源"]
    pub OrderSource: TThostFtdcOrderSourceType,
    #[doc = "报单状态"]
    pub OrderStatus: TThostFtdcOrderStatusType,
    #[doc = "报单类型"]
    pub OrderType: TThostFtdcOrderTypeType,
    #[doc = "今成交数量"]
    pub VolumeTraded: TThostFtdcVolumeType,
    #[doc = "剩余数量"]
    pub VolumeTotal: TThostFtdcVolumeType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "委托时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "激活时间"]
    pub ActiveTime: TThostFtdcTimeType,
    #[doc = "挂起时间"]
    pub SuspendTime: TThostFtdcTimeType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "最后修改交易所交易员代码"]
    pub ActiveTraderID: TThostFtdcTraderIDType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所报单插入失败"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOrderInsertErrorField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcExchangeOrderInsertErrorField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "输入报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量变化"]
    pub VolumeChange: TThostFtdcVolumeType,
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
#[doc = "报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量变化"]
    pub VolumeChange: TThostFtdcVolumeType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOrderActionField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量变化"]
    pub VolumeChange: TThostFtdcVolumeType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所报单操作失败"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOrderActionErrorField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcExchangeOrderActionErrorField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所成交"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeTradeField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "成交编号"]
    pub TradeID: TThostFtdcTradeIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易角色"]
    pub TradingRole: TThostFtdcTradingRoleType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "价格"]
    pub Price: TThostFtdcPriceType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "成交时期"]
    pub TradeDate: TThostFtdcDateType,
    #[doc = "成交时间"]
    pub TradeTime: TThostFtdcTimeType,
    #[doc = "成交类型"]
    pub TradeType: TThostFtdcTradeTypeType,
    #[doc = "成交价来源"]
    pub PriceSource: TThostFtdcPriceSourceType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "成交来源"]
    pub TradeSource: TThostFtdcTradeSourceType,
}
#[doc = "成交"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "成交编号"]
    pub TradeID: TThostFtdcTradeIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易角色"]
    pub TradingRole: TThostFtdcTradingRoleType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "价格"]
    pub Price: TThostFtdcPriceType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "成交时期"]
    pub TradeDate: TThostFtdcDateType,
    #[doc = "成交时间"]
    pub TradeTime: TThostFtdcTimeType,
    #[doc = "成交类型"]
    pub TradeType: TThostFtdcTradeTypeType,
    #[doc = "成交价来源"]
    pub PriceSource: TThostFtdcPriceSourceType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "经纪公司报单编号"]
    pub BrokerOrderSeq: TThostFtdcSequenceNoType,
    #[doc = "成交来源"]
    pub TradeSource: TThostFtdcTradeSourceType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "用户会话"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcUserSessionField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "登录日期"]
    pub LoginDate: TThostFtdcDateType,
    #[doc = "登录时间"]
    pub LoginTime: TThostFtdcTimeType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
}
impl Default for CThostFtdcUserSessionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询最大报单数量"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQueryMaxOrderVolumeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "最大允许报单数量"]
    pub MaxVolume: TThostFtdcVolumeType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "投资者结算结果确认信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSettlementInfoConfirmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "确认日期"]
    pub ConfirmDate: TThostFtdcDateType,
    #[doc = "确认时间"]
    pub ConfirmTime: TThostFtdcTimeType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}

#[doc = "出入金同步"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncDepositField {
    #[doc = "出入金流水号"]
    pub DepositSeqNo: TThostFtdcDepositSeqNoType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "入金金额"]
    pub Deposit: TThostFtdcMoneyType,
    #[doc = "是否强制进行"]
    pub IsForce: TThostFtdcBoolType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "货币质押同步"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncFundMortgageField {
    #[doc = "货币质押流水号"]
    pub MortgageSeqNo: TThostFtdcDepositSeqNoType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "源币种"]
    pub FromCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "质押金额"]
    pub MortgageAmount: TThostFtdcMoneyType,
    #[doc = "目标币种"]
    pub ToCurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "经纪公司同步"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerSyncField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "正在同步中的投资者"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInvestorField {
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者分组代码"]
    pub InvestorGroupID: TThostFtdcInvestorIDType,
    #[doc = "投资者名称"]
    #[serde(with = "BigArray")]
    pub InvestorName: TThostFtdcPartyNameType,
    #[doc = "证件类型"]
    pub IdentifiedCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "联系电话"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "通讯地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "开户日期"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "手机"]
    #[serde(with = "BigArray")]
    pub Mobile: TThostFtdcMobileType,
    #[doc = "手续费率模板代码"]
    pub CommModelID: TThostFtdcInvestorIDType,
    #[doc = "保证金率模板代码"]
    pub MarginModelID: TThostFtdcInvestorIDType,
}
impl Default for CThostFtdcSyncingInvestorField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "正在同步中的交易代码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingTradingCodeField {
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "交易编码类型"]
    pub ClientIDType: TThostFtdcClientIDTypeType,
}
#[doc = "正在同步中的投资者分组"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInvestorGroupField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者分组代码"]
    pub InvestorGroupID: TThostFtdcInvestorIDType,
    #[doc = "投资者分组名称"]
    #[serde(with = "BigArray")]
    pub InvestorGroupName: TThostFtdcInvestorGroupNameType,
}
impl Default for CThostFtdcSyncingInvestorGroupField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "正在同步中的交易账号"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingTradingAccountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "上次质押金额"]
    pub PreMortgage: TThostFtdcMoneyType,
    #[doc = "上次信用额度"]
    pub PreCredit: TThostFtdcMoneyType,
    #[doc = "上次存款额"]
    pub PreDeposit: TThostFtdcMoneyType,
    #[doc = "上次结算准备金"]
    pub PreBalance: TThostFtdcMoneyType,
    #[doc = "上次占用的保证金"]
    pub PreMargin: TThostFtdcMoneyType,
    #[doc = "利息基数"]
    pub InterestBase: TThostFtdcMoneyType,
    #[doc = "利息收入"]
    pub Interest: TThostFtdcMoneyType,
    #[doc = "入金金额"]
    pub Deposit: TThostFtdcMoneyType,
    #[doc = "出金金额"]
    pub Withdraw: TThostFtdcMoneyType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
    #[doc = "冻结的资金"]
    pub FrozenCash: TThostFtdcMoneyType,
    #[doc = "冻结的手续费"]
    pub FrozenCommission: TThostFtdcMoneyType,
    #[doc = "当前保证金总额"]
    pub CurrMargin: TThostFtdcMoneyType,
    #[doc = "资金差额"]
    pub CashIn: TThostFtdcMoneyType,
    #[doc = "手续费"]
    pub Commission: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "持仓盈亏"]
    pub PositionProfit: TThostFtdcMoneyType,
    #[doc = "期货结算准备金"]
    pub Balance: TThostFtdcMoneyType,
    #[doc = "可用资金"]
    pub Available: TThostFtdcMoneyType,
    #[doc = "可取资金"]
    pub WithdrawQuota: TThostFtdcMoneyType,
    #[doc = "基本准备金"]
    pub Reserve: TThostFtdcMoneyType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "信用额度"]
    pub Credit: TThostFtdcMoneyType,
    #[doc = "质押金额"]
    pub Mortgage: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchangeMargin: TThostFtdcMoneyType,
    #[doc = "投资者交割保证金"]
    pub DeliveryMargin: TThostFtdcMoneyType,
    #[doc = "交易所交割保证金"]
    pub ExchangeDeliveryMargin: TThostFtdcMoneyType,
    #[doc = "保底期货结算准备金"]
    pub ReserveBalance: TThostFtdcMoneyType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "上次货币质入金额"]
    pub PreFundMortgageIn: TThostFtdcMoneyType,
    #[doc = "上次货币质出金额"]
    pub PreFundMortgageOut: TThostFtdcMoneyType,
    #[doc = "货币质入金额"]
    pub FundMortgageIn: TThostFtdcMoneyType,
    #[doc = "货币质出金额"]
    pub FundMortgageOut: TThostFtdcMoneyType,
    #[doc = "货币质押余额"]
    pub FundMortgageAvailable: TThostFtdcMoneyType,
    #[doc = "可质押货币金额"]
    pub MortgageableFund: TThostFtdcMoneyType,
    #[doc = "特殊产品占用保证金"]
    pub SpecProductMargin: TThostFtdcMoneyType,
    #[doc = "特殊产品冻结保证金"]
    pub SpecProductFrozenMargin: TThostFtdcMoneyType,
    #[doc = "特殊产品手续费"]
    pub SpecProductCommission: TThostFtdcMoneyType,
    #[doc = "特殊产品冻结手续费"]
    pub SpecProductFrozenCommission: TThostFtdcMoneyType,
    #[doc = "特殊产品持仓盈亏"]
    pub SpecProductPositionProfit: TThostFtdcMoneyType,
    #[doc = "特殊产品平仓盈亏"]
    pub SpecProductCloseProfit: TThostFtdcMoneyType,
    #[doc = "根据持仓盈亏算法计算的特殊产品持仓盈亏"]
    pub SpecProductPositionProfitByAlg: TThostFtdcMoneyType,
    #[doc = "特殊产品交易所保证金"]
    pub SpecProductExchangeMargin: TThostFtdcMoneyType,
    #[doc = "延时换汇冻结金额"]
    pub FrozenSwap: TThostFtdcMoneyType,
    #[doc = "剩余换汇额度"]
    pub RemainSwap: TThostFtdcMoneyType,
}
#[doc = "正在同步中的投资者持仓"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInvestorPositionField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "持仓多空方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "持仓日期"]
    pub PositionDate: TThostFtdcPositionDateType,
    #[doc = "上日持仓"]
    pub YdPosition: TThostFtdcVolumeType,
    #[doc = "今日持仓"]
    pub Position: TThostFtdcVolumeType,
    #[doc = "多头冻结"]
    pub LongFrozen: TThostFtdcVolumeType,
    #[doc = "空头冻结"]
    pub ShortFrozen: TThostFtdcVolumeType,
    #[doc = "开仓冻结金额"]
    pub LongFrozenAmount: TThostFtdcMoneyType,
    #[doc = "开仓冻结金额"]
    pub ShortFrozenAmount: TThostFtdcMoneyType,
    #[doc = "开仓量"]
    pub OpenVolume: TThostFtdcVolumeType,
    #[doc = "平仓量"]
    pub CloseVolume: TThostFtdcVolumeType,
    #[doc = "开仓金额"]
    pub OpenAmount: TThostFtdcMoneyType,
    #[doc = "平仓金额"]
    pub CloseAmount: TThostFtdcMoneyType,
    #[doc = "持仓成本"]
    pub PositionCost: TThostFtdcMoneyType,
    #[doc = "上次占用的保证金"]
    pub PreMargin: TThostFtdcMoneyType,
    #[doc = "占用的保证金"]
    pub UseMargin: TThostFtdcMoneyType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
    #[doc = "冻结的资金"]
    pub FrozenCash: TThostFtdcMoneyType,
    #[doc = "冻结的手续费"]
    pub FrozenCommission: TThostFtdcMoneyType,
    #[doc = "资金差额"]
    pub CashIn: TThostFtdcMoneyType,
    #[doc = "手续费"]
    pub Commission: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "持仓盈亏"]
    pub PositionProfit: TThostFtdcMoneyType,
    #[doc = "上次结算价"]
    pub PreSettlementPrice: TThostFtdcPriceType,
    #[doc = "本次结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "开仓成本"]
    pub OpenCost: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchangeMargin: TThostFtdcMoneyType,
    #[doc = "组合成交形成的持仓"]
    pub CombPosition: TThostFtdcVolumeType,
    #[doc = "组合多头冻结"]
    pub CombLongFrozen: TThostFtdcVolumeType,
    #[doc = "组合空头冻结"]
    pub CombShortFrozen: TThostFtdcVolumeType,
    #[doc = "逐日盯市平仓盈亏"]
    pub CloseProfitByDate: TThostFtdcMoneyType,
    #[doc = "逐笔对冲平仓盈亏"]
    pub CloseProfitByTrade: TThostFtdcMoneyType,
    #[doc = "今日持仓"]
    pub TodayPosition: TThostFtdcVolumeType,
    #[doc = "保证金率"]
    pub MarginRateByMoney: TThostFtdcRatioType,
    #[doc = "保证金率(按手数)"]
    pub MarginRateByVolume: TThostFtdcRatioType,
    #[doc = "执行冻结"]
    pub StrikeFrozen: TThostFtdcVolumeType,
    #[doc = "执行冻结金额"]
    pub StrikeFrozenAmount: TThostFtdcMoneyType,
    #[doc = "放弃执行冻结"]
    pub AbandonFrozen: TThostFtdcVolumeType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行冻结的昨仓"]
    pub YdStrikeFrozen: TThostFtdcVolumeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[cfg(not(feature = "v6_3_13"))]
    #[doc = "大商所持仓成本差值，只有大商所使用"]
    pub PositionCostOffset: TThostFtdcMoneyType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "tas持仓手数"]
    pub TasPosition: TThostFtdcVolumeType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "tas持仓成本"]
    pub TasPositionCost: TThostFtdcMoneyType,
}
#[doc = "正在同步中的合约保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInstrumentMarginRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "是否相对交易所收取"]
    pub IsRelative: TThostFtdcBoolType,
}
#[doc = "正在同步中的合约手续费率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInstrumentCommissionRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "开仓手续费率"]
    pub OpenRatioByMoney: TThostFtdcRatioType,
    #[doc = "开仓手续费"]
    pub OpenRatioByVolume: TThostFtdcRatioType,
    #[doc = "平仓手续费率"]
    pub CloseRatioByMoney: TThostFtdcRatioType,
    #[doc = "平仓手续费"]
    pub CloseRatioByVolume: TThostFtdcRatioType,
    #[doc = "平今手续费率"]
    pub CloseTodayRatioByMoney: TThostFtdcRatioType,
    #[doc = "平今手续费"]
    pub CloseTodayRatioByVolume: TThostFtdcRatioType,
}
#[doc = "正在同步中的合约交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncingInstrumentTradingRightField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易权限"]
    pub TradingRight: TThostFtdcTradingRightType,
}
#[doc = "查询报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询成交"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "查询投资者持仓"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "查询资金账户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTradingAccountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "业务类型"]
    pub BizType: TThostFtdcBizTypeType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
}
#[doc = "查询投资者"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInvestorField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "查询交易编码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTradingCodeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易编码类型"]
    pub ClientIDType: TThostFtdcClientIDTypeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询投资者组"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInvestorGroupField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "查询合约保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInstrumentMarginRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询手续费率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "查询合约交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInstrumentTradingRightField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "查询经纪公司"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "查询交易员"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTraderField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "查询管理用户功能权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySuperUserFunctionField {
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "查询用户会话"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryUserSessionField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "查询经纪公司会员代码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryPartBrokerField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
}
#[doc = "查询前置状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryFrontStatusField {
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
}
#[doc = "查询交易所报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeOrderField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "查询报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询交易所报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeOrderActionField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "查询管理用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySuperUserField {
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "查询交易所"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询产品"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryProductField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "产品类型"]
    pub ProductClass: TThostFtdcProductClassType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询合约"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "查询行情"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryDepthMarketDataField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询经纪公司用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerUserField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "查询经纪公司用户权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerUserFunctionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "查询交易员报盘机"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTraderOfferField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "查询出入金流水"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySyncDepositField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "出入金流水号"]
    pub DepositSeqNo: TThostFtdcDepositSeqNoType,
}
#[doc = "查询投资者结算结果"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySettlementInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "查询交易所保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeMarginRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询交易所调整保证金率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeMarginRateAdjustField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
}
#[doc = "查询汇率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "源币种"]
    pub FromCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "目标币种"]
    pub ToCurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "查询货币质押流水"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySyncFundMortgageField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "货币质押流水号"]
    pub MortgageSeqNo: TThostFtdcDepositSeqNoType,
}
#[doc = "查询报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryHisOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
}
#[doc = "当前期权合约最小保证金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrMiniMarginField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "单位（手）期权合约最小保证金"]
    pub MinMargin: TThostFtdcMoneyType,
    #[doc = "取值方式"]
    pub ValueMethod: TThostFtdcValueMethodType,
    #[doc = "是否跟随交易所收取"]
    pub IsRelative: TThostFtdcBoolType,
}
#[doc = "当前期权合约保证金调整系数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrMarginAdjustField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机空头保证金调整系数"]
    pub SShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "投机空头保证金调整系数"]
    pub SShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "保值空头保证金调整系数"]
    pub HShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "保值空头保证金调整系数"]
    pub HShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "套利空头保证金调整系数"]
    pub AShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "套利空头保证金调整系数"]
    pub AShortMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "是否跟随交易所收取"]
    pub IsRelative: TThostFtdcBoolType,
    #[doc = "做市商空头保证金调整系数"]
    pub MShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "做市商空头保证金调整系数"]
    pub MShortMarginRatioByVolume: TThostFtdcMoneyType,
}
#[doc = "当前期权合约手续费的详细内容"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrCommRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "开仓手续费率"]
    pub OpenRatioByMoney: TThostFtdcRatioType,
    #[doc = "开仓手续费"]
    pub OpenRatioByVolume: TThostFtdcRatioType,
    #[doc = "平仓手续费率"]
    pub CloseRatioByMoney: TThostFtdcRatioType,
    #[doc = "平仓手续费"]
    pub CloseRatioByVolume: TThostFtdcRatioType,
    #[doc = "平今手续费率"]
    pub CloseTodayRatioByMoney: TThostFtdcRatioType,
    #[doc = "平今手续费"]
    pub CloseTodayRatioByVolume: TThostFtdcRatioType,
    #[doc = "执行手续费率"]
    pub StrikeRatioByMoney: TThostFtdcRatioType,
    #[doc = "执行手续费"]
    pub StrikeRatioByVolume: TThostFtdcRatioType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "期权交易成本"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrTradeCostField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "期权合约保证金不变部分"]
    pub FixedMargin: TThostFtdcMoneyType,
    #[doc = "期权合约最小保证金"]
    pub MiniMargin: TThostFtdcMoneyType,
    #[doc = "期权合约权利金"]
    pub Royalty: TThostFtdcMoneyType,
    #[doc = "交易所期权合约保证金不变部分"]
    pub ExchFixedMargin: TThostFtdcMoneyType,
    #[doc = "交易所期权合约最小保证金"]
    pub ExchMiniMargin: TThostFtdcMoneyType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "期权交易成本查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOptionInstrTradeCostField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "期权合约报价"]
    pub InputPrice: TThostFtdcPriceType,
    #[doc = "标的价格,填0则用昨结算价"]
    pub UnderlyingPrice: TThostFtdcPriceType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "期权手续费率查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOptionInstrCommRateField {
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
#[doc = "股指现货指数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcIndexPriceField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "指数现货收盘价"]
    pub ClosePrice: TThostFtdcPriceType,
}
#[doc = "输入的执行宣告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputExecOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "保留头寸申请的持仓方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "期权行权后是否保留期货头寸的标记,该字段已废弃"]
    pub ReservePositionFlag: TThostFtdcExecOrderPositionFlagType,
    #[doc = "期权行权后生成的头寸是否自动平仓"]
    pub CloseFlag: TThostFtdcExecOrderCloseFlagType,
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
#[doc = "输入执行宣告操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputExecOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "执行宣告操作引用"]
    pub ExecOrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行宣告操作编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
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
#[doc = "执行宣告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcExecOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "保留头寸申请的持仓方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "期权行权后是否保留期货头寸的标记,该字段已废弃"]
    pub ReservePositionFlag: TThostFtdcExecOrderPositionFlagType,
    #[doc = "期权行权后生成的头寸是否自动平仓"]
    pub CloseFlag: TThostFtdcExecOrderCloseFlagType,
    #[doc = "本地执行宣告编号"]
    pub ExecOrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "执行宣告提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "执行宣告编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "执行结果"]
    pub ExecResult: TThostFtdcExecResultType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司报单编号"]
    pub BrokerExecOrderSeq: TThostFtdcSequenceNoType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcExecOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "执行宣告操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcExecOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "执行宣告操作引用"]
    pub ExecOrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行宣告操作编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地执行宣告编号"]
    pub ExecOrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcExecOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "执行宣告查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExecOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行宣告编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
}
#[doc = "交易所执行宣告信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeExecOrderField {
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "保留头寸申请的持仓方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "期权行权后是否保留期货头寸的标记,该字段已废弃"]
    pub ReservePositionFlag: TThostFtdcExecOrderPositionFlagType,
    #[doc = "期权行权后生成的头寸是否自动平仓"]
    pub CloseFlag: TThostFtdcExecOrderCloseFlagType,
    #[doc = "本地执行宣告编号"]
    pub ExecOrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "执行宣告提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "执行宣告编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "执行结果"]
    pub ExecResult: TThostFtdcExecResultType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所执行宣告查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeExecOrderField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "执行宣告操作查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExecOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "交易所执行宣告操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeExecOrderActionField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行宣告操作编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地执行宣告编号"]
    pub ExecOrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
}
#[doc = "交易所执行宣告操作查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeExecOrderActionField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "错误执行宣告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcErrExecOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "执行类型"]
    pub ActionType: TThostFtdcActionTypeType,
    #[doc = "保留头寸申请的持仓方向"]
    pub PosiDirection: TThostFtdcPosiDirectionType,
    #[doc = "期权行权后是否保留期货头寸的标记,该字段已废弃"]
    pub ReservePositionFlag: TThostFtdcExecOrderPositionFlagType,
    #[doc = "期权行权后生成的头寸是否自动平仓"]
    pub CloseFlag: TThostFtdcExecOrderCloseFlagType,
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
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcErrExecOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询错误执行宣告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryErrExecOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "错误执行宣告操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcErrExecOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "执行宣告操作引用"]
    pub ExecOrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "执行宣告引用"]
    pub ExecOrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "执行宣告操作编号"]
    pub ExecOrderSysID: TThostFtdcExecOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
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
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcErrExecOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询错误执行宣告操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryErrExecOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "投资者期权合约交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrTradingRightField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "交易权限"]
    pub TradingRight: TThostFtdcTradingRightType,
}
#[doc = "查询期权合约交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOptionInstrTradingRightField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
}
#[doc = "输入的询价"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputForQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "询价引用"]
    pub ForQuoteRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "询价"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcForQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "询价引用"]
    pub ForQuoteRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "本地询价编号"]
    pub ForQuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "询价状态"]
    pub ForQuoteStatus: TThostFtdcForQuoteStatusType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司询价编号"]
    pub BrokerForQutoSeq: TThostFtdcSequenceNoType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcForQuoteField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "询价查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryForQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "交易所询价信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeForQuoteField {
    #[doc = "本地询价编号"]
    pub ForQuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "询价状态"]
    pub ForQuoteStatus: TThostFtdcForQuoteStatusType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所询价查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeForQuoteField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "输入的报价"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报价引用"]
    pub QuoteRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "卖价格"]
    pub AskPrice: TThostFtdcPriceType,
    #[doc = "买价格"]
    pub BidPrice: TThostFtdcPriceType,
    #[doc = "卖数量"]
    pub AskVolume: TThostFtdcVolumeType,
    #[doc = "买数量"]
    pub BidVolume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "卖开平标志"]
    pub AskOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "买开平标志"]
    pub BidOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "卖投机套保标志"]
    pub AskHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "买投机套保标志"]
    pub BidHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "衍生卖报单引用"]
    pub AskOrderRef: TThostFtdcOrderRefType,
    #[doc = "衍生买报单引用"]
    pub BidOrderRef: TThostFtdcOrderRefType,
    #[doc = "应价编号"]
    pub ForQuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "交易编码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "输入报价操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputQuoteActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报价操作引用"]
    pub QuoteActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报价引用"]
    pub QuoteRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报价操作编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "交易编码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "报价"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报价引用"]
    pub QuoteRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "卖价格"]
    pub AskPrice: TThostFtdcPriceType,
    #[doc = "买价格"]
    pub BidPrice: TThostFtdcPriceType,
    #[doc = "卖数量"]
    pub AskVolume: TThostFtdcVolumeType,
    #[doc = "买数量"]
    pub BidVolume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "卖开平标志"]
    pub AskOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "买开平标志"]
    pub BidOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "卖投机套保标志"]
    pub AskHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "买投机套保标志"]
    pub BidHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "本地报价编号"]
    pub QuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报价提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "报价提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "报价编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "报价状态"]
    pub QuoteStatus: TThostFtdcOrderStatusType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "卖方报单编号"]
    pub AskOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "买方报单编号"]
    pub BidOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司报价编号"]
    pub BrokerQuoteSeq: TThostFtdcSequenceNoType,
    #[doc = "衍生卖报单引用"]
    pub AskOrderRef: TThostFtdcOrderRefType,
    #[doc = "衍生买报单引用"]
    pub BidOrderRef: TThostFtdcOrderRefType,
    #[doc = "应价编号"]
    pub ForQuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcQuoteField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "报价操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcQuoteActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报价操作引用"]
    pub QuoteActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报价引用"]
    pub QuoteRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报价操作编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报价编号"]
    pub QuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcQuoteActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "报价查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryQuoteField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报价编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "交易所报价信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeQuoteField {
    #[doc = "卖价格"]
    pub AskPrice: TThostFtdcPriceType,
    #[doc = "买价格"]
    pub BidPrice: TThostFtdcPriceType,
    #[doc = "卖数量"]
    pub AskVolume: TThostFtdcVolumeType,
    #[doc = "买数量"]
    pub BidVolume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "卖开平标志"]
    pub AskOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "买开平标志"]
    pub BidOffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "卖投机套保标志"]
    pub AskHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "买投机套保标志"]
    pub BidHedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "本地报价编号"]
    pub QuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报价提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "报价提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "报价编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "报价状态"]
    pub QuoteStatus: TThostFtdcOrderStatusType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "卖方报单编号"]
    pub AskOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "买方报单编号"]
    pub BidOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "应价编号"]
    pub ForQuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所报价查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeQuoteField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "报价操作查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryQuoteActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "交易所报价操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeQuoteActionField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报价操作编号"]
    pub QuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报价编号"]
    pub QuoteLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "交易所报价操作查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeQuoteActionField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "期权合约delta值"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcOptionInstrDeltaField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "Delta值"]
    pub Delta: TThostFtdcRatioType,
}
#[doc = "发给做市商的询价请求"]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcForQuoteRspField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "询价编号"]
    pub ForQuoteSysID: TThostFtdcOrderSysIDType,
    #[doc = "询价时间"]
    pub ForQuoteTime: TThostFtdcTimeType,
    #[doc = "业务日期"]
    pub ActionDay: TThostFtdcDateType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "当前期权合约执行偏移值的详细内容"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcStrikeOffsetField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "执行偏移值"]
    pub Offset: TThostFtdcMoneyType,
    #[doc = "执行偏移类型"]
    pub OffsetType: TThostFtdcStrikeOffsetTypeType,
}
#[doc = "期权执行偏移值查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryStrikeOffsetField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "输入批量报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputBatchOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "批量报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBatchOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcBatchOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所批量报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeBatchOrderActionField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "查询批量报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBatchOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "组合合约安全系数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCombInstrumentGuardField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = ""]
    pub GuarantRatio: TThostFtdcRatioType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "组合合约安全系数查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCombInstrumentGuardField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "输入的申请组合"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputCombActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "组合引用"]
    pub CombActionRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "组合指令方向"]
    pub CombDirection: TThostFtdcCombDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
}
#[doc = "申请组合"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcCombActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "组合引用"]
    pub CombActionRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "组合指令方向"]
    pub CombDirection: TThostFtdcCombDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "本地申请组合编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "组合状态"]
    pub ActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "组合编号"]
    pub ComTradeID: TThostFtdcTradeIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
impl Default for CThostFtdcCombActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "申请组合查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCombActionField {
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
#[doc = "交易所申请组合信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeCombActionField {
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "组合指令方向"]
    pub CombDirection: TThostFtdcCombDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "本地申请组合编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "组合状态"]
    pub ActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "组合编号"]
    pub ComTradeID: TThostFtdcTradeIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
}
#[doc = "交易所申请组合查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeCombActionField {
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "产品报价汇率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcProductExchRateField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "报价币种类型"]
    pub QuoteCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇率"]
    pub ExchangeRate: TThostFtdcExchangeRateType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "产品报价汇率查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryProductExchRateField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "查询询价价差参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryForQuoteParamField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "询价价差参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcForQuoteParamField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "最新价"]
    pub LastPrice: TThostFtdcPriceType,
    #[doc = "价差"]
    pub PriceInterval: TThostFtdcPriceType,
}
#[doc = "当前做市商期权合约手续费的详细内容"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMMOptionInstrCommRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "开仓手续费率"]
    pub OpenRatioByMoney: TThostFtdcRatioType,
    #[doc = "开仓手续费"]
    pub OpenRatioByVolume: TThostFtdcRatioType,
    #[doc = "平仓手续费率"]
    pub CloseRatioByMoney: TThostFtdcRatioType,
    #[doc = "平仓手续费"]
    pub CloseRatioByVolume: TThostFtdcRatioType,
    #[doc = "平今手续费率"]
    pub CloseTodayRatioByMoney: TThostFtdcRatioType,
    #[doc = "平今手续费"]
    pub CloseTodayRatioByVolume: TThostFtdcRatioType,
    #[doc = "执行手续费率"]
    pub StrikeRatioByMoney: TThostFtdcRatioType,
    #[doc = "执行手续费"]
    pub StrikeRatioByVolume: TThostFtdcRatioType,
}
#[doc = "做市商期权手续费率查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryMMOptionInstrCommRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "做市商合约手续费率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMMInstrumentCommissionRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "开仓手续费率"]
    pub OpenRatioByMoney: TThostFtdcRatioType,
    #[doc = "开仓手续费"]
    pub OpenRatioByVolume: TThostFtdcRatioType,
    #[doc = "平仓手续费率"]
    pub CloseRatioByMoney: TThostFtdcRatioType,
    #[doc = "平仓手续费"]
    pub CloseRatioByVolume: TThostFtdcRatioType,
    #[doc = "平今手续费率"]
    pub CloseTodayRatioByMoney: TThostFtdcRatioType,
    #[doc = "平今手续费"]
    pub CloseTodayRatioByVolume: TThostFtdcRatioType,
}
#[doc = "查询做市商合约手续费率"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryMMInstrumentCommissionRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "当前报单手续费的详细内容"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentOrderCommRateField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "报单手续费"]
    pub OrderCommByVolume: TThostFtdcRatioType,
    #[doc = "撤单手续费"]
    pub OrderActionCommByVolume: TThostFtdcRatioType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "报单手续费率查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInstrumentOrderCommRateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "交易参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradeParamField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "参数代码"]
    pub TradeParamID: TThostFtdcTradeParamIDType,
    #[doc = "参数代码值"]
    #[serde(with = "BigArray")]
    pub TradeParamValue: TThostFtdcSettlementParamValueType,
    #[doc = "备注"]
    #[serde(with = "BigArray")]
    pub Memo: TThostFtdcMemoType,
}
impl Default for CThostFtdcTradeParamField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "合约保证金率调整"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentMarginRateULField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "多头保证金率"]
    pub LongMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "多头保证金费"]
    pub LongMarginRatioByVolume: TThostFtdcMoneyType,
    #[doc = "空头保证金率"]
    pub ShortMarginRatioByMoney: TThostFtdcRatioType,
    #[doc = "空头保证金费"]
    pub ShortMarginRatioByVolume: TThostFtdcMoneyType,
}
#[doc = "期货持仓限制参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcFutureLimitPosiParamField {
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "当日投机开仓数量限制"]
    pub SpecOpenVolume: TThostFtdcVolumeType,
    #[doc = "当日套利开仓数量限制"]
    pub ArbiOpenVolume: TThostFtdcVolumeType,
    #[doc = "当日投机+套利开仓数量限制"]
    pub OpenVolume: TThostFtdcVolumeType,
}
#[doc = "禁止登录IP"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcLoginForbiddenIPField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "IP列表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcIPListField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "是否白名单"]
    pub IsWhite: TThostFtdcBoolType,
}
#[doc = "输入的期权自对冲"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputOptionSelfCloseField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "期权自对冲引用"]
    pub OptionSelfCloseRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "期权行权的头寸是否自对冲"]
    pub OptSelfCloseFlag: TThostFtdcOptSelfCloseFlagType,
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
#[doc = "输入期权自对冲操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInputOptionSelfCloseActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "期权自对冲操作引用"]
    pub OptionSelfCloseActionRef: TThostFtdcOrderActionRefType,
    #[doc = "期权自对冲引用"]
    pub OptionSelfCloseRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "期权自对冲操作编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
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
#[doc = "期权自对冲"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcOptionSelfCloseField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "期权自对冲引用"]
    pub OptionSelfCloseRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "期权行权的头寸是否自对冲"]
    pub OptSelfCloseFlag: TThostFtdcOptSelfCloseFlagType,
    #[doc = "本地期权自对冲编号"]
    pub OptionSelfCloseLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期权自对冲提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "期权自对冲编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "自对冲结果"]
    pub ExecResult: TThostFtdcExecResultType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司报单编号"]
    pub BrokerOptionSelfCloseSeq: TThostFtdcSequenceNoType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcOptionSelfCloseField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期权自对冲操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcOptionSelfCloseActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "期权自对冲操作引用"]
    pub OptionSelfCloseActionRef: TThostFtdcOrderActionRefType,
    #[doc = "期权自对冲引用"]
    pub OptionSelfCloseRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "期权自对冲操作编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地期权自对冲编号"]
    pub OptionSelfCloseLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcOptionSelfCloseActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期权自对冲查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOptionSelfCloseField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "期权自对冲编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "开始时间"]
    pub InsertTimeStart: TThostFtdcTimeType,
    #[doc = "结束时间"]
    pub InsertTimeEnd: TThostFtdcTimeType,
}
#[doc = "交易所期权自对冲信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOptionSelfCloseField {
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "期权行权的头寸是否自对冲"]
    pub OptSelfCloseFlag: TThostFtdcOptSelfCloseFlagType,
    #[doc = "本地期权自对冲编号"]
    pub OptionSelfCloseLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期权自对冲提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "期权自对冲编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "插入时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "自对冲结果"]
    pub ExecResult: TThostFtdcExecResultType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "期权自对冲操作查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryOptionSelfCloseActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "交易所期权自对冲操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeOptionSelfCloseActionField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "期权自对冲操作编号"]
    pub OptionSelfCloseSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地期权自对冲编号"]
    pub OptionSelfCloseLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "期权行权的头寸是否自对冲"]
    pub OptSelfCloseFlag: TThostFtdcOptSelfCloseFlagType,
}
#[doc = "延时换汇同步"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncDelaySwapField {
    #[doc = "换汇流水号"]
    pub DelaySwapSeqNo: TThostFtdcDepositSeqNoType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "源币种"]
    pub FromCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "源金额"]
    pub FromAmount: TThostFtdcMoneyType,
    #[doc = "源换汇冻结金额(可用冻结)"]
    pub FromFrozenSwap: TThostFtdcMoneyType,
    #[doc = "源剩余换汇额度(可提冻结)"]
    pub FromRemainSwap: TThostFtdcMoneyType,
    #[doc = "目标币种"]
    pub ToCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "目标金额"]
    pub ToAmount: TThostFtdcMoneyType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "是否手工换汇"]
    pub IsManualSwap: TThostFtdcBoolType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "是否将所有外币的剩余换汇额度设置为0"]
    pub IsAllRemainSetZero: TThostFtdcBoolType,
}
#[doc = "查询延时换汇同步"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySyncDelaySwapField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "延时换汇流水号"]
    pub DelaySwapSeqNo: TThostFtdcDepositSeqNoType,
}
#[doc = "投资单元"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInvestUnitField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "投资者单元名称"]
    #[serde(with = "BigArray")]
    pub InvestorUnitName: TThostFtdcPartyNameType,
    #[doc = "投资者分组代码"]
    pub InvestorGroupID: TThostFtdcInvestorIDType,
    #[doc = "手续费率模板代码"]
    pub CommModelID: TThostFtdcInvestorIDType,
    #[doc = "保证金率模板代码"]
    pub MarginModelID: TThostFtdcInvestorIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcInvestUnitField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询投资单元"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInvestUnitField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "二级代理商资金校验模式"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSecAgentCheckModeField {
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "币种"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "境外中介机构资金帐号"]
    pub BrokerSecAgentID: TThostFtdcAccountIDType,
    #[doc = "是否需要校验自己的资金账户"]
    pub CheckSelfAccount: TThostFtdcBoolType,
}
#[doc = "二级代理商信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcSecAgentTradeInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "境外中介机构资金帐号"]
    pub BrokerSecAgentID: TThostFtdcAccountIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "二级代理商姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcSecAgentTradeInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "市场行情"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "最新价"]
    pub LastPrice: TThostFtdcPriceType,
    #[doc = "上次结算价"]
    pub PreSettlementPrice: TThostFtdcPriceType,
    #[doc = "昨收盘"]
    pub PreClosePrice: TThostFtdcPriceType,
    #[doc = "昨持仓量"]
    pub PreOpenInterest: TThostFtdcLargeVolumeType,
    #[doc = "今开盘"]
    pub OpenPrice: TThostFtdcPriceType,
    #[doc = "最高价"]
    pub HighestPrice: TThostFtdcPriceType,
    #[doc = "最低价"]
    pub LowestPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "成交金额"]
    pub Turnover: TThostFtdcMoneyType,
    #[doc = "持仓量"]
    pub OpenInterest: TThostFtdcLargeVolumeType,
    #[doc = "今收盘"]
    pub ClosePrice: TThostFtdcPriceType,
    #[doc = "本次结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "涨停板价"]
    pub UpperLimitPrice: TThostFtdcPriceType,
    #[doc = "跌停板价"]
    pub LowerLimitPrice: TThostFtdcPriceType,
    #[doc = "昨虚实度"]
    pub PreDelta: TThostFtdcRatioType,
    #[doc = "今虚实度"]
    pub CurrDelta: TThostFtdcRatioType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "最后修改毫秒"]
    pub UpdateMillisec: TThostFtdcMillisecType,
    #[doc = "业务日期"]
    pub ActionDay: TThostFtdcDateType,
}
#[doc = "行情基础属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataBaseField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "上次结算价"]
    pub PreSettlementPrice: TThostFtdcPriceType,
    #[doc = "昨收盘"]
    pub PreClosePrice: TThostFtdcPriceType,
    #[doc = "昨持仓量"]
    pub PreOpenInterest: TThostFtdcLargeVolumeType,
    #[doc = "昨虚实度"]
    pub PreDelta: TThostFtdcRatioType,
}
#[doc = "行情静态属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataStaticField {
    #[doc = "今开盘"]
    pub OpenPrice: TThostFtdcPriceType,
    #[doc = "最高价"]
    pub HighestPrice: TThostFtdcPriceType,
    #[doc = "最低价"]
    pub LowestPrice: TThostFtdcPriceType,
    #[doc = "今收盘"]
    pub ClosePrice: TThostFtdcPriceType,
    #[doc = "涨停板价"]
    pub UpperLimitPrice: TThostFtdcPriceType,
    #[doc = "跌停板价"]
    pub LowerLimitPrice: TThostFtdcPriceType,
    #[doc = "本次结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "今虚实度"]
    pub CurrDelta: TThostFtdcRatioType,
}
#[doc = "行情最新成交属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataLastMatchField {
    #[doc = "最新价"]
    pub LastPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "成交金额"]
    pub Turnover: TThostFtdcMoneyType,
    #[doc = "持仓量"]
    pub OpenInterest: TThostFtdcLargeVolumeType,
}
#[doc = "行情最优价属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataBestPriceField {
    #[doc = "申买价一"]
    pub BidPrice1: TThostFtdcPriceType,
    #[doc = "申买量一"]
    pub BidVolume1: TThostFtdcVolumeType,
    #[doc = "申卖价一"]
    pub AskPrice1: TThostFtdcPriceType,
    #[doc = "申卖量一"]
    pub AskVolume1: TThostFtdcVolumeType,
}
#[doc = "行情申买二、三属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataBid23Field {
    #[doc = "申买价二"]
    pub BidPrice2: TThostFtdcPriceType,
    #[doc = "申买量二"]
    pub BidVolume2: TThostFtdcVolumeType,
    #[doc = "申买价三"]
    pub BidPrice3: TThostFtdcPriceType,
    #[doc = "申买量三"]
    pub BidVolume3: TThostFtdcVolumeType,
}
#[doc = "行情申卖二、三属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataAsk23Field {
    #[doc = "申卖价二"]
    pub AskPrice2: TThostFtdcPriceType,
    #[doc = "申卖量二"]
    pub AskVolume2: TThostFtdcVolumeType,
    #[doc = "申卖价三"]
    pub AskPrice3: TThostFtdcPriceType,
    #[doc = "申卖量三"]
    pub AskVolume3: TThostFtdcVolumeType,
}
#[doc = "行情申买四、五属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataBid45Field {
    #[doc = "申买价四"]
    pub BidPrice4: TThostFtdcPriceType,
    #[doc = "申买量四"]
    pub BidVolume4: TThostFtdcVolumeType,
    #[doc = "申买价五"]
    pub BidPrice5: TThostFtdcPriceType,
    #[doc = "申买量五"]
    pub BidVolume5: TThostFtdcVolumeType,
}
#[doc = "行情申卖四、五属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataAsk45Field {
    #[doc = "申卖价四"]
    pub AskPrice4: TThostFtdcPriceType,
    #[doc = "申卖量四"]
    pub AskVolume4: TThostFtdcVolumeType,
    #[doc = "申卖价五"]
    pub AskPrice5: TThostFtdcPriceType,
    #[doc = "申卖量五"]
    pub AskVolume5: TThostFtdcVolumeType,
}
#[doc = "行情更新时间属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataUpdateTimeField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "最后修改毫秒"]
    pub UpdateMillisec: TThostFtdcMillisecType,
    #[doc = "业务日期"]
    pub ActionDay: TThostFtdcDateType,
}
#[doc = "行情交易所代码属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataExchangeField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "指定的合约"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSpecificInstrumentField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "合约状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcInstrumentStatusField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "结算组代码"]
    pub SettlementGroupID: TThostFtdcSettlementGroupIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "合约交易状态"]
    pub InstrumentStatus: TThostFtdcInstrumentStatusType,
    #[doc = "交易阶段编号"]
    pub TradingSegmentSN: TThostFtdcTradingSegmentSNType,
    #[doc = "进入本状态时间"]
    pub EnterTime: TThostFtdcTimeType,
    #[doc = "进入本状态原因"]
    pub EnterReason: TThostFtdcInstStatusEnterReasonType,
}
#[doc = "查询合约状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInstrumentStatusField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
}
#[doc = "投资者账户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorAccountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "浮动盈亏算法"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcPositionProfitAlgorithmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "盈亏算法"]
    pub Algorithm: TThostFtdcAlgorithmType,
    #[doc = "备注"]
    #[serde(with = "BigArray")]
    pub Memo: TThostFtdcMemoType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcPositionProfitAlgorithmField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "会员资金折扣"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcDiscountField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "资金折扣比例"]
    pub Discount: TThostFtdcRatioType,
}
#[doc = "查询转帐银行"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTransferBankField {
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
}
#[doc = "转帐银行"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferBankField {
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
    #[doc = "银行名称"]
    #[serde(with = "BigArray")]
    pub BankName: TThostFtdcBankNameType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
}
impl Default for CThostFtdcTransferBankField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询投资者持仓明细"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "投资者持仓明细"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorPositionDetailField {
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "买卖"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "开仓日期"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "成交编号"]
    pub TradeID: TThostFtdcTradeIDType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "开仓价"]
    pub OpenPrice: TThostFtdcPriceType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "成交类型"]
    pub TradeType: TThostFtdcTradeTypeType,
    #[doc = "组合合约代码"]
    pub CombInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "逐日盯市平仓盈亏"]
    pub CloseProfitByDate: TThostFtdcMoneyType,
    #[doc = "逐笔对冲平仓盈亏"]
    pub CloseProfitByTrade: TThostFtdcMoneyType,
    #[doc = "逐日盯市持仓盈亏"]
    pub PositionProfitByDate: TThostFtdcMoneyType,
    #[doc = "逐笔对冲持仓盈亏"]
    pub PositionProfitByTrade: TThostFtdcMoneyType,
    #[doc = "投资者保证金"]
    pub Margin: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchMargin: TThostFtdcMoneyType,
    #[doc = "保证金率"]
    pub MarginRateByMoney: TThostFtdcRatioType,
    #[doc = "保证金率(按手数)"]
    pub MarginRateByVolume: TThostFtdcRatioType,
    #[doc = "昨结算价"]
    pub LastSettlementPrice: TThostFtdcPriceType,
    #[doc = "结算价"]
    pub SettlementPrice: TThostFtdcPriceType,
    #[doc = "平仓量"]
    pub CloseVolume: TThostFtdcVolumeType,
    #[doc = "平仓金额"]
    pub CloseAmount: TThostFtdcMoneyType,
    #[cfg(not(feature = "v6_3_13"))]
    #[doc = "先开先平剩余数量（DCE）"]
    pub TimeFirstVolume: TThostFtdcVolumeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))]
    #[doc = "特殊持仓标志"]
    pub SpecPosiType: TThostFtdcSpecPosiTypeType,
}
#[doc = "资金账户口令域"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingAccountPasswordField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcTradingAccountPasswordField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易所行情报盘机"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcMDTraderOfferField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所交易员连接状态"]
    pub TraderConnectStatus: TThostFtdcTraderConnectStatusType,
    #[doc = "发出连接请求的日期"]
    pub ConnectRequestDate: TThostFtdcDateType,
    #[doc = "发出连接请求的时间"]
    pub ConnectRequestTime: TThostFtdcTimeType,
    #[doc = "上次报告日期"]
    pub LastReportDate: TThostFtdcDateType,
    #[doc = "上次报告时间"]
    pub LastReportTime: TThostFtdcTimeType,
    #[doc = "完成连接日期"]
    pub ConnectDate: TThostFtdcDateType,
    #[doc = "完成连接时间"]
    pub ConnectTime: TThostFtdcTimeType,
    #[doc = "启动日期"]
    pub StartDate: TThostFtdcDateType,
    #[doc = "启动时间"]
    pub StartTime: TThostFtdcTimeType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "本席位最大成交编号"]
    pub MaxTradeID: TThostFtdcTradeIDType,
    #[doc = "本席位最大报单备拷"]
    pub MaxOrderMessageReference: TThostFtdcReturnCodeType,
}
impl Default for CThostFtdcMDTraderOfferField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询行情报盘机"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryMDTraderOfferField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
}
#[doc = "查询客户通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryNoticeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "客户通知"]
#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CThostFtdcNoticeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "消息正文"]
    #[serde(with = "BigArray")]
    pub Content: TThostFtdcContentType,
    #[doc = "经纪公司通知内容序列号"]
    pub SequenceLabel: TThostFtdcSequenceLabelType,
}
impl Default for CThostFtdcNoticeField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcUserRightField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "客户权限类型"]
    pub UserRightType: TThostFtdcUserRightTypeType,
    #[doc = "是否禁止"]
    pub IsForbidden: TThostFtdcBoolType,
}
#[doc = "查询结算信息确认域"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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
#[doc = "装载结算信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcLoadSettlementInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "经纪公司可提资金算法表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerWithdrawAlgorithmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "可提资金算法"]
    pub WithdrawAlgorithm: TThostFtdcAlgorithmType,
    #[doc = "资金使用率"]
    pub UsingRatio: TThostFtdcRatioType,
    #[doc = "可提是否包含平仓盈利"]
    pub IncludeCloseProfit: TThostFtdcIncludeCloseProfitType,
    #[doc = "本日无仓且无成交客户是否受可提比例限制"]
    pub AllWithoutTrade: TThostFtdcAllWithoutTradeType,
    #[doc = "可用是否包含平仓盈利"]
    pub AvailIncludeCloseProfit: TThostFtdcIncludeCloseProfitType,
    #[doc = "是否启用用户事件"]
    pub IsBrokerUserEvent: TThostFtdcBoolType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "货币质押比率"]
    pub FundMortgageRatio: TThostFtdcRatioType,
    #[doc = "权益算法"]
    pub BalanceAlgorithm: TThostFtdcBalanceAlgorithmType,
}
#[doc = "资金账户口令变更域"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingAccountPasswordUpdateV1Field {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "原来的口令"]
    #[serde(with = "BigArray")]
    pub OldPassword: TThostFtdcPasswordType,
    #[doc = "新的口令"]
    #[serde(with = "BigArray")]
    pub NewPassword: TThostFtdcPasswordType,
}
impl Default for CThostFtdcTradingAccountPasswordUpdateV1Field {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "资金账户口令变更域"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingAccountPasswordUpdateField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "原来的口令"]
    #[serde(with = "BigArray")]
    pub OldPassword: TThostFtdcPasswordType,
    #[doc = "新的口令"]
    #[serde(with = "BigArray")]
    pub NewPassword: TThostFtdcPasswordType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcTradingAccountPasswordUpdateField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询组合合约分腿"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCombinationLegField {
    #[doc = "组合合约代码"]
    pub CombInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "单腿编号"]
    pub LegID: TThostFtdcLegIDType,
    #[doc = "单腿合约代码"]
    pub LegInstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "查询组合合约分腿"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySyncStatusField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
}
#[doc = "组合交易合约的单腿"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCombinationLegField {
    #[doc = "组合合约代码"]
    pub CombInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "单腿编号"]
    pub LegID: TThostFtdcLegIDType,
    #[doc = "单腿合约代码"]
    pub LegInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "单腿乘数"]
    pub LegMultiple: TThostFtdcLegMultipleType,
    #[doc = "派生层数"]
    pub ImplyLevel: TThostFtdcImplyLevelType,
}
#[doc = "数据同步状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncStatusField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "数据同步状态"]
    pub DataSyncStatus: TThostFtdcDataSyncStatusType,
}
#[doc = "查询联系人"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryLinkManField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "联系人"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcLinkManField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "联系人类型"]
    pub PersonType: TThostFtdcPersonTypeType,
    #[doc = "证件类型"]
    pub IdentifiedCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "名称"]
    #[serde(with = "BigArray")]
    pub PersonName: TThostFtdcPartyNameType,
    #[doc = "联系电话"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "通讯地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮政编码"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "优先级"]
    pub Priority: TThostFtdcPriorityType,
    #[doc = "开户邮政编码"]
    pub UOAZipCode: TThostFtdcUOAZipCodeType,
    #[doc = "全称"]
    #[serde(with = "BigArray")]
    pub PersonFullName: TThostFtdcInvestorFullNameType,
}
impl Default for CThostFtdcLinkManField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询经纪公司用户事件"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerUserEventField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户事件类型"]
    pub UserEventType: TThostFtdcUserEventTypeType,
}
#[doc = "查询经纪公司用户事件"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserEventField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户事件类型"]
    pub UserEventType: TThostFtdcUserEventTypeType,
    #[doc = "用户事件序号"]
    pub EventSequenceNo: TThostFtdcSequenceNoType,
    #[doc = "事件发生日期"]
    pub EventDate: TThostFtdcDateType,
    #[doc = "事件发生时间"]
    pub EventTime: TThostFtdcTimeType,
    #[doc = "用户事件信息"]
    #[serde(with = "BigArray")]
    pub UserEventInfo: TThostFtdcUserEventInfoType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
impl Default for CThostFtdcBrokerUserEventField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询签约银行请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryContractBankField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
}
#[doc = "查询签约银行响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcContractBankField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分中心代码"]
    pub BankBrchID: TThostFtdcBankBrchIDType,
    #[doc = "银行名称"]
    #[serde(with = "BigArray")]
    pub BankName: TThostFtdcBankNameType,
}
impl Default for CThostFtdcContractBankField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "投资者组合持仓明细"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorPositionCombineDetailField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "开仓日期"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "组合编号"]
    pub ComTradeID: TThostFtdcTradeIDType,
    #[doc = "撮合编号"]
    pub TradeID: TThostFtdcTradeIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "买卖"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "持仓量"]
    pub TotalAmt: TThostFtdcVolumeType,
    #[doc = "投资者保证金"]
    pub Margin: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchMargin: TThostFtdcMoneyType,
    #[doc = "保证金率"]
    pub MarginRateByMoney: TThostFtdcRatioType,
    #[doc = "保证金率(按手数)"]
    pub MarginRateByVolume: TThostFtdcRatioType,
    #[doc = "单腿编号"]
    pub LegID: TThostFtdcLegIDType,
    #[doc = "单腿乘数"]
    pub LegMultiple: TThostFtdcLegMultipleType,
    #[doc = "组合持仓合约编码"]
    pub CombInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "成交组号"]
    pub TradeGroupID: TThostFtdcTradeGroupIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "预埋单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcParkedOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "用户强评标志"]
    pub UserForceClose: TThostFtdcBoolType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "预埋报单编号"]
    pub ParkedOrderID: TThostFtdcParkedOrderIDType,
    #[doc = "用户类型"]
    pub UserType: TThostFtdcUserTypeType,
    #[doc = "预埋单状态"]
    pub Status: TThostFtdcParkedOrderStatusType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "互换单标志"]
    pub IsSwapOrder: TThostFtdcBoolType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "交易编码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcParkedOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "输入预埋单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcParkedOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量变化"]
    pub VolumeChange: TThostFtdcVolumeType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "预埋撤单单编号"]
    pub ParkedOrderActionID: TThostFtdcParkedOrderActionIDType,
    #[doc = "用户类型"]
    pub UserType: TThostFtdcUserTypeType,
    #[doc = "预埋撤单状态"]
    pub Status: TThostFtdcParkedOrderStatusType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcParkedOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询预埋单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryParkedOrderField {
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
#[doc = "查询预埋撤单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryParkedOrderActionField {
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
#[doc = "删除预埋单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcRemoveParkedOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "预埋报单编号"]
    pub ParkedOrderID: TThostFtdcParkedOrderIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "删除预埋撤单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcRemoveParkedOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "预埋撤单编号"]
    pub ParkedOrderActionID: TThostFtdcParkedOrderActionIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "经纪公司可提资金算法表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorWithdrawAlgorithmField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "可提资金比例"]
    pub UsingRatio: TThostFtdcRatioType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "货币质押比率"]
    pub FundMortgageRatio: TThostFtdcRatioType,
}
#[doc = "查询组合持仓明细"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInvestorPositionCombineDetailField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "组合持仓合约编码"]
    pub CombInstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "成交均价"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMarketDataAveragePriceField {
    #[doc = "当日均价"]
    pub AveragePrice: TThostFtdcPriceType,
}
#[doc = "校验投资者密码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcVerifyInvestorPasswordField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
}
impl Default for CThostFtdcVerifyInvestorPasswordField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户IP"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcUserIPField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "IP地址掩码"]
    pub IPMask: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
#[doc = "用户事件通知信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingNoticeInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "发送时间"]
    pub SendTime: TThostFtdcTimeType,
    #[doc = "消息正文"]
    #[serde(with = "BigArray")]
    pub FieldContent: TThostFtdcContentType,
    #[doc = "序列系列号"]
    pub SequenceSeries: TThostFtdcSequenceSeriesType,
    #[doc = "序列号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
impl Default for CThostFtdcTradingNoticeInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户事件通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTradingNoticeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcInvestorRangeType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "序列系列号"]
    pub SequenceSeries: TThostFtdcSequenceSeriesType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "发送时间"]
    pub SendTime: TThostFtdcTimeType,
    #[doc = "序列号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "消息正文"]
    #[serde(with = "BigArray")]
    pub FieldContent: TThostFtdcContentType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
impl Default for CThostFtdcTradingNoticeField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询交易事件通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTradingNoticeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询错误报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryErrOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "错误报单"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcErrOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "用户强评标志"]
    pub UserForceClose: TThostFtdcBoolType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "互换单标志"]
    pub IsSwapOrder: TThostFtdcBoolType,
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
impl Default for CThostFtdcErrOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询错误报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcErrorConditionalOrderField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "报单价格条件"]
    pub OrderPriceType: TThostFtdcOrderPriceTypeType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "组合开平标志"]
    pub CombOffsetFlag: TThostFtdcCombOffsetFlagType,
    #[doc = "组合投机套保标志"]
    pub CombHedgeFlag: TThostFtdcCombHedgeFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量"]
    pub VolumeTotalOriginal: TThostFtdcVolumeType,
    #[doc = "有效期类型"]
    pub TimeCondition: TThostFtdcTimeConditionType,
    #[doc = "GTD日期"]
    pub GTDDate: TThostFtdcDateType,
    #[doc = "成交量类型"]
    pub VolumeCondition: TThostFtdcVolumeConditionType,
    #[doc = "最小成交量"]
    pub MinVolume: TThostFtdcVolumeType,
    #[doc = "触发条件"]
    pub ContingentCondition: TThostFtdcContingentConditionType,
    #[doc = "止损价"]
    pub StopPrice: TThostFtdcPriceType,
    #[doc = "强平原因"]
    pub ForceCloseReason: TThostFtdcForceCloseReasonType,
    #[doc = "自动挂起标志"]
    pub IsAutoSuspend: TThostFtdcBoolType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "合约在交易所的代码"]
    pub ExchangeInstID: TThostFtdcExchangeInstIDType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "报单提交状态"]
    pub OrderSubmitStatus: TThostFtdcOrderSubmitStatusType,
    #[doc = "报单提示序号"]
    pub NotifySequence: TThostFtdcSequenceNoType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "报单来源"]
    pub OrderSource: TThostFtdcOrderSourceType,
    #[doc = "报单状态"]
    pub OrderStatus: TThostFtdcOrderStatusType,
    #[doc = "报单类型"]
    pub OrderType: TThostFtdcOrderTypeType,
    #[doc = "今成交数量"]
    pub VolumeTraded: TThostFtdcVolumeType,
    #[doc = "剩余数量"]
    pub VolumeTotal: TThostFtdcVolumeType,
    #[doc = "报单日期"]
    pub InsertDate: TThostFtdcDateType,
    #[doc = "委托时间"]
    pub InsertTime: TThostFtdcTimeType,
    #[doc = "激活时间"]
    pub ActiveTime: TThostFtdcTimeType,
    #[doc = "挂起时间"]
    pub SuspendTime: TThostFtdcTimeType,
    #[doc = "最后修改时间"]
    pub UpdateTime: TThostFtdcTimeType,
    #[doc = "撤销时间"]
    pub CancelTime: TThostFtdcTimeType,
    #[doc = "最后修改交易所交易员代码"]
    pub ActiveTraderID: TThostFtdcTraderIDType,
    #[doc = "结算会员编号"]
    pub ClearingPartID: TThostFtdcParticipantIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "用户强评标志"]
    pub UserForceClose: TThostFtdcBoolType,
    #[doc = "操作用户代码"]
    pub ActiveUserID: TThostFtdcUserIDType,
    #[doc = "经纪公司报单编号"]
    pub BrokerOrderSeq: TThostFtdcSequenceNoType,
    #[doc = "相关报单"]
    pub RelativeOrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "郑商所成交数量"]
    pub ZCETotalTradedVolume: TThostFtdcVolumeType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "互换单标志"]
    pub IsSwapOrder: TThostFtdcBoolType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "资金账号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
}
impl Default for CThostFtdcErrorConditionalOrderField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询错误报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryErrOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "错误报单操作"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcErrOrderActionField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "报单操作引用"]
    pub OrderActionRef: TThostFtdcOrderActionRefType,
    #[doc = "报单引用"]
    pub OrderRef: TThostFtdcOrderRefType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "前置编号"]
    pub FrontID: TThostFtdcFrontIDType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "报单编号"]
    pub OrderSysID: TThostFtdcOrderSysIDType,
    #[doc = "操作标志"]
    pub ActionFlag: TThostFtdcActionFlagType,
    #[doc = "价格"]
    pub LimitPrice: TThostFtdcPriceType,
    #[doc = "数量变化"]
    pub VolumeChange: TThostFtdcVolumeType,
    #[doc = "操作日期"]
    pub ActionDate: TThostFtdcDateType,
    #[doc = "操作时间"]
    pub ActionTime: TThostFtdcTimeType,
    #[doc = "交易所交易员代码"]
    pub TraderID: TThostFtdcTraderIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "本地报单编号"]
    pub OrderLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "操作本地编号"]
    pub ActionLocalID: TThostFtdcOrderLocalIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "客户代码"]
    pub ClientID: TThostFtdcClientIDType,
    #[doc = "业务单元"]
    pub BusinessUnit: TThostFtdcBusinessUnitType,
    #[doc = "报单操作状态"]
    pub OrderActionStatus: TThostFtdcOrderActionStatusType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "状态信息"]
    #[serde(with = "BigArray")]
    pub StatusMsg: TThostFtdcErrorMsgType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "营业部编号"]
    pub BranchID: TThostFtdcBranchIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcErrOrderActionField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询交易所状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryExchangeSequenceField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "交易所状态"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcExchangeSequenceField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "序号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "合约交易状态"]
    pub MarketStatus: TThostFtdcInstrumentStatusType,
}
#[doc = "根据价格查询最大报单数量"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQueryMaxOrderVolumeWithPriceField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "开平标志"]
    pub OffsetFlag: TThostFtdcOffsetFlagType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "最大允许报单数量"]
    pub MaxVolume: TThostFtdcVolumeType,
    #[doc = "报单价格"]
    pub Price: TThostFtdcPriceType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询经纪公司交易参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerTradingParamsField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
}
#[doc = "经纪公司交易参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerTradingParamsField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "保证金价格类型"]
    pub MarginPriceType: TThostFtdcMarginPriceTypeType,
    #[doc = "盈亏算法"]
    pub Algorithm: TThostFtdcAlgorithmType,
    #[doc = "可用是否包含平仓盈利"]
    pub AvailIncludeCloseProfit: TThostFtdcIncludeCloseProfitType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "期权权利金价格类型"]
    pub OptionRoyaltyPriceType: TThostFtdcOptionRoyaltyPriceTypeType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
}
#[doc = "查询经纪公司交易算法"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBrokerTradingAlgosField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "经纪公司交易算法"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerTradingAlgosField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "持仓处理算法编号"]
    pub HandlePositionAlgoID: TThostFtdcHandlePositionAlgoIDType,
    #[doc = "寻找保证金率算法编号"]
    pub FindMarginRateAlgoID: TThostFtdcFindMarginRateAlgoIDType,
    #[doc = "资金处理算法编号"]
    pub HandleTradingAccountAlgoID: TThostFtdcHandleTradingAccountAlgoIDType,
}
#[doc = "查询经纪公司资金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQueryBrokerDepositField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "经纪公司资金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerDepositField {
    #[doc = "交易日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "会员代码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "上次结算准备金"]
    pub PreBalance: TThostFtdcMoneyType,
    #[doc = "当前保证金总额"]
    pub CurrMargin: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "期货结算准备金"]
    pub Balance: TThostFtdcMoneyType,
    #[doc = "入金金额"]
    pub Deposit: TThostFtdcMoneyType,
    #[doc = "出金金额"]
    pub Withdraw: TThostFtdcMoneyType,
    #[doc = "可提资金"]
    pub Available: TThostFtdcMoneyType,
    #[doc = "基本准备金"]
    pub Reserve: TThostFtdcMoneyType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
}
#[doc = "查询保证金监管系统经纪公司密钥"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCFMMCBrokerKeyField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
}
#[doc = "保证金监管系统经纪公司密钥"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCFMMCBrokerKeyField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "经纪公司统一编码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "密钥生成日期"]
    pub CreateDate: TThostFtdcDateType,
    #[doc = "密钥生成时间"]
    pub CreateTime: TThostFtdcTimeType,
    #[doc = "密钥编号"]
    pub KeyID: TThostFtdcSequenceNoType,
    #[doc = "动态密钥"]
    pub CurrentKey: TThostFtdcCFMMCKeyType,
    #[doc = "动态密钥类型"]
    pub KeyKind: TThostFtdcCFMMCKeyKindType,
}
#[doc = "保证金监管系统经纪公司资金账户密钥"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCFMMCTradingAccountKeyField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "经纪公司统一编码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "密钥编号"]
    pub KeyID: TThostFtdcSequenceNoType,
    #[doc = "动态密钥"]
    pub CurrentKey: TThostFtdcCFMMCKeyType,
}
#[doc = "请求查询保证金监管系统经纪公司资金账户密钥"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCFMMCTradingAccountKeyField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "用户动态令牌参数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserOTPParamField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "动态令牌提供商"]
    pub OTPVendorsID: TThostFtdcOTPVendorsIDType,
    #[doc = "动态令牌序列号"]
    pub SerialNumber: TThostFtdcSerialNumberType,
    #[doc = "令牌密钥"]
    #[serde(with = "BigArray")]
    pub AuthKey: TThostFtdcAuthKeyType,
    #[doc = "漂移值"]
    pub LastDrift: TThostFtdcLastDriftType,
    #[doc = "成功值"]
    pub LastSuccess: TThostFtdcLastSuccessType,
    #[doc = "动态令牌类型"]
    pub OTPType: TThostFtdcOTPTypeType,
}
impl Default for CThostFtdcBrokerUserOTPParamField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "手工同步用户动态令牌"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcManualSyncBrokerUserOTPField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "动态令牌类型"]
    pub OTPType: TThostFtdcOTPTypeType,
    #[doc = "第一个动态密码"]
    #[serde(with = "BigArray")]
    pub FirstOTP: TThostFtdcPasswordType,
    #[doc = "第二个动态密码"]
    #[serde(with = "BigArray")]
    pub SecondOTP: TThostFtdcPasswordType,
}
impl Default for CThostFtdcManualSyncBrokerUserOTPField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "投资者手续费率模板"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcCommRateModelField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "手续费率模板代码"]
    pub CommModelID: TThostFtdcInvestorIDType,
    #[doc = "模板名称"]
    #[serde(with = "BigArray")]
    pub CommModelName: TThostFtdcCommModelNameType,
}
impl Default for CThostFtdcCommRateModelField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "请求查询投资者手续费率模板"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCommRateModelField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "手续费率模板代码"]
    pub CommModelID: TThostFtdcInvestorIDType,
}
#[doc = "投资者保证金率模板"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcMarginModelField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "保证金率模板代码"]
    pub MarginModelID: TThostFtdcInvestorIDType,
    #[doc = "模板名称"]
    #[serde(with = "BigArray")]
    pub MarginModelName: TThostFtdcCommModelNameType,
}
impl Default for CThostFtdcMarginModelField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "请求查询投资者保证金率模板"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryMarginModelField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "保证金率模板代码"]
    pub MarginModelID: TThostFtdcInvestorIDType,
}
#[doc = "仓单折抵信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcEWarrantOffsetField {
    #[doc = "交易日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "买卖方向"]
    pub Direction: TThostFtdcDirectionType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "数量"]
    pub Volume: TThostFtdcVolumeType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询仓单折抵信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryEWarrantOffsetField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询投资者品种/跨品种保证金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryInvestorProductGroupMarginField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "品种/跨品种标示"]
    pub ProductGroupID: TThostFtdcInstrumentIDType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "投资者品种/跨品种保证金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcInvestorProductGroupMarginField {
    #[doc = "品种/跨品种标示"]
    pub ProductGroupID: TThostFtdcInstrumentIDType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "结算编号"]
    pub SettlementID: TThostFtdcSettlementIDType,
    #[doc = "冻结的保证金"]
    pub FrozenMargin: TThostFtdcMoneyType,
    #[doc = "多头冻结的保证金"]
    pub LongFrozenMargin: TThostFtdcMoneyType,
    #[doc = "空头冻结的保证金"]
    pub ShortFrozenMargin: TThostFtdcMoneyType,
    #[doc = "占用的保证金"]
    pub UseMargin: TThostFtdcMoneyType,
    #[doc = "多头保证金"]
    pub LongUseMargin: TThostFtdcMoneyType,
    #[doc = "空头保证金"]
    pub ShortUseMargin: TThostFtdcMoneyType,
    #[doc = "交易所保证金"]
    pub ExchMargin: TThostFtdcMoneyType,
    #[doc = "交易所多头保证金"]
    pub LongExchMargin: TThostFtdcMoneyType,
    #[doc = "交易所空头保证金"]
    pub ShortExchMargin: TThostFtdcMoneyType,
    #[doc = "平仓盈亏"]
    pub CloseProfit: TThostFtdcMoneyType,
    #[doc = "冻结的手续费"]
    pub FrozenCommission: TThostFtdcMoneyType,
    #[doc = "手续费"]
    pub Commission: TThostFtdcMoneyType,
    #[doc = "冻结的资金"]
    pub FrozenCash: TThostFtdcMoneyType,
    #[doc = "资金差额"]
    pub CashIn: TThostFtdcMoneyType,
    #[doc = "持仓盈亏"]
    pub PositionProfit: TThostFtdcMoneyType,
    #[doc = "折抵总金额"]
    pub OffsetAmount: TThostFtdcMoneyType,
    #[doc = "多头折抵总金额"]
    pub LongOffsetAmount: TThostFtdcMoneyType,
    #[doc = "空头折抵总金额"]
    pub ShortOffsetAmount: TThostFtdcMoneyType,
    #[doc = "交易所折抵总金额"]
    pub ExchOffsetAmount: TThostFtdcMoneyType,
    #[doc = "交易所多头折抵总金额"]
    pub LongExchOffsetAmount: TThostFtdcMoneyType,
    #[doc = "交易所空头折抵总金额"]
    pub ShortExchOffsetAmount: TThostFtdcMoneyType,
    #[doc = "投机套保标志"]
    pub HedgeFlag: TThostFtdcHedgeFlagType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "查询监控中心用户令牌"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQueryCFMMCTradingAccountTokenField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "投资单元代码"]
    pub InvestUnitID: TThostFtdcInvestUnitIDType,
}
#[doc = "监控中心用户令牌"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCFMMCTradingAccountTokenField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "经纪公司统一编码"]
    pub ParticipantID: TThostFtdcParticipantIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "密钥编号"]
    pub KeyID: TThostFtdcSequenceNoType,
    #[doc = "动态令牌"]
    pub Token: TThostFtdcCFMMCTokenType,
}
#[doc = "查询产品组"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryProductGroupField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
}
#[doc = "投资者品种/跨品种保证金产品组"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcProductGroupField {
    #[doc = "产品代码"]
    pub ProductID: TThostFtdcInstrumentIDType,
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "产品组代码"]
    pub ProductGroupID: TThostFtdcInstrumentIDType,
}
#[doc = "交易所公告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcBulletinField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "公告编号"]
    pub BulletinID: TThostFtdcBulletinIDType,
    #[doc = "序列号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "公告类型"]
    pub NewsType: TThostFtdcNewsTypeType,
    #[doc = "紧急程度"]
    pub NewsUrgency: TThostFtdcNewsUrgencyType,
    #[doc = "发送时间"]
    pub SendTime: TThostFtdcTimeType,
    #[doc = "消息摘要"]
    #[serde(with = "BigArray")]
    pub Abstract: TThostFtdcAbstractType,
    #[doc = "消息来源"]
    pub ComeFrom: TThostFtdcComeFromType,
    #[doc = "消息正文"]
    #[serde(with = "BigArray")]
    pub Content: TThostFtdcContentType,
    #[doc = "WEB地址"]
    #[serde(with = "BigArray")]
    pub URLLink: TThostFtdcURLLinkType,
    #[doc = "市场代码"]
    pub MarketID: TThostFtdcMarketIDType,
}
impl Default for CThostFtdcBulletinField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询交易所公告"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryBulletinField {
    #[doc = "交易所代码"]
    pub ExchangeID: TThostFtdcExchangeIDType,
    #[doc = "公告编号"]
    pub BulletinID: TThostFtdcBulletinIDType,
    #[doc = "序列号"]
    pub SequenceNo: TThostFtdcSequenceNoType,
    #[doc = "公告类型"]
    pub NewsType: TThostFtdcNewsTypeType,
    #[doc = "紧急程度"]
    pub NewsUrgency: TThostFtdcNewsUrgencyType,
}
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
#[doc = "MulticastInstrument"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMulticastInstrumentField {
    #[doc = "主题号"]
    pub TopicID: TThostFtdcInstallIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
    #[doc = "合约编号"]
    pub InstrumentNo: TThostFtdcInstallIDType,
    #[doc = "基准价"]
    pub CodePrice: TThostFtdcPriceType,
    #[doc = "合约数量乘数"]
    pub VolumeMultiple: TThostFtdcVolumeMultipleType,
    #[doc = "最小变动价位"]
    pub PriceTick: TThostFtdcPriceType,
}
#[doc = "QryMulticastInstrument"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryMulticastInstrumentField {
    #[doc = "主题号"]
    pub TopicID: TThostFtdcInstallIDType,
    #[doc = "合约代码"]
    pub InstrumentID: TThostFtdcInstrumentIDType,
}
#[doc = "App客户端权限分配"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcAppIDAuthAssignField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "App代码"]
    #[serde(with="BigArray")]
    pub AppID: TThostFtdcAppIDType,
    #[doc = "交易中心代码"]
    pub DRIdentityID: TThostFtdcDRIdentityIDType,
}
impl Default for CThostFtdcAppIDAuthAssignField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
}}

#[doc = "转帐开户请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqOpenAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇钞标志"]
    pub CashExchangeCode: TThostFtdcCashExchangeCodeType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqOpenAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "转帐销户请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqCancelAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇钞标志"]
    pub CashExchangeCode: TThostFtdcCashExchangeCodeType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqCancelAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "变更银行账户请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqChangeAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "新银行帐号"]
    #[serde(with = "BigArray")]
    pub NewBankAccount: TThostFtdcBankAccountType,
    #[doc = "新银行密码"]
    #[serde(with = "BigArray")]
    pub NewBankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqChangeAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "转账请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqTransferField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "期货可取金额"]
    pub FutureFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "费用支付标志"]
    pub FeePayFlag: TThostFtdcFeePayFlagType,
    #[doc = "应收客户费用"]
    pub CustFee: TThostFtdcCustFeeType,
    #[doc = "应收期货公司费用"]
    pub BrokerFee: TThostFtdcFutureFeeType,
    #[doc = "发送方给接收方的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "转账交易状态"]
    pub TransferStatus: TThostFtdcTransferStatusType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqTransferField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银行发起银行资金转期货响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspTransferField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "期货可取金额"]
    pub FutureFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "费用支付标志"]
    pub FeePayFlag: TThostFtdcFeePayFlagType,
    #[doc = "应收客户费用"]
    pub CustFee: TThostFtdcCustFeeType,
    #[doc = "应收期货公司费用"]
    pub BrokerFee: TThostFtdcFutureFeeType,
    #[doc = "发送方给接收方的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "转账交易状态"]
    pub TransferStatus: TThostFtdcTransferStatusType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcRspTransferField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "冲正请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqRepealField {
    #[doc = "冲正时间间隔"]
    pub RepealTimeInterval: TThostFtdcRepealTimeIntervalType,
    #[doc = "已经冲正次数"]
    pub RepealedTimes: TThostFtdcRepealedTimesType,
    #[doc = "银行冲正标志"]
    pub BankRepealFlag: TThostFtdcBankRepealFlagType,
    #[doc = "期商冲正标志"]
    pub BrokerRepealFlag: TThostFtdcBrokerRepealFlagType,
    #[doc = "被冲正平台流水号"]
    pub PlateRepealSerial: TThostFtdcPlateSerialType,
    #[doc = "被冲正银行流水号"]
    pub BankRepealSerial: TThostFtdcBankSerialType,
    #[doc = "被冲正期货流水号"]
    pub FutureRepealSerial: TThostFtdcFutureSerialType,
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "期货可取金额"]
    pub FutureFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "费用支付标志"]
    pub FeePayFlag: TThostFtdcFeePayFlagType,
    #[doc = "应收客户费用"]
    pub CustFee: TThostFtdcCustFeeType,
    #[doc = "应收期货公司费用"]
    pub BrokerFee: TThostFtdcFutureFeeType,
    #[doc = "发送方给接收方的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "转账交易状态"]
    pub TransferStatus: TThostFtdcTransferStatusType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqRepealField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "冲正响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspRepealField {
    #[doc = "冲正时间间隔"]
    pub RepealTimeInterval: TThostFtdcRepealTimeIntervalType,
    #[doc = "已经冲正次数"]
    pub RepealedTimes: TThostFtdcRepealedTimesType,
    #[doc = "银行冲正标志"]
    pub BankRepealFlag: TThostFtdcBankRepealFlagType,
    #[doc = "期商冲正标志"]
    pub BrokerRepealFlag: TThostFtdcBrokerRepealFlagType,
    #[doc = "被冲正平台流水号"]
    pub PlateRepealSerial: TThostFtdcPlateSerialType,
    #[doc = "被冲正银行流水号"]
    pub BankRepealSerial: TThostFtdcBankSerialType,
    #[doc = "被冲正期货流水号"]
    pub FutureRepealSerial: TThostFtdcFutureSerialType,
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "期货可取金额"]
    pub FutureFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "费用支付标志"]
    pub FeePayFlag: TThostFtdcFeePayFlagType,
    #[doc = "应收客户费用"]
    pub CustFee: TThostFtdcCustFeeType,
    #[doc = "应收期货公司费用"]
    pub BrokerFee: TThostFtdcFutureFeeType,
    #[doc = "发送方给接收方的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "转账交易状态"]
    pub TransferStatus: TThostFtdcTransferStatusType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcRspRepealField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询账户信息请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqQueryAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqQueryAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询账户信息响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspQueryAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "银行可用金额"]
    pub BankUseAmount: TThostFtdcTradeAmountType,
    #[doc = "银行可取金额"]
    pub BankFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcRspQueryAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期商签到签退"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcFutureSignIOField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
}
impl Default for CThostFtdcFutureSignIOField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期商签到响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspFutureSignInField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "PIN密钥"]
    #[serde(with = "BigArray")]
    pub PinKey: TThostFtdcPasswordKeyType,
    #[doc = "MAC密钥"]
    #[serde(with = "BigArray")]
    pub MacKey: TThostFtdcPasswordKeyType,
}
impl Default for CThostFtdcRspFutureSignInField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期商签退请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqFutureSignOutField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
}
impl Default for CThostFtdcReqFutureSignOutField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期商签退响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspFutureSignOutField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcRspFutureSignOutField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询指定流水号的交易结果请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqQueryTradeResultBySerialField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "流水号"]
    pub Reference: TThostFtdcSerialType,
    #[doc = "本流水号发布者的机构类型"]
    pub RefrenceIssureType: TThostFtdcInstitutionTypeType,
    #[doc = "本流水号发布者机构编码"]
    #[serde(with = "BigArray")]
    pub RefrenceIssure: TThostFtdcOrganCodeType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcReqQueryTradeResultBySerialField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询指定流水号的交易结果响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspQueryTradeResultBySerialField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "流水号"]
    pub Reference: TThostFtdcSerialType,
    #[doc = "本流水号发布者的机构类型"]
    pub RefrenceIssureType: TThostFtdcInstitutionTypeType,
    #[doc = "本流水号发布者机构编码"]
    #[serde(with = "BigArray")]
    pub RefrenceIssure: TThostFtdcOrganCodeType,
    #[doc = "原始返回代码"]
    pub OriginReturnCode: TThostFtdcReturnCodeType,
    #[doc = "原始返回码描述"]
    #[serde(with = "BigArray")]
    pub OriginDescrInfoForReturnCode: TThostFtdcDescrInfoForReturnCodeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "转帐金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
}
impl Default for CThostFtdcRspQueryTradeResultBySerialField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "日终文件就绪请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqDayEndFileReadyField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "文件业务功能"]
    pub FileBusinessCode: TThostFtdcFileBusinessCodeType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
}
impl Default for CThostFtdcReqDayEndFileReadyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "返回结果"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReturnResultField {
    #[doc = "返回代码"]
    pub ReturnCode: TThostFtdcReturnCodeType,
    #[doc = "返回码描述"]
    #[serde(with = "BigArray")]
    pub DescrInfoForReturnCode: TThostFtdcDescrInfoForReturnCodeType,
}
impl Default for CThostFtdcReturnResultField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "验证期货资金密码"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcVerifyFuturePasswordField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcVerifyFuturePasswordField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "验证客户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcVerifyCustInfoField {
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcVerifyCustInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "验证期货资金密码和客户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcVerifyFuturePasswordAndCustInfoField {
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcVerifyFuturePasswordAndCustInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "验证期货资金密码和客户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcDepositResultInformField {
    #[doc = "出入金流水号，该流水号为银期报盘返回的流水号"]
    pub DepositSeqNo: TThostFtdcDepositSeqNoType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "入金金额"]
    pub Deposit: TThostFtdcMoneyType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "返回代码"]
    pub ReturnCode: TThostFtdcReturnCodeType,
    #[doc = "返回码描述"]
    #[serde(with = "BigArray")]
    pub DescrInfoForReturnCode: TThostFtdcDescrInfoForReturnCodeType,
}
impl Default for CThostFtdcDepositResultInformField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易核心向银期报盘发出密钥同步请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqSyncKeyField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易核心给银期报盘的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
}
impl Default for CThostFtdcReqSyncKeyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易核心向银期报盘发出密钥同步响应"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspSyncKeyField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易核心给银期报盘的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcRspSyncKeyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询账户信息通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcNotifyQueryAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "银行可用金额"]
    pub BankUseAmount: TThostFtdcTradeAmountType,
    #[doc = "银行可取金额"]
    pub BankFetchAmount: TThostFtdcTradeAmountType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcNotifyQueryAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期转账交易流水表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcTransferSerialField {
    #[doc = "平台流水号"]
    pub PlateSerial: TThostFtdcPlateSerialType,
    #[doc = "交易发起方日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易日期"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "交易代码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "会话编号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "银行编码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构编码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "期货公司编码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "期货公司帐号类型"]
    pub FutureAccType: TThostFtdcFutureAccTypeType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "期货公司流水号"]
    pub FutureSerial: TThostFtdcFutureSerialType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "交易金额"]
    pub TradeAmount: TThostFtdcTradeAmountType,
    #[doc = "应收客户费用"]
    pub CustFee: TThostFtdcCustFeeType,
    #[doc = "应收期货公司费用"]
    pub BrokerFee: TThostFtdcFutureFeeType,
    #[doc = "有效标志"]
    pub AvailabilityFlag: TThostFtdcAvailabilityFlagType,
    #[doc = "操作员"]
    pub OperatorCode: TThostFtdcOperatorCodeType,
    #[doc = "新银行帐号"]
    #[serde(with = "BigArray")]
    pub BankNewAccount: TThostFtdcBankAccountType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcTransferSerialField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "请求查询转帐流水"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryTransferSerialField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "银行编码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "期商签到通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcNotifyFutureSignInField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "PIN密钥"]
    #[serde(with = "BigArray")]
    pub PinKey: TThostFtdcPasswordKeyType,
    #[doc = "MAC密钥"]
    #[serde(with = "BigArray")]
    pub MacKey: TThostFtdcPasswordKeyType,
}
impl Default for CThostFtdcNotifyFutureSignInField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "期商签退通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcNotifyFutureSignOutField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcNotifyFutureSignOutField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "交易核心向银期报盘发出密钥同步处理结果的通知"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcNotifySyncKeyField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易核心给银期报盘的消息"]
    #[serde(with = "BigArray")]
    pub Message: TThostFtdcAddInfoType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "请求编号"]
    pub RequestID: TThostFtdcRequestIDType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcNotifySyncKeyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "请求查询银期签约关系"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryAccountregisterField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "银行编码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构编码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "客户开销户信息表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcAccountregisterField {
    #[doc = "交易日期"]
    pub TradeDay: TThostFtdcTradeDateType,
    #[doc = "银行编码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构编码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "期货公司编码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期货公司分支机构编码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "开销户类别"]
    pub OpenOrDestroy: TThostFtdcOpenOrDestroyType,
    #[doc = "签约日期"]
    pub RegDate: TThostFtdcTradeDateType,
    #[doc = "解约日期"]
    pub OutDate: TThostFtdcTradeDateType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcAccountregisterField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期开户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcOpenAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇钞标志"]
    pub CashExchangeCode: TThostFtdcCashExchangeCodeType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcOpenAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期销户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcCancelAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "汇钞标志"]
    pub CashExchangeCode: TThostFtdcCashExchangeCodeType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "渠道标志"]
    pub DeviceID: TThostFtdcDeviceIDType,
    #[doc = "期货单位帐号类型"]
    pub BankSecuAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "期货单位帐号"]
    #[serde(with = "BigArray")]
    pub BankSecuAcc: TThostFtdcBankAccountType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易柜员"]
    pub OperNo: TThostFtdcOperNoType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "用户标识"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcCancelAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期变更银行账号信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcChangeAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "新银行帐号"]
    #[serde(with = "BigArray")]
    pub NewBankAccount: TThostFtdcBankAccountType,
    #[doc = "新银行密码"]
    #[serde(with = "BigArray")]
    pub NewBankPassWord: TThostFtdcPasswordType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "银行密码标志"]
    pub BankPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "期货资金密码核对标志"]
    pub SecuPwdFlag: TThostFtdcPwdFlagType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
    #[doc = "长客户姓名"]
    #[serde(with = "BigArray")]
    pub LongCustomerName: TThostFtdcLongIndividualNameType,
}
impl Default for CThostFtdcChangeAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "二级代理操作员银期权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSecAgentACIDMapField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "资金账户"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "境外中介机构资金帐号"]
    pub BrokerSecAgentID: TThostFtdcAccountIDType,
}
#[doc = "二级代理操作员银期权限查询"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySecAgentACIDMapField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "资金账户"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "币种"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "灾备中心交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcUserRightsAssignField {
    #[doc = "应用单元代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "交易中心代码"]
    pub DRIdentityID: TThostFtdcDRIdentityIDType,
}
#[doc = "经济公司是否有在本标示的交易权限"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcBrokerUserRightAssignField {
    #[doc = "应用单元代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "交易中心代码"]
    pub DRIdentityID: TThostFtdcDRIdentityIDType,
    #[doc = "能否交易"]
    pub Tradeable: TThostFtdcBoolType,
}
#[doc = "灾备交易转换报文"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcDRTransferField {
    #[doc = "原交易中心代码"]
    pub OrigDRIdentityID: TThostFtdcDRIdentityIDType,
    #[doc = "目标交易中心代码"]
    pub DestDRIdentityID: TThostFtdcDRIdentityIDType,
    #[doc = "原应用单元代码"]
    pub OrigBrokerID: TThostFtdcBrokerIDType,
    #[doc = "目标易用单元代码"]
    pub DestBrokerID: TThostFtdcBrokerIDType,
}
#[doc = "Fens用户信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcFensUserInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "登录模式"]
    pub LoginMode: TThostFtdcLoginModeType,
}
#[doc = "当前银期所属交易中心"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCurrTransferIdentityField {
    #[doc = "交易中心代码"]
    pub IdentityID: TThostFtdcDRIdentityIDType,
}
#[doc = "禁止登录用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcLoginForbiddenUserField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "查询禁止登录用户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryLoginForbiddenUserField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "UDP组播组信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcMulticastGroupInfoField {
    #[doc = "组播组IP地址"]
    pub GroupIP: TThostFtdcIPAddressType,
    #[doc = "组播组IP端口"]
    pub GroupPort: TThostFtdcIPPortType,
    #[doc = "源地址"]
    pub SourceIP: TThostFtdcIPAddressType,
}
#[doc = "资金账户基本准备金"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcTradingAccountReserveField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "基本准备金"]
    pub Reserve: TThostFtdcMoneyType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
#[doc = "查询禁止登录IP"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryLoginForbiddenIPField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "查询IP列表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryIPListField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "查询用户下单权限分配表"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryUserRightsAssignField {
    #[doc = "应用单元代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "银期预约开户确认请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReserveOpenAccountConfirmField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcLongIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "期货密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "预约开户银行流水号"]
    pub BankReserveOpenSeq: TThostFtdcBankSerialType,
    #[doc = "预约开户日期"]
    pub BookDate: TThostFtdcTradeDateType,
    #[doc = "预约开户验证密码"]
    #[serde(with = "BigArray")]
    pub BookPsw: TThostFtdcPasswordType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcReserveOpenAccountConfirmField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银期预约开户"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReserveOpenAccountField {
    #[doc = "业务功能码"]
    pub TradeCode: TThostFtdcTradeCodeType,
    #[doc = "银行代码"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行分支机构代码"]
    pub BankBranchID: TThostFtdcBankBrchIDType,
    #[doc = "期商代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "期商分支机构代码"]
    pub BrokerBranchID: TThostFtdcFutureBranchIDType,
    #[doc = "交易日期"]
    pub TradeDate: TThostFtdcTradeDateType,
    #[doc = "交易时间"]
    pub TradeTime: TThostFtdcTradeTimeType,
    #[doc = "银行流水号"]
    pub BankSerial: TThostFtdcBankSerialType,
    #[doc = "交易系统日期"]
    pub TradingDay: TThostFtdcTradeDateType,
    #[doc = "银期平台消息流水号"]
    pub PlateSerial: TThostFtdcSerialType,
    #[doc = "最后分片标志"]
    pub LastFragment: TThostFtdcLastFragmentType,
    #[doc = "会话号"]
    pub SessionID: TThostFtdcSessionIDType,
    #[doc = "客户姓名"]
    #[serde(with = "BigArray")]
    pub CustomerName: TThostFtdcLongIndividualNameType,
    #[doc = "证件类型"]
    pub IdCardType: TThostFtdcIdCardTypeType,
    #[doc = "证件号码"]
    #[serde(with = "BigArray")]
    pub IdentifiedCardNo: TThostFtdcIdentifiedCardNoType,
    #[doc = "性别"]
    pub Gender: TThostFtdcGenderType,
    #[doc = "国家代码"]
    pub CountryCode: TThostFtdcCountryCodeType,
    #[doc = "客户类型"]
    pub CustType: TThostFtdcCustTypeType,
    #[doc = "地址"]
    #[serde(with = "BigArray")]
    pub Address: TThostFtdcAddressType,
    #[doc = "邮编"]
    pub ZipCode: TThostFtdcZipCodeType,
    #[doc = "电话号码"]
    #[serde(with = "BigArray")]
    pub Telephone: TThostFtdcTelephoneType,
    #[doc = "手机"]
    pub MobilePhone: TThostFtdcMobilePhoneType,
    #[doc = "传真"]
    #[serde(with = "BigArray")]
    pub Fax: TThostFtdcFaxType,
    #[doc = "电子邮件"]
    #[serde(with = "BigArray")]
    pub EMail: TThostFtdcEMailType,
    #[doc = "资金账户状态"]
    pub MoneyAccountStatus: TThostFtdcMoneyAccountStatusType,
    #[doc = "银行帐号"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行密码"]
    #[serde(with = "BigArray")]
    pub BankPassWord: TThostFtdcPasswordType,
    #[doc = "安装编号"]
    pub InstallID: TThostFtdcInstallIDType,
    #[doc = "验证客户证件号码标志"]
    pub VerifyCertNoFlag: TThostFtdcYesNoIndicatorType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "摘要"]
    #[serde(with = "BigArray")]
    pub Digest: TThostFtdcDigestType,
    #[doc = "银行帐号类型"]
    pub BankAccType: TThostFtdcBankAccTypeType,
    #[doc = "期货公司银行编码"]
    #[serde(with = "BigArray")]
    pub BrokerIDByBank: TThostFtdcBankCodingForFutureType,
    #[doc = "交易ID"]
    pub TID: TThostFtdcTIDType,
    #[doc = "预约开户状态"]
    pub ReserveOpenAccStas: TThostFtdcReserveOpenAccStasType,
    #[doc = "错误代码"]
    pub ErrorID: TThostFtdcErrorIDType,
    #[doc = "错误信息"]
    #[serde(with = "BigArray")]
    pub ErrorMsg: TThostFtdcErrorMsgType,
}
impl Default for CThostFtdcReserveOpenAccountField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "银行账户属性"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcAccountPropertyField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者帐号"]
    pub AccountID: TThostFtdcAccountIDType,
    #[doc = "银行统一标识类型"]
    pub BankID: TThostFtdcBankIDType,
    #[doc = "银行账户"]
    #[serde(with = "BigArray")]
    pub BankAccount: TThostFtdcBankAccountType,
    #[doc = "银行账户的开户人名称"]
    #[serde(with = "BigArray")]
    pub OpenName: TThostFtdcInvestorFullNameType,
    #[doc = "银行账户的开户行"]
    #[serde(with = "BigArray")]
    pub OpenBank: TThostFtdcOpenBankType,
    #[doc = "是否活跃"]
    pub IsActive: TThostFtdcBoolType,
    #[doc = "账户来源"]
    pub AccountSourceType: TThostFtdcAccountSourceTypeType,
    #[doc = "开户日期"]
    pub OpenDate: TThostFtdcDateType,
    #[doc = "注销日期"]
    pub CancelDate: TThostFtdcDateType,
    #[doc = "录入员代码"]
    #[serde(with = "BigArray")]
    pub OperatorID: TThostFtdcOperatorIDType,
    #[doc = "录入日期"]
    pub OperateDate: TThostFtdcDateType,
    #[doc = "录入时间"]
    pub OperateTime: TThostFtdcTimeType,
    #[doc = "币种代码"]
    pub CurrencyID: TThostFtdcCurrencyIDType,
}
impl Default for CThostFtdcAccountPropertyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "查询当前交易中心"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryCurrDRIdentityField {
    #[doc = "交易中心代码"]
    pub DRIdentityID: TThostFtdcDRIdentityIDType,
}
#[doc = "当前交易中心"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcCurrDRIdentityField {
    #[doc = "交易中心代码"]
    pub DRIdentityID: TThostFtdcDRIdentityIDType,
}
#[doc = "查询二级代理商资金校验模式"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySecAgentCheckModeField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "查询二级代理商信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQrySecAgentTradeInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "境外中介机构资金帐号"]
    pub BrokerSecAgentID: TThostFtdcAccountIDType,
}
#[doc = "用户系统信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcUserSystemInfoField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "用户端系统内部信息长度"]
    pub ClientSystemInfoLen: TThostFtdcSystemInfoLenType,
    #[doc = "用户端系统内部信息"]
    #[serde(with = "BigArray")]
    pub ClientSystemInfo: TThostFtdcClientSystemInfoType,
    #[doc = "用户公网IP"]
    pub ClientPublicIP: TThostFtdcIPAddressType,
    #[doc = "终端IP端口"]
    pub ClientIPPort: TThostFtdcIPPortType,
    #[doc = "登录成功时间"]
    pub ClientLoginTime: TThostFtdcTimeType,
    #[doc = "App代码"]
    #[serde(with = "BigArray")]
    pub ClientAppID: TThostFtdcAppIDType,
}
impl Default for CThostFtdcUserSystemInfoField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户发出获取安全安全登陆方法请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcReqUserAuthMethodField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "用户发出获取安全安全登陆方法回复"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcRspUserAuthMethodField {
    #[doc = "当前可以用的认证模式"]
    pub UsableAuthMethod: TThostFtdcCurrentAuthMethodType,
}
#[doc = "用户发出获取安全安全登陆方法请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcReqGenUserCaptchaField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "生成的图片验证码信息"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspGenUserCaptchaField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "图片信息长度"]
    pub CaptchaInfoLen: TThostFtdcCaptchaInfoLenType,
    #[doc = "图片信息"]
    #[serde(with = "BigArray")]
    pub CaptchaInfo: TThostFtdcCaptchaInfoType,
}
impl Default for CThostFtdcRspGenUserCaptchaField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户发出获取安全安全登陆方法请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcReqGenUserTextField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
}
#[doc = "短信验证码生成的回复"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcRspGenUserTextField {
    #[doc = "短信验证码序号"]
    pub UserTextSeq: TThostFtdcUserTextSeqType,
}
#[doc = "用户发出带图形验证码的登录请求请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqUserLoginWithCaptchaField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
    #[doc = "图形验证码的文字内容"]
    #[serde(with = "BigArray")]
    pub Captcha: TThostFtdcPasswordType,
    #[doc = "终端IP端口"]
    pub ClientIPPort: TThostFtdcIPPortType,
}
impl Default for CThostFtdcReqUserLoginWithCaptchaField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户发出带短信验证码的登录请求请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqUserLoginWithTextField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
    #[doc = "短信验证码文字内容"]
    #[serde(with = "BigArray")]
    pub Text: TThostFtdcPasswordType,
    #[doc = "终端IP端口"]
    pub ClientIPPort: TThostFtdcIPPortType,
}
impl Default for CThostFtdcReqUserLoginWithTextField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "用户发出带动态验证码的登录请求请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqUserLoginWithOTPField {
    #[doc = "交易日"]
    pub TradingDay: TThostFtdcDateType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "密码"]
    #[serde(with = "BigArray")]
    pub Password: TThostFtdcPasswordType,
    #[doc = "用户端产品信息"]
    pub UserProductInfo: TThostFtdcProductInfoType,
    #[doc = "接口端产品信息"]
    pub InterfaceProductInfo: TThostFtdcProductInfoType,
    #[doc = "协议信息"]
    pub ProtocolInfo: TThostFtdcProtocolInfoType,
    #[doc = "Mac地址"]
    pub MacAddress: TThostFtdcMacAddressType,
    #[doc = "终端IP地址"]
    pub ClientIPAddress: TThostFtdcIPAddressType,
    #[doc = "登录备注"]
    #[serde(with = "BigArray")]
    pub LoginRemark: TThostFtdcLoginRemarkType,
    #[doc = "OTP密码"]
    #[serde(with = "BigArray")]
    pub OTPPassword: TThostFtdcPasswordType,
    #[doc = "终端IP端口"]
    pub ClientIPPort: TThostFtdcIPPortType,
}
impl Default for CThostFtdcReqUserLoginWithOTPField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "api握手请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcReqApiHandshakeField {
    #[doc = "api与front通信密钥版本号"]
    pub CryptoKeyVersion: TThostFtdcCryptoKeyVersionType,
}
#[doc = "front发给api的握手回复"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcRspApiHandshakeField {
    #[doc = "握手回复数据长度"]
    pub FrontHandshakeDataLen: TThostFtdcHandshakeDataLenType,
    #[doc = "握手回复数据"]
    #[serde(with = "BigArray")]
    pub FrontHandshakeData: TThostFtdcHandshakeDataType,
    #[doc = "API认证是否开启"]
    pub IsApiAuthEnabled: TThostFtdcBoolType,
}
impl Default for CThostFtdcRspApiHandshakeField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "api给front的验证key的请求"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct CThostFtdcReqVerifyApiKeyField {
    #[doc = "握手回复数据长度"]
    pub ApiHandshakeDataLen: TThostFtdcHandshakeDataLenType,
    #[doc = "握手回复数据"]
    #[serde(with = "BigArray")]
    pub ApiHandshakeData: TThostFtdcHandshakeDataType,
}
impl Default for CThostFtdcReqVerifyApiKeyField {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[doc = "操作员组织架构关系"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcDepartmentUserField {
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "用户代码"]
    pub UserID: TThostFtdcUserIDType,
    #[doc = "投资者范围"]
    pub InvestorRange: TThostFtdcDepartmentRangeType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
}
#[doc = "查询频率，每秒查询比数"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQueryFreqField {
    #[doc = "查询频率"]
    pub QueryFreq: TThostFtdcQueryFreqType,
}
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
#[doc = "禁止认证IP"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcAuthForbiddenIPField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "查询禁止认证IP"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcQryAuthForbiddenIPField {
    #[doc = "IP地址"]
    pub IPAddress: TThostFtdcIPAddressType,
}
#[doc = "换汇可提冻结"]
#[repr(C)]
#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct CThostFtdcSyncDelaySwapFrozenField {
    #[doc = "换汇流水号"]
    pub DelaySwapSeqNo: TThostFtdcDepositSeqNoType,
    #[doc = "经纪公司代码"]
    pub BrokerID: TThostFtdcBrokerIDType,
    #[doc = "投资者代码"]
    pub InvestorID: TThostFtdcInvestorIDType,
    #[doc = "源币种"]
    pub FromCurrencyID: TThostFtdcCurrencyIDType,
    #[doc = "源剩余换汇额度(可提冻结)"]
    pub FromRemainSwap: TThostFtdcMoneyType,
    #[doc = "是否手工换汇"]
    pub IsManualSwap: TThostFtdcBoolType,
}
}}
#[repr(C)]
pub struct CThostFtdcMdSpi__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CThostFtdcMdSpi {
    pub vtable_: *const CThostFtdcMdSpi__bindgen_vtable,
}
impl Default for CThostFtdcMdSpi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CThostFtdcMdApi__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct CThostFtdcMdApi {
    pub vtable_: *const CThostFtdcMdApi__bindgen_vtable,
}

extern "C" {
    #[doc = "创建MdApi"]
    #[doc = "@param pszFlowPath 存贮订阅信息文件的目录，默认为当前目录"]
    #[doc = "@return 创建出的UserApi"]
    #[doc = "modify for udp marketdata"]
    #[link_name = link_name!("ZN15CThostFtdcMdApi15CreateFtdcMdApiEPKcbb")]
    pub fn CThostFtdcMdApi_CreateFtdcMdApi(
        pszFlowPath: *const ::std::os::raw::c_char,
        bIsUsingUdp: bool,
        bIsMulticast: bool,
    ) -> *mut CThostFtdcMdApi;
}

extern "C" {
    #[doc = "获取API的版本信息"]
    #[doc = "@retrun 获取到的版本号"]
    #[link_name = link_name!("ZN15CThostFtdcMdApi13GetApiVersionEv")]
    pub fn CThostFtdcMdApi_GetApiVersion() -> *const ::std::os::raw::c_char;
}
impl Default for CThostFtdcMdApi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl CThostFtdcMdApi {
    #[inline]
    pub unsafe fn CreateFtdcMdApi(
        pszFlowPath: *const ::std::os::raw::c_char,
        bIsUsingUdp: bool,
        bIsMulticast: bool,
    ) -> *mut CThostFtdcMdApi {
        CThostFtdcMdApi_CreateFtdcMdApi(pszFlowPath, bIsUsingUdp, bIsMulticast)
    }
    #[inline]
    pub unsafe fn GetApiVersion() -> *const ::std::os::raw::c_char {
        CThostFtdcMdApi_GetApiVersion()
    }
}
#[repr(C)]
pub struct CThostFtdcTraderSpi__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CThostFtdcTraderSpi {
    pub vtable_: *const CThostFtdcTraderSpi__bindgen_vtable,
}
impl Default for CThostFtdcTraderSpi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CThostFtdcTraderApi__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct CThostFtdcTraderApi {
    pub vtable_: *const CThostFtdcTraderApi__bindgen_vtable,
}

extern "C" {
    #[doc = "创建TraderApi"]
    #[doc = "@param pszFlowPath 存贮订阅信息文件的目录，默认为当前目录"]
    #[doc = "@return 创建出的UserApi"]
    #[link_name = link_name!("ZN19CThostFtdcTraderApi19CreateFtdcTraderApiEPKc")]
    pub fn CThostFtdcTraderApi_CreateFtdcTraderApi(
        pszFlowPath: *const ::std::os::raw::c_char,
    ) -> *mut CThostFtdcTraderApi;
}

extern "C" {
    #[doc = "获取API的版本信息"]
    #[doc = "@retrun 获取到的版本号"]
    #[link_name = link_name!("ZN19CThostFtdcTraderApi13GetApiVersionEv")]
    pub fn CThostFtdcTraderApi_GetApiVersion() -> *const ::std::os::raw::c_char;
}
impl Default for CThostFtdcTraderApi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl CThostFtdcTraderApi {
    #[inline]
    pub unsafe fn CreateFtdcTraderApi(
        pszFlowPath: *const ::std::os::raw::c_char,
    ) -> *mut CThostFtdcTraderApi {
        CThostFtdcTraderApi_CreateFtdcTraderApi(pszFlowPath)
    }
    #[inline]
    pub unsafe fn GetApiVersion() -> *const ::std::os::raw::c_char {
        CThostFtdcTraderApi_GetApiVersion()
    }
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTPMarketSpi {
    pub _base: CThostFtdcMdSpi,
    pub _mdspi_on_front_connected_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_front_disconnected_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_heartbeat_warning_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_error_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_qry_multicast_instrument_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_sub_for_quote_rsp_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_sub_market_data_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_unsub_for_quote_rsp_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_unsub_market_data_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_user_login_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rsp_user_logout_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rtn_depth_market_data_cb: *const ::std::os::raw::c_void,
    pub _mdspi_on_rtn_for_quote_rsp_cb: *const ::std::os::raw::c_void,
}
impl Default for CTPMarketSpi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi16OnFrontConnectedEv")]
    pub fn CTPMarketSpi_OnFrontConnected(this: *const ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi19OnFrontDisconnectedEi")]
    pub fn CTPMarketSpi_OnFrontDisconnected(
        this: *const ::std::os::raw::c_void,
        nReason: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi18OnHeartBeatWarningEi")]
    pub fn CTPMarketSpi_OnHeartBeatWarning(
        this: *const ::std::os::raw::c_void,
        nTimeLapse: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi14OnRspUserLoginEP27CThostFtdcRspUserLoginFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspUserLogin(
        this: *const ::std::os::raw::c_void,
        pRspUserLogin: *const CThostFtdcRspUserLoginField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi15OnRspUserLogoutEP25CThostFtdcUserLogoutFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspUserLogout(
        this: *const ::std::os::raw::c_void,
        pUserLogout: *const CThostFtdcUserLogoutField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi27OnRspQryMulticastInstrumentEP34CThostFtdcMulticastInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspQryMulticastInstrument(
        this: *const ::std::os::raw::c_void,
        pMulticastInstrument: *const CThostFtdcMulticastInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
}}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi10OnRspErrorEP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspError(
        this: *const ::std::os::raw::c_void,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi18OnRspSubMarketDataEP33CThostFtdcSpecificInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspSubMarketData(
        this: *const ::std::os::raw::c_void,
        pSpecificInstrument: *const CThostFtdcSpecificInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi20OnRspUnSubMarketDataEP33CThostFtdcSpecificInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspUnSubMarketData(
        this: *const ::std::os::raw::c_void,
        pSpecificInstrument: *const CThostFtdcSpecificInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi19OnRspSubForQuoteRspEP33CThostFtdcSpecificInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspSubForQuoteRsp(
        this: *const ::std::os::raw::c_void,
        pSpecificInstrument: *const CThostFtdcSpecificInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi21OnRspUnSubForQuoteRspEP33CThostFtdcSpecificInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPMarketSpi_OnRspUnSubForQuoteRsp(
        this: *const ::std::os::raw::c_void,
        pSpecificInstrument: *const CThostFtdcSpecificInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi20OnRtnDepthMarketDataEP30CThostFtdcDepthMarketDataField")]
    pub fn CTPMarketSpi_OnRtnDepthMarketData(
        this: *const ::std::os::raw::c_void,
        pDepthMarketData: *const CThostFtdcDepthMarketDataField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi16OnRtnForQuoteRspEP26CThostFtdcForQuoteRspField")]
    pub fn CTPMarketSpi_OnRtnForQuoteRsp(
        this: *const ::std::os::raw::c_void,
        pForQuoteRsp: *const CThostFtdcForQuoteRspField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPMarketSpi7ReleaseEv")]
    pub fn CTPMarketSpi_Release(this: *mut CTPMarketSpi);
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CTPTraderSpi {
    pub _base: CThostFtdcTraderSpi,
    pub _traderspi_on_front_connected_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_front_disconnected_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_heartbeat_warning_cb: *const ::std::os::raw::c_void,

    pub _traderspi_on_rsp_authenticate_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_user_login_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_user_logout_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_user_password_update_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_trading_account_password_update_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_user_auth_method_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_gen_user_captcha_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_gen_user_text_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_order_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_parked_order_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_parked_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_query_max_order_volume_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_settlement_info_confirm_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_remove_parked_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_remove_parked_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_exec_order_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_exec_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_for_quote_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_quote_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_quote_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_batch_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_option_self_close_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_option_self_close_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_comb_action_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_trade_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_investor_position_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_trading_account_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_investor_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_trading_code_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_instrument_margin_rate_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_instrument_commission_rate_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_exchange_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_product_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_instrument_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_depth_market_data_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_settlement_info_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_transfer_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_investor_position_detail_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_notice_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_error_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_contract_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_parked_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_parked_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_trading_notice_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_broker_trading_params_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_qry_broker_trading_algos_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_query_cfmmc_trading_account_token_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_from_bank_to_future_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_from_future_to_bank_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rsp_query_bank_account_money_by_future_cb: *const ::std::os::raw::c_void,

    pub _traderspi_on_rtn_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_exec_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_trade_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_quote_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_instrument_status_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_bulletin_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_trading_notice_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_error_conditional_order_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_for_quote_rsp_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_cfmmc_trading_account_token_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_option_self_close_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_comb_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_from_bank_to_future_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_from_future_to_bank_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_bank_to_future_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_future_to_bank_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_from_bank_to_future_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_from_future_to_bank_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_bank_to_future_by_future_manual_cb:
        *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_future_to_bank_by_future_manual_cb:
        *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_query_bank_balance_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_bank_to_future_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_repeal_from_future_to_bank_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_open_account_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_cancel_account_by_bank_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_rtn_change_account_by_bank_cb: *const ::std::os::raw::c_void,

    pub _traderspi_on_err_rtn_order_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_exec_order_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_exec_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_quote_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_quote_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_for_quote_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_batch_order_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_option_self_close_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_option_self_close_action_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_comb_action_insert_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_bank_to_future_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_future_to_bank_by_future_cb: *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_repeal_bank_to_future_by_future_manual_cb:
        *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_repeal_future_to_bank_by_future_manual_cb:
        *const ::std::os::raw::c_void,
    pub _traderspi_on_err_rtn_query_bank_balance_by_future_cb: *const ::std::os::raw::c_void,
}
impl Default for CTPTraderSpi {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnFrontConnectedEv")]
    pub fn CTPTraderSpi_OnFrontConnected(this: *const ::std::os::raw::c_void);
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnFrontDisconnectedEi")]
    pub fn CTPTraderSpi_OnFrontDisconnected(
        this: *const ::std::os::raw::c_void,
        nReason: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi18OnHeartBeatWarningEi")]
    pub fn CTPTraderSpi_OnHeartBeatWarning(
        this: *const ::std::os::raw::c_void,
        nTimeLapse: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi17OnRspAuthenticateEP30CThostFtdcRspAuthenticateFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspAuthenticate(
        this: *const ::std::os::raw::c_void,
        pRspAuthenticateField: *const CThostFtdcRspAuthenticateField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi14OnRspUserLoginEP27CThostFtdcRspUserLoginFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspUserLogin(
        this: *const ::std::os::raw::c_void,
        pRspUserLogin: *const CThostFtdcRspUserLoginField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi15OnRspUserLogoutEP25CThostFtdcUserLogoutFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspUserLogout(
        this: *const ::std::os::raw::c_void,
        pUserLogout: *const CThostFtdcUserLogoutField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi23OnRspUserPasswordUpdateEP33CThostFtdcUserPasswordUpdateFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspUserPasswordUpdate(
        this: *const ::std::os::raw::c_void,
        pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi33OnRspTradingAccountPasswordUpdateEP43CThostFtdcTradingAccountPasswordUpdateFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspTradingAccountPasswordUpdate(
        this: *const ::std::os::raw::c_void,
        pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnRspUserAuthMethodEP32CThostFtdcRspUserAuthMethodFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspUserAuthMethod(
        this: *const ::std::os::raw::c_void,
        pRspUserAuthMethod: *const CThostFtdcRspUserAuthMethodField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnRspGenUserCaptchaEP32CThostFtdcRspGenUserCaptchaFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspGenUserCaptcha(
        this: *const ::std::os::raw::c_void,
        pRspGenUserCaptcha: *const CThostFtdcRspGenUserCaptchaField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspGenUserTextEP29CThostFtdcRspGenUserTextFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspGenUserText(
        this: *const ::std::os::raw::c_void,
        pRspGenUserText: *const CThostFtdcRspGenUserTextField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspOrderInsertEP25CThostFtdcInputOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspOrderInsert(
        this: *const ::std::os::raw::c_void,
        pInputOrder: *const CThostFtdcInputOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnRspParkedOrderInsertEP26CThostFtdcParkedOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspParkedOrderInsert(
        this: *const ::std::os::raw::c_void,
        pParkedOrder: *const CThostFtdcParkedOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnRspParkedOrderActionEP32CThostFtdcParkedOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspParkedOrderAction(
        this: *const ::std::os::raw::c_void,
        pParkedOrderAction: *const CThostFtdcParkedOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspOrderActionEP31CThostFtdcInputOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspOrderAction(
        this: *const ::std::os::raw::c_void,
        pInputOrderAction: *const CThostFtdcInputOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnRspQueryMaxOrderVolumeEP34CThostFtdcQueryMaxOrderVolumeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQueryMaxOrderVolume(
        this: *const ::std::os::raw::c_void,
        pQueryMaxOrderVolume: *const CThostFtdcQueryMaxOrderVolumeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi26OnRspSettlementInfoConfirmEP36CThostFtdcSettlementInfoConfirmFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspSettlementInfoConfirm(
        this: *const ::std::os::raw::c_void,
        pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnRspRemoveParkedOrderEP32CThostFtdcRemoveParkedOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspRemoveParkedOrder(
        this: *const ::std::os::raw::c_void,
        pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi28OnRspRemoveParkedOrderActionEP38CThostFtdcRemoveParkedOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspRemoveParkedOrderAction(
        this: *const ::std::os::raw::c_void,
        pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi20OnRspExecOrderInsertEP29CThostFtdcInputExecOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspExecOrderInsert(
        this: *const ::std::os::raw::c_void,
        pInputExecOrder: *const CThostFtdcInputExecOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi20OnRspExecOrderActionEP35CThostFtdcInputExecOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspExecOrderAction(
        this: *const ::std::os::raw::c_void,
        pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnRspForQuoteInsertEP28CThostFtdcInputForQuoteFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspForQuoteInsert(
        this: *const ::std::os::raw::c_void,
        pInputForQuote: *const CThostFtdcInputForQuoteField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspQuoteInsertEP25CThostFtdcInputQuoteFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQuoteInsert(
        this: *const ::std::os::raw::c_void,
        pInputQuote: *const CThostFtdcInputQuoteField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspQuoteActionEP31CThostFtdcInputQuoteActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQuoteAction(
        this: *const ::std::os::raw::c_void,
        pInputQuoteAction: *const CThostFtdcInputQuoteActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi21OnRspBatchOrderActionEP36CThostFtdcInputBatchOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspBatchOrderAction(
        this: *const ::std::os::raw::c_void,
        pInputBatchOrderAction: *const CThostFtdcInputBatchOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi26OnRspOptionSelfCloseInsertEP35CThostFtdcInputOptionSelfCloseFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspOptionSelfCloseInsert(
        this: *const ::std::os::raw::c_void,
        pInputOptionSelfClose: *const CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi26OnRspOptionSelfCloseActionEP41CThostFtdcInputOptionSelfCloseActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspOptionSelfCloseAction(
        this: *const ::std::os::raw::c_void,
        pInputOptionSelfCloseAction: *const CThostFtdcInputOptionSelfCloseActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi21OnRspCombActionInsertEP30CThostFtdcInputCombActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspCombActionInsert(
        this: *const ::std::os::raw::c_void,
        pInputCombAction: *const CThostFtdcInputCombActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi13OnRspQryOrderEP20CThostFtdcOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryOrder(
        this: *const ::std::os::raw::c_void,
        pOrder: *const CThostFtdcOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi13OnRspQryTradeEP20CThostFtdcTradeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryTrade(
        this: *const ::std::os::raw::c_void,
        pTrade: *const CThostFtdcTradeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnRspQryInvestorPositionEP31CThostFtdcInvestorPositionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInvestorPosition(
        this: *const ::std::os::raw::c_void,
        pInvestorPosition: *const CThostFtdcInvestorPositionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnRspQryTradingAccountEP29CThostFtdcTradingAccountFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryTradingAccount(
        this: *const ::std::os::raw::c_void,
        pTradingAccount: *const CThostFtdcTradingAccountField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspQryInvestorEP23CThostFtdcInvestorFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInvestor(
        this: *const ::std::os::raw::c_void,
        pInvestor: *const CThostFtdcInvestorField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnRspQryTradingCodeEP26CThostFtdcTradingCodeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryTradingCode(
        this: *const ::std::os::raw::c_void,
        pTradingCode: *const CThostFtdcTradingCodeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi28OnRspQryInstrumentMarginRateEP35CThostFtdcInstrumentMarginRateFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInstrumentMarginRate(
        this: *const ::std::os::raw::c_void,
        pInstrumentMarginRate: *const CThostFtdcInstrumentMarginRateField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi32OnRspQryInstrumentCommissionRateEP39CThostFtdcInstrumentCommissionRateFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInstrumentCommissionRate(
        this: *const ::std::os::raw::c_void,
        pInstrumentCommissionRate: *const CThostFtdcInstrumentCommissionRateField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRspQryExchangeEP23CThostFtdcExchangeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryExchange(
        this: *const ::std::os::raw::c_void,
        pExchange: *const CThostFtdcExchangeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi15OnRspQryProductEP22CThostFtdcProductFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryProduct(
        this: *const ::std::os::raw::c_void,
        pProduct: *const CThostFtdcProductField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi18OnRspQryInstrumentEP25CThostFtdcInstrumentFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInstrument(
        this: *const ::std::os::raw::c_void,
        pInstrument: *const CThostFtdcInstrumentField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("_ZN12CTPTraderSpi23OnRspQryDepthMarketDataEP30CThostFtdcDepthMarketDataFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryDepthMarketData(
        this: *const ::std::os::raw::c_void,
        pDepthMarketData: *const CThostFtdcDepthMarketDataField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("_ZN12CTPTraderSpi22OnRspQrySettlementInfoEP29CThostFtdcSettlementInfoFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQrySettlementInfo(
        this: *const ::std::os::raw::c_void,
        pSettlementInfo: *const CThostFtdcSettlementInfoField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("_ZN12CTPTraderSpi20OnRspQryTransferBankEP27CThostFtdcTransferBankFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryTransferBank(
        this: *const ::std::os::raw::c_void,
        pTransferBank: *const CThostFtdcTransferBankField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("_ZN12CTPTraderSpi30OnRspQryInvestorPositionDetailEP37CThostFtdcInvestorPositionDetailFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryInvestorPositionDetail(
        this: *const ::std::os::raw::c_void,
        pInvestorPositionDetail: *const CThostFtdcInvestorPositionDetailField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("_ZN12CTPTraderSpi14OnRspQryNoticeEP21CThostFtdcNoticeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryNotice(
        this: *const ::std::os::raw::c_void,
        pNotice: *const CThostFtdcNoticeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi10OnRspErrorEP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspError(
        this: *const ::std::os::raw::c_void,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi10OnRtnOrderEP20CThostFtdcOrderField")]
    pub fn CTPTraderSpi_OnRtnOrder(
        this: *const ::std::os::raw::c_void,
        pOrder: *const CThostFtdcOrderField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi10OnRtnTradeEP20CThostFtdcTradeField")]
    pub fn CTPTraderSpi_OnRtnTrade(
        this: *const ::std::os::raw::c_void,
        pTrade: *const CThostFtdcTradeField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnErrRtnOrderInsertEP25CThostFtdcInputOrderFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnOrderInsert(
        this: *const ::std::os::raw::c_void,
        pInputOrder: *const CThostFtdcInputOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnErrRtnOrderActionEP26CThostFtdcOrderActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnOrderAction(
        this: *const ::std::os::raw::c_void,
        pOrderAction: *const CThostFtdcOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi21OnRtnInstrumentStatusEP31CThostFtdcInstrumentStatusField")]
    pub fn CTPTraderSpi_OnRtnInstrumentStatus(
        this: *const ::std::os::raw::c_void,
        pInstrumentStatus: *const CThostFtdcInstrumentStatusField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi13OnRtnBulletinEP23CThostFtdcBulletinField")]
    pub fn CTPTraderSpi_OnRtnBulletin(
        this: *const ::std::os::raw::c_void,
        pBulletin: *const CThostFtdcBulletinField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi18OnRtnTradingNoticeEP32CThostFtdcTradingNoticeInfoField")]
    pub fn CTPTraderSpi_OnRtnTradingNotice(
        this: *const ::std::os::raw::c_void,
        pTradingNoticeInfo: *const CThostFtdcTradingNoticeInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi26OnRtnErrorConditionalOrderEP36CThostFtdcErrorConditionalOrderField")]
    pub fn CTPTraderSpi_OnRtnErrorConditionalOrder(
        this: *const ::std::os::raw::c_void,
        pErrorConditionalOrder: *const CThostFtdcErrorConditionalOrderField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi14OnRtnExecOrderEP24CThostFtdcExecOrderField")]
    pub fn CTPTraderSpi_OnRtnExecOrder(
        this: *const ::std::os::raw::c_void,
        pExecOrder: *const CThostFtdcExecOrderField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi23OnErrRtnExecOrderInsertEP29CThostFtdcInputExecOrderFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnExecOrderInsert(
        this: *const ::std::os::raw::c_void,
        pInputExecOrder: *const CThostFtdcInputExecOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi23OnErrRtnExecOrderActionEP30CThostFtdcExecOrderActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnExecOrderAction(
        this: *const ::std::os::raw::c_void,
        pExecOrderAction: *const CThostFtdcExecOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnErrRtnForQuoteInsertEP28CThostFtdcInputForQuoteFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnForQuoteInsert(
        this: *const ::std::os::raw::c_void,
        pInputForQuote: *const CThostFtdcInputForQuoteField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi10OnRtnQuoteEP20CThostFtdcQuoteField")]
    pub fn CTPTraderSpi_OnRtnQuote(
        this: *const ::std::os::raw::c_void,
        pQuote: *const CThostFtdcQuoteField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnErrRtnQuoteInsertEP25CThostFtdcInputQuoteFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnQuoteInsert(
        this: *const ::std::os::raw::c_void,
        pInputQuote: *const CThostFtdcInputQuoteField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnErrRtnQuoteActionEP26CThostFtdcQuoteActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnQuoteAction(
        this: *const ::std::os::raw::c_void,
        pQuoteAction: *const CThostFtdcQuoteActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi16OnRtnForQuoteRspEP26CThostFtdcForQuoteRspField")]
    pub fn CTPTraderSpi_OnRtnForQuoteRsp(
        this: *const ::std::os::raw::c_void,
        pForQuoteRsp: *const CThostFtdcForQuoteRspField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRtnCFMMCTradingAccountTokenEP39CThostFtdcCFMMCTradingAccountTokenField")]
    pub fn CTPTraderSpi_OnRtnCFMMCTradingAccountToken(
        this: *const ::std::os::raw::c_void,
        pCFMMCTradingAccountToken: *const CThostFtdcCFMMCTradingAccountTokenField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnErrRtnBatchOrderActionEP31CThostFtdcBatchOrderActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnBatchOrderAction(
        this: *const ::std::os::raw::c_void,
        pBatchOrderAction: *const CThostFtdcBatchOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi20OnRtnOptionSelfCloseEP30CThostFtdcOptionSelfCloseField")]
    pub fn CTPTraderSpi_OnRtnOptionSelfClose(
        this: *const ::std::os::raw::c_void,
        pOptionSelfClose: *const CThostFtdcOptionSelfCloseField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnErrRtnOptionSelfCloseInsertEP35CThostFtdcInputOptionSelfCloseFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnOptionSelfCloseInsert(
        this: *const ::std::os::raw::c_void,
        pInputOptionSelfClose: *const CThostFtdcInputOptionSelfCloseField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnErrRtnOptionSelfCloseActionEP36CThostFtdcOptionSelfCloseActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnOptionSelfCloseAction(
        this: *const ::std::os::raw::c_void,
        pOptionSelfCloseAction: *const CThostFtdcOptionSelfCloseActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi15OnRtnCombActionEP25CThostFtdcCombActionField")]
    pub fn CTPTraderSpi_OnRtnCombAction(
        this: *const ::std::os::raw::c_void,
        pCombAction: *const CThostFtdcCombActionField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnErrRtnCombActionInsertEP30CThostFtdcInputCombActionFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnCombActionInsert(
        this: *const ::std::os::raw::c_void,
        pInputCombAction: *const CThostFtdcInputCombActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi20OnRspQryContractBankEP27CThostFtdcContractBankFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryContractBank(
        this: *const ::std::os::raw::c_void,
        pContractBank: *const CThostFtdcContractBankField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi19OnRspQryParkedOrderEP26CThostFtdcParkedOrderFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryParkedOrder(
        this: *const ::std::os::raw::c_void,
        pParkedOrder: *const CThostFtdcParkedOrderField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi25OnRspQryParkedOrderActionEP32CThostFtdcParkedOrderActionFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryParkedOrderAction(
        this: *const ::std::os::raw::c_void,
        pParkedOrderAction: *const CThostFtdcParkedOrderActionField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi21OnRspQryTradingNoticeEP28CThostFtdcTradingNoticeFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryTradingNotice(
        this: *const ::std::os::raw::c_void,
        pTradingNotice: *const CThostFtdcTradingNoticeField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi27OnRspQryBrokerTradingParamsEP34CThostFtdcBrokerTradingParamsFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryBrokerTradingParams(
        this: *const ::std::os::raw::c_void,
        pBrokerTradingParams: *const CThostFtdcBrokerTradingParamsField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi26OnRspQryBrokerTradingAlgosEP33CThostFtdcBrokerTradingAlgosFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQryBrokerTradingAlgos(
        this: *const ::std::os::raw::c_void,
        pBrokerTradingAlgos: *const CThostFtdcBrokerTradingAlgosField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi34OnRspQueryCFMMCTradingAccountTokenEP44CThostFtdcQueryCFMMCTradingAccountTokenFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQueryCFMMCTradingAccountToken(
        this: *const ::std::os::raw::c_void,
        pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi27OnRtnFromBankToFutureByBankEP26CThostFtdcRspTransferField")]
    pub fn CTPTraderSpi_OnRtnFromBankToFutureByBank(
        this: *const ::std::os::raw::c_void,
        pRspTransfer: *const CThostFtdcRspTransferField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi27OnRtnFromFutureToBankByBankEP26CThostFtdcRspTransferField")]
    pub fn CTPTraderSpi_OnRtnFromFutureToBankByBank(
        this: *const ::std::os::raw::c_void,
        pRspTransfer: *const CThostFtdcRspTransferField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi33OnRtnRepealFromBankToFutureByBankEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromBankToFutureByBank(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi33OnRtnRepealFromFutureToBankByBankEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromFutureToBankByBank(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRtnFromBankToFutureByFutureEP26CThostFtdcRspTransferField")]
    pub fn CTPTraderSpi_OnRtnFromBankToFutureByFuture(
        this: *const ::std::os::raw::c_void,
        pRspTransfer: *const CThostFtdcRspTransferField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRtnFromFutureToBankByFutureEP26CThostFtdcRspTransferField")]
    pub fn CTPTraderSpi_OnRtnFromFutureToBankByFuture(
        this: *const ::std::os::raw::c_void,
        pRspTransfer: *const CThostFtdcRspTransferField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi41OnRtnRepealFromBankToFutureByFutureManualEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromBankToFutureByFutureManual(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi41OnRtnRepealFromFutureToBankByFutureManualEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromFutureToBankByFutureManual(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRtnQueryBankBalanceByFutureEP33CThostFtdcNotifyQueryAccountField")]
    pub fn CTPTraderSpi_OnRtnQueryBankBalanceByFuture(
        this: *const ::std::os::raw::c_void,
        pNotifyQueryAccount: *const CThostFtdcNotifyQueryAccountField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi28OnErrRtnBankToFutureByFutureEP26CThostFtdcReqTransferFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnBankToFutureByFuture(
        this: *const ::std::os::raw::c_void,
        pReqTransfer: *const CThostFtdcReqTransferField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi28OnErrRtnFutureToBankByFutureEP26CThostFtdcReqTransferFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnFutureToBankByFuture(
        this: *const ::std::os::raw::c_void,
        pReqTransfer: *const CThostFtdcReqTransferField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi40OnErrRtnRepealBankToFutureByFutureManualEP24CThostFtdcReqRepealFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnRepealBankToFutureByFutureManual(
        this: *const ::std::os::raw::c_void,
        pReqRepeal: *const CThostFtdcReqRepealField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi40OnErrRtnRepealFutureToBankByFutureManualEP24CThostFtdcReqRepealFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnRepealFutureToBankByFutureManual(
        this: *const ::std::os::raw::c_void,
        pReqRepeal: *const CThostFtdcReqRepealField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi32OnErrRtnQueryBankBalanceByFutureEP30CThostFtdcReqQueryAccountFieldP22CThostFtdcRspInfoField")]
    pub fn CTPTraderSpi_OnErrRtnQueryBankBalanceByFuture(
        this: *const ::std::os::raw::c_void,
        pReqQueryAccount: *const CThostFtdcReqQueryAccountField,
        pRspInfo: *const CThostFtdcRspInfoField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi35OnRtnRepealFromBankToFutureByFutureEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromBankToFutureByFuture(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi35OnRtnRepealFromFutureToBankByFutureEP24CThostFtdcRspRepealField")]
    pub fn CTPTraderSpi_OnRtnRepealFromFutureToBankByFuture(
        this: *const ::std::os::raw::c_void,
        pRspRepeal: *const CThostFtdcRspRepealField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRspFromBankToFutureByFutureEP26CThostFtdcReqTransferFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspFromBankToFutureByFuture(
        this: *const ::std::os::raw::c_void,
        pReqTransfer: *const CThostFtdcReqTransferField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi29OnRspFromFutureToBankByFutureEP26CThostFtdcReqTransferFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspFromFutureToBankByFuture(
        this: *const ::std::os::raw::c_void,
        pReqTransfer: *const CThostFtdcReqTransferField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi34OnRspQueryBankAccountMoneyByFutureEP30CThostFtdcReqQueryAccountFieldP22CThostFtdcRspInfoFieldib")]
    pub fn CTPTraderSpi_OnRspQueryBankAccountMoneyByFuture(
        this: *const ::std::os::raw::c_void,
        pReqQueryAccount: *const CThostFtdcReqQueryAccountField,
        pRspInfo: *const CThostFtdcRspInfoField,
        nRequestID: ::std::os::raw::c_int,
        bIsLast: bool,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi22OnRtnOpenAccountByBankEP26CThostFtdcOpenAccountField")]
    pub fn CTPTraderSpi_OnRtnOpenAccountByBank(
        this: *const ::std::os::raw::c_void,
        pOpenAccount: *const CThostFtdcOpenAccountField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnRtnCancelAccountByBankEP28CThostFtdcCancelAccountField")]
    pub fn CTPTraderSpi_OnRtnCancelAccountByBank(
        this: *const ::std::os::raw::c_void,
        pCancelAccount: *const CThostFtdcCancelAccountField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi24OnRtnChangeAccountByBankEP28CThostFtdcChangeAccountField")]
    pub fn CTPTraderSpi_OnRtnChangeAccountByBank(
        this: *const ::std::os::raw::c_void,
        pChangeAccount: *const CThostFtdcChangeAccountField,
    );
}
extern "C" {
    #[link_name = link_name!("ZN12CTPTraderSpi7ReleaseEv")]
    pub fn CTPTraderSpi_Release(this: *mut CTPTraderSpi);
}
extern "C" {
    pub fn rust_log_trace(msg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn rust_log_debug(msg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn rust_log_info(msg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn rust_log_warn(msg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn rust_log_error(msg: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn MdApi_Release(p: *const CThostFtdcMdApi);
}
extern "C" {
    pub fn MdApi_Init(p: *const CThostFtdcMdApi);
}
extern "C" {
    pub fn MdApi_Join(p: *const CThostFtdcMdApi) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_GetTradingDay(p: *const CThostFtdcMdApi) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn MdApi_RegisterFront(
        p: *const CThostFtdcMdApi,
        pszFrontAddress: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn MdApi_RegisterNameServer(
        p: *const CThostFtdcMdApi,
        pszNsAddress: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn MdApi_RegisterFensUserInfo(
        p: *const CThostFtdcMdApi,
        pi: *const CThostFtdcFensUserInfoField,
    );
}
extern "C" {
    pub fn MdApi_RegisterSpi(p: *const CThostFtdcMdApi, pSpi: *const CTPMarketSpi);
}
extern "C" {
    pub fn MdApi_SubscribeMarketData(
        p: *const CThostFtdcMdApi,
        ppInstrumentID: *const *mut ::std::os::raw::c_char,
        nCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_UnSubscribeMarketData(
        p: *const CThostFtdcMdApi,
        ppInstrumentID: *const *mut ::std::os::raw::c_char,
        nCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_SubscribeForQuoteRsp(
        p: *const CThostFtdcMdApi,
        ppInstrumentID: *const *mut ::std::os::raw::c_char,
        nCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_UnSubscribeForQuoteRsp(
        p: *const CThostFtdcMdApi,
        ppInstrumentID: *const *mut ::std::os::raw::c_char,
        nCount: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_ReqUserLogin(
        p: *const CThostFtdcMdApi,
        pReqUserLoginField: *const CThostFtdcReqUserLoginField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn MdApi_ReqUserLogout(
        p: *const CThostFtdcMdApi,
        pUserLogout: *const CThostFtdcUserLogoutField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
cfg_if::cfg_if! {if #[cfg(any(feature = "v6_3_19", feature = "v6_7_0"))] {
extern "C" {
    pub fn MdApi_ReqQryMulticastInstrument(
        p: *const CThostFtdcMdApi,
        pQryMulticastInstrument: *const CThostFtdcQryMulticastInstrumentField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
}}
extern "C" {
    pub fn TraderApi_Release(p: *const CThostFtdcTraderApi);
}
extern "C" {
    pub fn TraderApi_Init(p: *const CThostFtdcTraderApi);
}
extern "C" {
    pub fn TraderApi_Join(p: *const CThostFtdcTraderApi) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_GetTradingDay(p: *const CThostFtdcTraderApi) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn TraderApi_RegisterFront(
        p: *const CThostFtdcTraderApi,
        pszFrontAddress: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn TraderApi_RegisterNameServer(
        p: *const CThostFtdcTraderApi,
        pszNsAddress: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn TraderApi_RegisterFensUserInfo(
        p: *const CThostFtdcTraderApi,
        pFensUserInfo: *const CThostFtdcFensUserInfoField,
    );
}
extern "C" {
    pub fn TraderApi_RegisterSpi(p: *const CThostFtdcTraderApi, pSpi: *const CTPTraderSpi);
}
extern "C" {
    pub fn TraderApi_SubscribePrivateTopic(
        p: *const CThostFtdcTraderApi,
        nResumeType: THOST_TE_RESUME_TYPE,
    );
}
extern "C" {
    pub fn TraderApi_SubscribePublicTopic(
        p: *const CThostFtdcTraderApi,
        nResumeType: THOST_TE_RESUME_TYPE,
    );
}
extern "C" {
    pub fn TraderApi_ReqAuthenticate(
        p: *const CThostFtdcTraderApi,
        pReqAuthenticateField: *const CThostFtdcReqAuthenticateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqUserLogin(
        p: *const CThostFtdcTraderApi,
        pReqUserLoginField: *const CThostFtdcReqUserLoginField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqUserLogout(
        p: *const CThostFtdcTraderApi,
        pUserLogout: *const CThostFtdcUserLogoutField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqUserPasswordUpdate(
        p: *const CThostFtdcTraderApi,
        pUserPasswordUpdate: *const CThostFtdcUserPasswordUpdateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqTradingAccountPasswordUpdate(
        p: *const CThostFtdcTraderApi,
        pTradingAccountPasswordUpdate: *const CThostFtdcTradingAccountPasswordUpdateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqOrderInsert(
        p: *const CThostFtdcTraderApi,
        pInputOrder: *const CThostFtdcInputOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqParkedOrderInsert(
        p: *const CThostFtdcTraderApi,
        pParkedOrder: *const CThostFtdcParkedOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqParkedOrderAction(
        p: *const CThostFtdcTraderApi,
        pParkedOrderAction: *const CThostFtdcParkedOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqOrderAction(
        p: *const CThostFtdcTraderApi,
        pInputOrderAction: *const CThostFtdcInputOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQueryMaxOrderVolume(
        p: *const CThostFtdcTraderApi,
        pQueryMaxOrderVolume: *const CThostFtdcQueryMaxOrderVolumeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqSettlementInfoConfirm(
        p: *const CThostFtdcTraderApi,
        pSettlementInfoConfirm: *const CThostFtdcSettlementInfoConfirmField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqRemoveParkedOrder(
        p: *const CThostFtdcTraderApi,
        pRemoveParkedOrder: *const CThostFtdcRemoveParkedOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqRemoveParkedOrderAction(
        p: *const CThostFtdcTraderApi,
        pRemoveParkedOrderAction: *const CThostFtdcRemoveParkedOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqExecOrderInsert(
        p: *const CThostFtdcTraderApi,
        pInputExecOrder: *const CThostFtdcInputExecOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqExecOrderAction(
        p: *const CThostFtdcTraderApi,
        pInputExecOrderAction: *const CThostFtdcInputExecOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqForQuoteInsert(
        p: *const CThostFtdcTraderApi,
        pInputForQuote: *const CThostFtdcInputForQuoteField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQuoteInsert(
        p: *const CThostFtdcTraderApi,
        pInputQuote: *const CThostFtdcInputQuoteField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQuoteAction(
        p: *const CThostFtdcTraderApi,
        pInputQuoteAction: *const CThostFtdcInputQuoteActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqBatchOrderAction(
        p: *const CThostFtdcTraderApi,
        pInputBatchOrderAction: *const CThostFtdcInputBatchOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqCombActionInsert(
        p: *const CThostFtdcTraderApi,
        pInputCombAction: *const CThostFtdcInputCombActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryOrder(
        p: *const CThostFtdcTraderApi,
        pQryOrder: *const CThostFtdcQryOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTrade(
        p: *const CThostFtdcTraderApi,
        pQryTrade: *const CThostFtdcQryTradeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestorPosition(
        p: *const CThostFtdcTraderApi,
        pQryInvestorPosition: *const CThostFtdcQryInvestorPositionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTradingAccount(
        p: *const CThostFtdcTraderApi,
        pQryTradingAccount: *const CThostFtdcQryTradingAccountField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestor(
        p: *const CThostFtdcTraderApi,
        pQryInvestor: *const CThostFtdcQryInvestorField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTradingCode(
        p: *const CThostFtdcTraderApi,
        pQryTradingCode: *const CThostFtdcQryTradingCodeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInstrumentMarginRate(
        p: *const CThostFtdcTraderApi,
        pQryInstrumentMarginRate: *const CThostFtdcQryInstrumentMarginRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInstrumentCommissionRate(
        p: *const CThostFtdcTraderApi,
        pQryInstrumentCommissionRate: *const CThostFtdcQryInstrumentCommissionRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryExchange(
        p: *const CThostFtdcTraderApi,
        pQryExchange: *const CThostFtdcQryExchangeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryProduct(
        p: *const CThostFtdcTraderApi,
        pQryProduct: *const CThostFtdcQryProductField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInstrument(
        p: *const CThostFtdcTraderApi,
        pQryInstrument: *const CThostFtdcQryInstrumentField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryDepthMarketData(
        p: *const CThostFtdcTraderApi,
        pQryDepthMarketData: *const CThostFtdcQryDepthMarketDataField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySettlementInfo(
        p: *const CThostFtdcTraderApi,
        pQrySettlementInfo: *const CThostFtdcQrySettlementInfoField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTransferBank(
        p: *const CThostFtdcTraderApi,
        pQryTransferBank: *const CThostFtdcQryTransferBankField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestorPositionDetail(
        p: *const CThostFtdcTraderApi,
        pQryInvestorPositionDetail: *const CThostFtdcQryInvestorPositionDetailField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryNotice(
        p: *const CThostFtdcTraderApi,
        pQryNotice: *const CThostFtdcQryNoticeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySettlementInfoConfirm(
        p: *const CThostFtdcTraderApi,
        pQrySettlementInfoConfirm: *const CThostFtdcQrySettlementInfoConfirmField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestorPositionCombineDetail(
        p: *const CThostFtdcTraderApi,
        pQryInvestorPositionCombineDetail: *const CThostFtdcQryInvestorPositionCombineDetailField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryCFMMCTradingAccountKey(
        p: *const CThostFtdcTraderApi,
        pQryCFMMCTradingAccountKey: *const CThostFtdcQryCFMMCTradingAccountKeyField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryEWarrantOffset(
        p: *const CThostFtdcTraderApi,
        pQryEWarrantOffset: *const CThostFtdcQryEWarrantOffsetField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestorProductGroupMargin(
        p: *const CThostFtdcTraderApi,
        pQryInvestorProductGroupMargin: *const CThostFtdcQryInvestorProductGroupMarginField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryExchangeMarginRate(
        p: *const CThostFtdcTraderApi,
        pQryExchangeMarginRate: *const CThostFtdcQryExchangeMarginRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryExchangeMarginRateAdjust(
        p: *const CThostFtdcTraderApi,
        pQryExchangeMarginRateAdjust: *const CThostFtdcQryExchangeMarginRateAdjustField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryExchangeRate(
        p: *const CThostFtdcTraderApi,
        pQryExchangeRate: *const CThostFtdcQryExchangeRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySecAgentACIDMap(
        p: *const CThostFtdcTraderApi,
        pQrySecAgentACIDMap: *const CThostFtdcQrySecAgentACIDMapField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryProductExchRate(
        p: *const CThostFtdcTraderApi,
        pQryProductExchRate: *const CThostFtdcQryProductExchRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryProductGroup(
        p: *const CThostFtdcTraderApi,
        pQryProductGroup: *const CThostFtdcQryProductGroupField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryMMInstrumentCommissionRate(
        p: *const CThostFtdcTraderApi,
        pQryMMInstrumentCommissionRate: *const CThostFtdcQryMMInstrumentCommissionRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryMMOptionInstrCommRate(
        p: *const CThostFtdcTraderApi,
        pQryMMOptionInstrCommRate: *const CThostFtdcQryMMOptionInstrCommRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInstrumentOrderCommRate(
        p: *const CThostFtdcTraderApi,
        pQryInstrumentOrderCommRate: *const CThostFtdcQryInstrumentOrderCommRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySecAgentTradingAccount(
        p: *const CThostFtdcTraderApi,
        pQryTradingAccount: *const CThostFtdcQryTradingAccountField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySecAgentCheckMode(
        p: *const CThostFtdcTraderApi,
        pQrySecAgentCheckMode: *const CThostFtdcQrySecAgentCheckModeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQrySecAgentTradeInfo(
        p: *const CThostFtdcTraderApi,
        pQrySecAgentTradeInfo: *const CThostFtdcQrySecAgentTradeInfoField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryOptionInstrTradeCost(
        p: *const CThostFtdcTraderApi,
        pQryOptionInstrTradeCost: *const CThostFtdcQryOptionInstrTradeCostField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryOptionInstrCommRate(
        p: *const CThostFtdcTraderApi,
        pQryOptionInstrCommRate: *const CThostFtdcQryOptionInstrCommRateField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryExecOrder(
        p: *const CThostFtdcTraderApi,
        pQryExecOrder: *const CThostFtdcQryExecOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryForQuote(
        p: *const CThostFtdcTraderApi,
        pQryForQuote: *const CThostFtdcQryForQuoteField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryQuote(
        p: *const CThostFtdcTraderApi,
        pQryQuote: *const CThostFtdcQryQuoteField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryOptionSelfClose(
        p: *const CThostFtdcTraderApi,
        pQryOptionSelfClose: *const CThostFtdcQryOptionSelfCloseField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryInvestUnit(
        p: *const CThostFtdcTraderApi,
        pQryInvestUnit: *const CThostFtdcQryInvestUnitField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryCombInstrumentGuard(
        p: *const CThostFtdcTraderApi,
        pQryCombInstrumentGuard: *const CThostFtdcQryCombInstrumentGuardField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryCombAction(
        p: *const CThostFtdcTraderApi,
        pQryCombAction: *const CThostFtdcQryCombActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTransferSerial(
        p: *const CThostFtdcTraderApi,
        pQryTransferSerial: *const CThostFtdcQryTransferSerialField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryAccountregister(
        p: *const CThostFtdcTraderApi,
        pQryAccountregister: *const CThostFtdcQryAccountregisterField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryContractBank(
        p: *const CThostFtdcTraderApi,
        pQryContractBank: *const CThostFtdcQryContractBankField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryParkedOrder(
        p: *const CThostFtdcTraderApi,
        pQryParkedOrder: *const CThostFtdcQryParkedOrderField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryParkedOrderAction(
        p: *const CThostFtdcTraderApi,
        pQryParkedOrderAction: *const CThostFtdcQryParkedOrderActionField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryTradingNotice(
        p: *const CThostFtdcTraderApi,
        pQryTradingNotice: *const CThostFtdcQryTradingNoticeField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryBrokerTradingParams(
        p: *const CThostFtdcTraderApi,
        pQryBrokerTradingParams: *const CThostFtdcQryBrokerTradingParamsField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQryBrokerTradingAlgos(
        p: *const CThostFtdcTraderApi,
        pQryBrokerTradingAlgos: *const CThostFtdcQryBrokerTradingAlgosField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQueryCFMMCTradingAccountToken(
        p: *const CThostFtdcTraderApi,
        pQueryCFMMCTradingAccountToken: *const CThostFtdcQueryCFMMCTradingAccountTokenField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqFromBankToFutureByFuture(
        p: *const CThostFtdcTraderApi,
        pReqTransfer: *const CThostFtdcReqTransferField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqFromFutureToBankByFuture(
        p: *const CThostFtdcTraderApi,
        pReqTransfer: *const CThostFtdcReqTransferField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn TraderApi_ReqQueryBankAccountMoneyByFuture(
        p: *const CThostFtdcTraderApi,
        pReqQueryAccount: *const CThostFtdcReqQueryAccountField,
        nRequestID: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn CTPMarketSpi_New() -> *mut CTPMarketSpi;
}
extern "C" {
    pub fn CTPTraderSpi_New() -> *mut CTPTraderSpi;
}
