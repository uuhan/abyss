use super::{TraderApi, TraderSpi};
use crate::ctp::api::*;
use crate::ctp::error::*;
use crate::ctp::opts::*;
use std::ffi::CStr;
use std::ffi::CString;

impl TraderApi {
    #[inline]
    pub unsafe fn new(lp: impl AsRef<str>) -> Self {
        let path = CString::new(lp.as_ref())
            .expect("wrong log dir path.")
            .into_raw();
        let inner = CThostFtdcTraderApi::CreateFtdcTraderApi(path);
        Self(&mut *inner)
    }

    /// 删除接口对象本身
    /// @remark 不再使用本接口对象时,调用该函数删除接口对象
    #[inline]
    fn release(self) {
        unsafe { TraderApi_Release(self.0) }
    }

    /// 初始化
    /// @remark 初始化运行环境,只有调用后,接口才开始工作
    #[inline]
    pub fn init(&mut self) {
        unsafe { TraderApi_Init(self.0) }
    }

    /// 等待接口线程结束运行
    /// @return 线程退出代码
    #[inline]
    pub fn join(&self) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_Join(self.0) })
    }

    /// 获取当前交易日
    /// @retrun 获取到的交易日
    /// @remark 只有登录成功后,才能得到正确的交易日
    #[inline]
    pub fn get_trading_day(&self) -> &str {
        unsafe {
            let p = TraderApi_GetTradingDay(self.0);
            CStr::from_ptr(p as _).to_str().unwrap()
        }
    }

    /// 注册前置机网络地址
    /// @param pszFrontAddress：前置机网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
    #[inline]
    pub fn register_front(&self, front: impl Into<String>) {
        unsafe {
            let front: String = front.into();
            let front = CString::new(front).unwrap();
            TraderApi_RegisterFront(self.0, front.as_ptr() as _)
        }
    }

    /// 注册名字服务器网络地址
    /// @param pszNsAddress：名字服务器网络地址。
    /// @remark
    /// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
    /// @remark
    /// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
    /// @remark RegisterNameServer优先于RegisterFront
    #[inline]
    pub fn register_name_server(&self, ns: impl Into<String>) {
        unsafe {
            let ns: String = ns.into();
            let ns = CString::new(ns).unwrap();
            TraderApi_RegisterNameServer(self.0, ns.as_ptr() as _)
        }
    }

    /// 注册名字服务器用户信息
    /// @param pFensUserInfo：用户信息。
    #[inline]
    pub fn register_fens_user_info(&self, info: &CThostFtdcFensUserInfoField) {
        unsafe { TraderApi_RegisterFensUserInfo(self.0, info) }
    }

    /// 注册回调接口
    /// @param pSpi 派生自回调接口类的实例
    #[inline]
    pub fn register_spi(&self, spi: &TraderSpi) {
        unsafe {
            TraderApi_RegisterSpi(self.0, spi.0);
        }
    }

    /// 订阅私有流。
    /// @param nResumeType 私有流重传方式
    ///         THOST_TERT_RESTART:从本交易日开始重传
    ///         THOST_TERT_RESUME:从上次收到的续传
    ///         THOST_TERT_QUICK:只传送登录后私有流的内容
    /// @remark 该方法要在Init方法前调用。若不调用则不会收到私有流的数据。
    #[inline]
    pub fn subscribe_private_topic(&self, nResumeType: TopicTertFlag) {
        unsafe { TraderApi_SubscribePrivateTopic(self.0, nResumeType as u32) }
    }

    /// 订阅公共流。
    /// @param nResumeType 公共流重传方式
    ///         THOST_TERT_RESTART:从本交易日开始重传
    ///         THOST_TERT_RESUME:从上次收到的续传
    ///         THOST_TERT_QUICK:只传送登录后公共流的内容
    /// @remark 该方法要在Init方法前调用。若不调用则不会收到公共流的数据。
    #[inline]
    pub fn subscribe_public_topic(&self, nResumeType: TopicTertFlag) {
        unsafe { TraderApi_SubscribePublicTopic(self.0, nResumeType as u32) }
    }

    /// 客户端认证请求
    #[inline]
    pub fn req_authenticate(
        &self,
        pReqAuthenticateField: &CThostFtdcReqAuthenticateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqAuthenticate(self.0, pReqAuthenticateField, nRequestID)
        })
    }

    /// 用户登录请求
    #[inline]
    pub fn req_user_login(
        &self,
        pReqUserLoginField: &CThostFtdcReqUserLoginField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqUserLogin(self.0, pReqUserLoginField, nRequestID)
        })
    }

    /// 用户登出请求
    #[inline]
    pub fn req_user_logout(
        &self,
        pReqUserLogoutField: &CThostFtdcUserLogoutField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqUserLogout(self.0, pReqUserLogoutField, nRequestID)
        })
    }

    /// 用户口令更新请求
    #[inline]
    pub fn req_user_password_update(
        &self,
        pUserPasswordUpdate: &CThostFtdcUserPasswordUpdateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqUserPasswordUpdate(self.0, pUserPasswordUpdate, nRequestID)
        })
    }

    /// 资金账户口令更新请求
    #[inline]
    pub fn req_trading_account_password_update(
        &self,
        pTradingAccountPasswordUpdate: &CThostFtdcTradingAccountPasswordUpdateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqTradingAccountPasswordUpdate(
                self.0,
                pTradingAccountPasswordUpdate,
                nRequestID,
            )
        })
    }

    /// 报单录入请求
    #[inline]
    pub fn req_order_insert(
        &self,
        pInputOrder: &CThostFtdcInputOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqOrderInsert(self.0, pInputOrder, nRequestID) })
    }

    /// 预埋单录入请求
    #[inline]
    pub fn req_parked_order_insert(
        &self,
        pParkedOrder: &CThostFtdcParkedOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqParkedOrderInsert(self.0, pParkedOrder, nRequestID)
        })
    }

    /// 预埋撤单录入请求
    #[inline]
    pub fn req_parked_order_action(
        &self,
        pParkedOrderAction: &CThostFtdcParkedOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqParkedOrderAction(self.0, pParkedOrderAction, nRequestID)
        })
    }

    /// 报单操作请求
    #[inline]
    pub fn req_order_action(
        &self,
        pInputOrderAction: &CThostFtdcInputOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqOrderAction(self.0, pInputOrderAction, nRequestID)
        })
    }

    /// 查询最大报单数量请求
    #[inline]
    pub fn req_query_max_order_volume(
        &self,
        pQueryMaxOrderVolume: &CThostFtdcQueryMaxOrderVolumeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQueryMaxOrderVolume(self.0, pQueryMaxOrderVolume, nRequestID)
        })
    }

    /// 投资者结算结果确认
    #[inline]
    pub fn req_settlement_info_confirm(
        &self,
        pSettlementInfoConfirm: &CThostFtdcSettlementInfoConfirmField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqSettlementInfoConfirm(self.0, pSettlementInfoConfirm, nRequestID)
        })
    }

    /// 请求删除预埋单
    #[inline]
    pub fn req_remove_parked_order(
        &self,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqRemoveParkedOrder(self.0, pRemoveParkedOrder, nRequestID)
        })
    }

    /// 请求删除预埋撤单
    #[inline]
    pub fn req_remove_parked_order_action(
        &self,
        pRemoveParkedOrderAction: &CThostFtdcRemoveParkedOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqRemoveParkedOrderAction(self.0, pRemoveParkedOrderAction, nRequestID)
        })
    }

    /// 执行宣告录入请求
    #[inline]
    pub fn req_exec_order_insert(
        &self,
        pInputExecOrder: &CThostFtdcInputExecOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqExecOrderInsert(self.0, pInputExecOrder, nRequestID)
        })
    }

    /// 执行宣告操作请求
    #[inline]
    pub fn req_exec_order_action(
        &self,
        pInputExecOrderAction: &CThostFtdcInputExecOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqExecOrderAction(self.0, pInputExecOrderAction, nRequestID)
        })
    }

    /// 询价录入请求
    #[inline]
    pub fn req_for_quote_insert(
        &self,
        pInputForQuote: &CThostFtdcInputForQuoteField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqForQuoteInsert(self.0, pInputForQuote, nRequestID)
        })
    }

    /// 报价录入请求
    #[inline]
    pub fn req_quote_insert(
        &self,
        pInputQuote: &CThostFtdcInputQuoteField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQuoteInsert(self.0, pInputQuote, nRequestID) })
    }

    /// 报价操作请求
    #[inline]
    pub fn req_quote_action(
        &self,
        pInputQuoteAction: &CThostFtdcInputQuoteActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQuoteAction(self.0, pInputQuoteAction, nRequestID)
        })
    }

    /// 批量报单操作请求
    #[inline]
    pub fn req_batch_order_action(
        &self,
        pInputBatchOrderAction: &CThostFtdcInputBatchOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqBatchOrderAction(self.0, pInputBatchOrderAction, nRequestID)
        })
    }

    /// 申请组合录入请求
    #[inline]
    pub fn req_comb_action_insert(
        &self,
        pInputCombAction: &CThostFtdcInputCombActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqCombActionInsert(self.0, pInputCombAction, nRequestID)
        })
    }

    /// 请求查询报单
    #[inline]
    pub fn req_qry_order(
        &self,
        pQryOrder: &CThostFtdcQryOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryOrder(self.0, pQryOrder, nRequestID) })
    }

    /// 请求查询成交
    #[inline]
    pub fn req_qry_trade(
        &self,
        pQryTrade: &CThostFtdcQryTradeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryTrade(self.0, pQryTrade, nRequestID) })
    }

    /// 请求查询投资者持仓
    #[inline]
    pub fn req_qry_investor_position(
        &self,
        pQryInvestorPosition: &CThostFtdcQryInvestorPositionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInvestorPosition(self.0, pQryInvestorPosition, nRequestID)
        })
    }

    /// 请求查询资金账户
    #[inline]
    pub fn req_qry_trading_account(
        &self,
        pQryTradingAccount: &CThostFtdcQryTradingAccountField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryTradingAccount(self.0, pQryTradingAccount, nRequestID)
        })
    }

    /// 请求查询投资者账户
    #[inline]
    pub fn req_qry_investor(
        &self,
        pQryInvestor: &CThostFtdcQryInvestorField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryInvestor(self.0, pQryInvestor, nRequestID) })
    }

    /// 请求查询交易编码
    #[inline]
    pub fn req_qry_trading_code(
        &self,
        pQryTradingCode: &CThostFtdcQryTradingCodeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryTradingCode(self.0, pQryTradingCode, nRequestID)
        })
    }

    /// 请求查询合约保证金率
    #[inline]
    pub fn req_qry_instrument_margin_rate(
        &self,
        pQryInstrumentMarginRate: &CThostFtdcQryInstrumentMarginRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInstrumentMarginRate(self.0, pQryInstrumentMarginRate, nRequestID)
        })
    }

    /// 请求查询合约手续费率
    #[inline]
    pub fn req_qry_instrument_commission_rate(
        &self,
        pQryInstrumentCommissionRate: &CThostFtdcQryInstrumentCommissionRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInstrumentCommissionRate(
                self.0,
                pQryInstrumentCommissionRate,
                nRequestID,
            )
        })
    }

    /// 请求查询交易所
    #[inline]
    pub fn req_qry_exchange(
        &self,
        pQryExchange: &CThostFtdcQryExchangeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryExchange(self.0, pQryExchange, nRequestID) })
    }

    /// 请求查询产品
    #[inline]
    pub fn req_qry_product(
        &self,
        pQryProduct: &CThostFtdcQryProductField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryProduct(self.0, pQryProduct, nRequestID) })
    }

    /// 请求查询合约
    #[inline]
    pub fn req_qry_instrument(
        &self,
        pQryInstrument: &CThostFtdcQryInstrumentField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInstrument(self.0, pQryInstrument, nRequestID)
        })
    }

    /// 请求查询行情
    #[inline]
    pub fn req_qry_depth_market_data(
        &self,
        pQryDepthMarketData: &CThostFtdcQryDepthMarketDataField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryDepthMarketData(self.0, pQryDepthMarketData, nRequestID)
        })
    }

    /// 请求查询投资者结算结果
    #[inline]
    pub fn req_qry_settlement_info(
        &self,
        pQrySettlementInfo: &CThostFtdcQrySettlementInfoField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySettlementInfo(self.0, pQrySettlementInfo, nRequestID)
        })
    }

    /// 请求查询转帐银行
    #[inline]
    pub fn req_qry_transfer_bank(
        &self,
        pQryTransferBank: &CThostFtdcQryTransferBankField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryTransferBank(self.0, pQryTransferBank, nRequestID)
        })
    }

    /// 请求查询投资者持仓明细
    #[inline]
    pub fn req_qry_investor_position_detail(
        &self,
        pQryInvestorPositionDetail: &CThostFtdcQryInvestorPositionDetailField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInvestorPositionDetail(self.0, pQryInvestorPositionDetail, nRequestID)
        })
    }

    /// 请求查询客户通知
    #[inline]
    pub fn req_qry_notice(
        &self,
        pQryNotice: &CThostFtdcQryNoticeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryNotice(self.0, pQryNotice, nRequestID) })
    }

    /// 请求查询结算信息确认
    #[inline]
    pub fn req_qry_settlement_info_confirm(
        &self,
        pQrySettlementInfoConfirm: &CThostFtdcQrySettlementInfoConfirmField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySettlementInfoConfirm(self.0, pQrySettlementInfoConfirm, nRequestID)
        })
    }

    /// 请求查询投资者持仓明细
    #[inline]
    pub fn req_qry_investor_position_combine_detail(
        &self,
        pQryInvestorPositionCombineDetail: &CThostFtdcQryInvestorPositionCombineDetailField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInvestorPositionCombineDetail(
                self.0,
                pQryInvestorPositionCombineDetail,
                nRequestID,
            )
        })
    }

    /// 请求查询保证金监管系统经纪公司资金账户密钥
    #[inline]
    pub fn req_qry_cfmmc_trading_account_key(
        &self,
        pQryCFMMCTradingAccountKey: &CThostFtdcQryCFMMCTradingAccountKeyField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryCFMMCTradingAccountKey(self.0, pQryCFMMCTradingAccountKey, nRequestID)
        })
    }

    /// 请求查询仓单折抵信息
    #[inline]
    pub fn req_qry_e_warrant_offset(
        &self,
        pQryEWarrantOffset: &CThostFtdcQryEWarrantOffsetField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryEWarrantOffset(self.0, pQryEWarrantOffset, nRequestID)
        })
    }

    /// 请求查询投资者品种/跨品种保证金
    #[inline]
    pub fn req_qry_investor_product_group_margin(
        &self,
        pQryInvestorProductGroupMargin: &CThostFtdcQryInvestorProductGroupMarginField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInvestorProductGroupMargin(
                self.0,
                pQryInvestorProductGroupMargin,
                nRequestID,
            )
        })
    }

    /// 请求查询交易所保证金率
    #[inline]
    pub fn req_qry_exchange_margin_rate(
        &self,
        pQryExchangeMarginRate: &CThostFtdcQryExchangeMarginRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryExchangeMarginRate(self.0, pQryExchangeMarginRate, nRequestID)
        })
    }

    /// 请求查询交易所调整保证金率
    #[inline]
    pub fn req_qry_exchange_margin_rate_adjust(
        &self,
        pQryExchangeMarginRateAdjust: &CThostFtdcQryExchangeMarginRateAdjustField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryExchangeMarginRateAdjust(
                self.0,
                pQryExchangeMarginRateAdjust,
                nRequestID,
            )
        })
    }

    /// 请求查询汇率
    #[inline]
    pub fn req_qry_exchange_rate(
        &self,
        pQryExchangeRate: &CThostFtdcQryExchangeRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryExchangeRate(self.0, pQryExchangeRate, nRequestID)
        })
    }

    /// 请求查询二级代理操作员银期权限
    #[inline]
    pub fn req_qry_sec_agent_acid_map(
        &self,
        pQrySecAgentACIDMap: &CThostFtdcQrySecAgentACIDMapField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySecAgentACIDMap(self.0, pQrySecAgentACIDMap, nRequestID)
        })
    }

    /// 请求查询产品报价汇率
    #[inline]
    pub fn req_qry_product_exch_rate(
        &self,
        pQryProductExchRate: &CThostFtdcQryProductExchRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryProductExchRate(self.0, pQryProductExchRate, nRequestID)
        })
    }

    /// 请求查询产品组
    #[inline]
    pub fn req_qry_product_group(
        &self,
        pQryProductGroup: &CThostFtdcQryProductGroupField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryProductGroup(self.0, pQryProductGroup, nRequestID)
        })
    }

    /// 请求查询做市商合约手续费率
    #[inline]
    pub fn req_qry_mm_instrument_commission_rate(
        &self,
        pQryMMInstrumentCommissionRate: &CThostFtdcQryMMInstrumentCommissionRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryMMInstrumentCommissionRate(
                self.0,
                pQryMMInstrumentCommissionRate,
                nRequestID,
            )
        })
    }

    /// 请求查询做市商期权合约手续费
    #[inline]
    pub fn req_qry_mm_option_instr_comm_rate(
        &self,
        pQryMMOptionInstrCommRate: &CThostFtdcQryMMOptionInstrCommRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryMMOptionInstrCommRate(self.0, pQryMMOptionInstrCommRate, nRequestID)
        })
    }

    /// 请求查询报单手续费
    #[inline]
    pub fn req_qry_instrument_order_comm_rate(
        &self,
        pQryInstrumentOrderCommRate: &CThostFtdcQryInstrumentOrderCommRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInstrumentOrderCommRate(self.0, pQryInstrumentOrderCommRate, nRequestID)
        })
    }

    /// 请求查询资金账户
    #[inline]
    pub fn req_qry_sec_agent_trading_account(
        &self,
        pQryTradingAccount: &CThostFtdcQryTradingAccountField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySecAgentTradingAccount(self.0, pQryTradingAccount, nRequestID)
        })
    }

    /// 请求查询二级代理商资金校验模式
    #[inline]
    pub fn req_qry_sec_agent_check_mode(
        &self,
        pQrySecAgentCheckMode: &CThostFtdcQrySecAgentCheckModeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySecAgentCheckMode(self.0, pQrySecAgentCheckMode, nRequestID)
        })
    }

    /// 请求查询二级代理商信息
    #[inline]
    pub fn req_qry_sec_agent_trade_info(
        &self,
        pQrySecAgentTradeInfo: &CThostFtdcQrySecAgentTradeInfoField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQrySecAgentTradeInfo(self.0, pQrySecAgentTradeInfo, nRequestID)
        })
    }

    /// 请求查询期权交易成本
    #[inline]
    pub fn req_qry_option_instr_trade_cost(
        &self,
        pQryOptionInstrTradeCost: &CThostFtdcQryOptionInstrTradeCostField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryOptionInstrTradeCost(self.0, pQryOptionInstrTradeCost, nRequestID)
        })
    }

    /// 请求查询期权合约手续费
    #[inline]
    pub fn req_qry_option_instr_comm_rate(
        &self,
        pQryOptionInstrCommRate: &CThostFtdcQryOptionInstrCommRateField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryOptionInstrCommRate(self.0, pQryOptionInstrCommRate, nRequestID)
        })
    }

    /// 请求查询执行宣告
    #[inline]
    pub fn req_qry_exec_order(
        &self,
        pQryExecOrder: &CThostFtdcQryExecOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryExecOrder(self.0, pQryExecOrder, nRequestID) })
    }

    /// 请求查询询价
    #[inline]
    pub fn req_qry_for_quote(
        &self,
        pQryForQuote: &CThostFtdcQryForQuoteField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryForQuote(self.0, pQryForQuote, nRequestID) })
    }

    /// 请求查询报价
    #[inline]
    pub fn req_qry_quote(
        &self,
        pQryQuote: &CThostFtdcQryQuoteField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { TraderApi_ReqQryQuote(self.0, pQryQuote, nRequestID) })
    }

    /// 请求查询期权自对冲
    #[inline]
    pub fn req_qry_option_self_close(
        &self,
        pQryOptionSelfClose: &CThostFtdcQryOptionSelfCloseField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryOptionSelfClose(self.0, pQryOptionSelfClose, nRequestID)
        })
    }

    /// 请求查询投资单元
    #[inline]
    pub fn req_qry_invest_unit(
        &self,
        pQryInvestUnit: &CThostFtdcQryInvestUnitField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryInvestUnit(self.0, pQryInvestUnit, nRequestID)
        })
    }

    /// 请求查询组合合约安全系数
    #[inline]
    pub fn req_qry_comb_instrument_guard(
        &self,
        pQryCombInstrumentGuard: &CThostFtdcQryCombInstrumentGuardField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryCombInstrumentGuard(self.0, pQryCombInstrumentGuard, nRequestID)
        })
    }

    /// 请求查询申请组合
    #[inline]
    pub fn req_qry_comb_action(
        &self,
        pQryCombAction: &CThostFtdcQryCombActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryCombAction(self.0, pQryCombAction, nRequestID)
        })
    }

    /// 请求查询转帐流水
    #[inline]
    pub fn req_qry_transfer_serial(
        &self,
        pQryTransferSerial: &CThostFtdcQryTransferSerialField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryTransferSerial(self.0, pQryTransferSerial, nRequestID)
        })
    }

    /// 请求查询银期签约关系
    #[inline]
    pub fn req_qry_account_register(
        &self,
        pQryAccountregister: &CThostFtdcQryAccountregisterField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryAccountregister(self.0, pQryAccountregister, nRequestID)
        })
    }

    /// 请求查询签约银行
    #[inline]
    pub fn req_qry_contract_bank(
        &self,
        pQryContractBank: &CThostFtdcQryContractBankField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryContractBank(self.0, pQryContractBank, nRequestID)
        })
    }

    /// 请求查询预埋单
    #[inline]
    pub fn req_qry_parked_order(
        &self,
        pQryParkedOrder: &CThostFtdcQryParkedOrderField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryParkedOrder(self.0, pQryParkedOrder, nRequestID)
        })
    }

    /// 请求查询预埋撤单
    #[inline]
    pub fn req_qry_parked_order_action(
        &self,
        pQryParkedOrderAction: &CThostFtdcQryParkedOrderActionField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryParkedOrderAction(self.0, pQryParkedOrderAction, nRequestID)
        })
    }

    /// 请求查询交易通知
    #[inline]
    pub fn req_qry_trading_notice(
        &self,
        pQryTradingNotice: &CThostFtdcQryTradingNoticeField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryTradingNotice(self.0, pQryTradingNotice, nRequestID)
        })
    }

    /// 请求查询经纪公司交易参数
    #[inline]
    pub fn req_qry_broker_trading_params(
        &self,
        pQryBrokerTradingParams: &CThostFtdcQryBrokerTradingParamsField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryBrokerTradingParams(self.0, pQryBrokerTradingParams, nRequestID)
        })
    }

    /// 请求查询经纪公司交易算法
    #[inline]
    pub fn req_qry_broker_trading_algos(
        &self,
        pQryBrokerTradingAlgos: &CThostFtdcQryBrokerTradingAlgosField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQryBrokerTradingAlgos(self.0, pQryBrokerTradingAlgos, nRequestID)
        })
    }

    /// 请求查询监控中心用户令牌
    #[inline]
    pub fn req_query_cfmmc_trading_account_token(
        &self,
        pQueryCFMMCTradingAccountToken: &CThostFtdcQueryCFMMCTradingAccountTokenField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQueryCFMMCTradingAccountToken(
                self.0,
                pQueryCFMMCTradingAccountToken,
                nRequestID,
            )
        })
    }

    /// 期货发起银行资金转期货请求
    #[inline]
    pub fn req_from_bank_to_future_by_future(
        &self,
        pReqTransfer: &CThostFtdcReqTransferField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqFromBankToFutureByFuture(self.0, pReqTransfer, nRequestID)
        })
    }

    /// 期货发起期货资金转银行请求
    #[inline]
    pub fn req_from_future_to_bank_by_future(
        &self,
        pReqTransfer: &CThostFtdcReqTransferField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqFromFutureToBankByFuture(self.0, pReqTransfer, nRequestID)
        })
    }

    /// 期货发起查询银行余额请求
    #[inline]
    pub fn req_query_bank_account_money_by_future(
        &self,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            TraderApi_ReqQueryBankAccountMoneyByFuture(self.0, pReqQueryAccount, nRequestID)
        })
    }
}
