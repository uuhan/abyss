use super::super::api::*;
/// 注意, 下面的 let trader->on_xxx_cb: Box<Vec<Box<dyn FnMut()>>> = Box::new(vec![Box::new(cb)])
/// 类型注释不能省略, 不然会段错误!
use std::os::raw::c_void;

use super::TraderSpi;

impl TraderSpi {
    pub unsafe fn new() -> Self {
        Self(&mut *CTPTraderSpi_New())
    }

    pub fn release(&mut self) {
        unsafe { CTPTraderSpi_Release(self.0) }
    }
}

impl TraderSpi {
    ctp_fn_wrapper!(
        "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
        trader->on_front_connected,
    );

    ctp_fn_wrapper!(
        "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。"
        "@param nReason 错误原因"
        "        0x1001 网络读失败"
        "        0x1002 网络写失败"
        "        0x2001 接收心跳超时"
        "        0x2002 发送心跳失败"
        "        0x2003 收到错误报文"
        trader->on_front_disconnected,
        reason: i32,
    );

    ctp_fn_wrapper!(
        "心跳超时警告。当长时间未收到报文时，该方法被调用。"
        "@param nTimeLapse 距离上次接收报文的时间"
        "注意: 这个方法永远不会被调用到"
        trader->on_heartbeat_warning,
        i32,
    );

    ctp_fn_wrapper!(
        "错误应答"
        trader->on_rsp_error,
        &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "客户端认证响应"
        trader->on_rsp_authenticate,
        &CThostFtdcRspAuthenticateField,
        &CThostFtdcRspInfoField,
        i32, bool
    );

    ctp_fn_wrapper!(
        "用户口令更新请求响应"
        trader->on_rsp_user_password_update,
        &CThostFtdcUserPasswordUpdateField,
        &CThostFtdcRspInfoField,
        i32, bool
    );

    ctp_fn_wrapper!(
        "资金账户口令更新请求响应"
        trader->on_rsp_trading_account_password_update,
        &CThostFtdcTradingAccountPasswordUpdateField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "查询用户当前支持的认证模式的回复"
        trader->on_rsp_user_auth_method,
        &CThostFtdcRspUserAuthMethodField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "获取图形验证码请求的回复"
        trader->on_rsp_gen_user_captcha,
        &CThostFtdcRspGenUserCaptchaField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "获取短信验证码请求的回复"
        trader->on_rsp_gen_user_text,
        &CThostFtdcRspGenUserTextField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "登录请求响应"
        trader->on_rsp_user_login,
        &CThostFtdcRspUserLoginField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "登出请求响应"
        trader->on_rsp_user_logout,
        &CThostFtdcUserLogoutField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "投资者结算结果确认响应"
        trader->on_rsp_settlement_info_confirm,
        &CThostFtdcSettlementInfoConfirmField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询合约响应"
        trader->on_rsp_qry_instrument,
        Option<&CThostFtdcInstrumentField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询行情响应"
        trader->on_rsp_qry_depth_market_data,
        Option<&CThostFtdcDepthMarketDataField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询投资者结算结果响应"
        trader->on_rsp_qry_settlement_info,
        Option<&CThostFtdcSettlementInfoField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询转帐银行响应"
        trader->on_rsp_qry_transfer_bank,
        Option<&CThostFtdcTransferBankField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询投资者持仓明细响应"
        trader->on_rsp_qry_investor_position_detail,
        Option<&CThostFtdcInvestorPositionDetailField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询客户通知响应"
        trader->on_rsp_qry_notice,
        Option<&CThostFtdcNoticeField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询资金账户响应"
        trader->on_rsp_qry_trading_account,
        Option<&CThostFtdcTradingAccountField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询投资者响应"
        trader->on_rsp_qry_investor,
        &CThostFtdcInvestorField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询交易编码响应"
        trader->on_rsp_qry_trading_code,
        &CThostFtdcTradingCodeField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询合约保证金率响应"
        trader->on_rsp_qry_instrument_margin_rate,
        Option<&CThostFtdcInstrumentMarginRateField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询合约手续费率响应"
        trader->on_rsp_qry_instrument_commission_rate,
        Option<&CThostFtdcInstrumentCommissionRateField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询投资者持仓响应"
        trader->on_rsp_qry_investor_position,
        Option<&CThostFtdcInvestorPositionField>,
        Option<&CThostFtdcRspInfoField>,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "报单录入请求响应"
        trader->on_rsp_order_insert,
        &CThostFtdcInputOrderField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "报单操作请求响应"
        trader->on_rsp_order_action,
        &CThostFtdcInputOrderActionField,
        &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "报单通知"
        trader->on_rtn_order,
        &CThostFtdcOrderField,
    );

    ctp_fn_wrapper!(
        "执行宣告通知"
        trader->on_rtn_exec_order,
        &CThostFtdcExecOrderField,
    );

    ctp_fn_wrapper!(
        "报价通知"
        trader->on_rtn_quote,
        &CThostFtdcQuoteField,
    );

    ctp_fn_wrapper!(
        "执行宣告录入错误回报"
        trader->on_err_rtn_exec_order_insert,
        &CThostFtdcInputExecOrderField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "执行宣告操作错误回报"
        trader->on_err_rtn_exec_order_action,
        &CThostFtdcExecOrderActionField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "成交通知"
        trader->on_rtn_trade,
        &CThostFtdcTradeField,
    );

    ctp_fn_wrapper!(
        "报单录入错误回报"
        trader->on_err_rtn_order_insert,
        &CThostFtdcInputOrderField,
        &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "报单操作错误回报"
        trader->on_err_rtn_order_action,
        &CThostFtdcOrderActionField,
        &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "合约交易状态通知"
        trader->on_rtn_instrument_status,
        &CThostFtdcInstrumentStatusField,
    );

    ctp_fn_wrapper!(
        "交易所公告通知"
        trader->on_rtn_bulletin,
        &CThostFtdcBulletinField,
    );

    ctp_fn_wrapper!(
        "交易通知"
        trader->on_rtn_trading_notice,
        &CThostFtdcTradingNoticeInfoField,
    );

    ctp_fn_wrapper!(
        "提示条件单校验错误"
        trader->on_rtn_error_conditional_order,
        &CThostFtdcErrorConditionalOrderField,
    );

    ctp_fn_wrapper!(
        "报价录入错误回报"
        trader->on_err_rtn_quote_insert,
        &CThostFtdcInputQuoteField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "报价操作错误回报"
        trader->on_err_rtn_quote_action,
        &CThostFtdcQuoteActionField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "询价录入错误回报"
        trader->on_err_rtn_for_quote_insert,
        &CThostFtdcInputForQuoteField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "询价通知"
        trader->on_rtn_for_quote_rsp,
        &CThostFtdcForQuoteRspField,
    );

    ctp_fn_wrapper!(
        "保证金监控中心用户令牌"
        trader->on_rtn_cfmmc_trading_account_token,
        &CThostFtdcCFMMCTradingAccountTokenField,
    );

    ctp_fn_wrapper!(
        "批量报单操作错误回报"
        trader->on_err_rtn_batch_order_action,
        &CThostFtdcBatchOrderActionField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "期权自对冲通知"
        trader->on_rtn_option_self_close,
        &CThostFtdcOptionSelfCloseField,
    );

    ctp_fn_wrapper!(
        "期权自对冲录入错误回报"
        trader->on_err_rtn_option_self_close_insert,
        &CThostFtdcInputOptionSelfCloseField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "期权自对冲操作错误回报"
        trader->on_err_rtn_option_self_close_action,
        &CThostFtdcOptionSelfCloseActionField, &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "申请组合录入错误回报"
        trader->on_err_rtn_comb_action_insert,
        pInputCombAction: &CThostFtdcInputCombActionField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "请求查询签约银行响应"
        trader->on_rsp_qry_contract_bank,
        pContractBank: &CThostFtdcContractBankField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询预埋单响应"
        trader->on_rsp_qry_parked_order,
        pParkedOrder: &CThostFtdcParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询预埋撤单响应"
        trader->on_rsp_qry_parked_order_action,
        pParkedOrderAction: &CThostFtdcParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询交易通知响应"
        trader->on_rsp_qry_trading_notice,
        pTradingNotice: &CThostFtdcTradingNoticeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询经纪公司交易参数响应"
        trader->on_rsp_qry_broker_trading_params,
        pBrokerTradingParams: &CThostFtdcBrokerTradingParamsField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询经纪公司交易算法响应"
        trader->on_rsp_qry_broker_trading_algos,
        pBrokerTradingParams: &CThostFtdcBrokerTradingAlgosField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询监控中心用户令牌"
        trader->on_rsp_query_cfmmc_trading_account_token,
        pQueryCFMMCTradingAccountToken: &CThostFtdcQueryCFMMCTradingAccountTokenField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "银行发起银行资金转期货通知"
        trader->on_rtn_from_bank_to_future_by_bank,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "银行发起期货资金转银行通知"
        trader->on_rtn_from_future_to_bank_by_bank,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "银行发起冲正银行转期货通知"
        trader->on_rtn_repeal_from_bank_to_future_by_bank,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "银行发起冲正期货转银行通知"
        trader->on_rtn_repeal_from_future_to_bank_by_bank,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "申请组合通知"
        trader->on_rtn_comb_action,
        &CThostFtdcCombActionField,
    );

    ctp_fn_wrapper!(
        "预埋单录入请求响应"
        trader->on_rsp_parked_order_insert,
        pParkedOrder: &CThostFtdcParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "预埋撤单录入请求响应"
        trader->on_rsp_parked_order_action,
        pParkedOrder: &CThostFtdcParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "查询最大报单数量响应"
        trader->on_rsp_query_max_order_volume,
        pQueryMaxOrderVolume: &CThostFtdcQueryMaxOrderVolumeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "删除预埋单响应"
        trader->on_rsp_remove_parked_order,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "删除预埋撤单响应"
        trader->on_rsp_remove_parked_order_action,
        pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "执行宣告录入请求响应"
        trader->on_rsp_exec_order_insert,
        pInputExecOrder: &CThostFtdcInputExecOrderField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询报单响应"
        trader->on_rsp_qry_order,
        &CThostFtdcOrderField,
        &CThostFtdcRspInfoField,
        i32, bool,
    );

    ctp_fn_wrapper!(
        "请求查询成交响应"
        trader->on_rsp_qry_trade,
        &CThostFtdcTradeField,
        &CThostFtdcRspInfoField,
        i32,
        bool,
    );

    ctp_fn_wrapper!(
        "执行宣告操作请求响应"
        trader->on_rsp_exec_order_action,
        pInputExecOrderAction: &CThostFtdcInputExecOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "询价录入请求响应"
        trader->on_rsp_for_quote_insert,
        pInputForQuote: &CThostFtdcInputForQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "报价录入请求响应"
        trader->on_rsp_quote_insert,
        pInputQuote: &CThostFtdcInputQuoteField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "报价操作请求响应"
        trader->on_rsp_quote_action,
        pInputQuoteAction: &CThostFtdcInputQuoteActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "批量报单操作请求响应"
        trader->on_rsp_batch_order_action,
        pInputBatchOrderAction: &CThostFtdcInputBatchOrderActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "期权自对冲录入请求响应"
        trader->on_rsp_option_self_close_insert,
        pInputOptionSelfClose: &CThostFtdcInputOptionSelfCloseField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "期权自对冲操作请求响应"
        trader->on_rsp_option_self_close_action,
        pInputOptionSelfCloseAction: &CThostFtdcInputOptionSelfCloseActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "申请组合录入请求响应"
        trader->on_rsp_comb_action_insert,
        pInputCombAction: &CThostFtdcInputCombActionField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询交易所响应"
        trader->on_rsp_qry_exchange,
        pExchange: &CThostFtdcExchangeField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "请求查询产品响应"
        trader->on_rsp_qry_product,
        pProduct: &CThostFtdcProductField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "期货发起银行资金转期货通知"
        trader->on_rtn_from_bank_to_future_by_future,
        pRspTransfer: &CThostFtdcRspTransferField,
    );

    ctp_fn_wrapper!(
        "期货发起期货资金转银行通知"
        trader->on_rtn_from_future_to_bank_by_future,
        pRspTransfer: &CThostFtdcRspTransferField,
    );

    ctp_fn_wrapper!(
        "系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_bank_to_future_by_future_manual,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_future_to_bank_by_future_manual,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "期货发起查询银行余额通知"
        trader->on_rtn_query_bank_balance_by_future,
        pNotifyQueryAccount: &CThostFtdcNotifyQueryAccountField,
    );

    ctp_fn_wrapper!(
        "期货发起银行资金转期货错误回报"
        trader->on_err_rtn_bank_to_future_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "期货发起期货资金转银行错误回报"
        trader->on_err_rtn_future_to_bank_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "系统运行时期货端手工发起冲正银行转期货错误回报"
        trader->on_err_rtn_repeal_bank_to_future_by_future_manual,
        pReqRepeal: &CThostFtdcReqRepealField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "系统运行时期货端手工发起冲正期货转银行错误回报"
        trader->on_err_rtn_repeal_future_to_bank_by_future_manual,
        pReqRepeal: &CThostFtdcReqRepealField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "期货发起查询银行余额错误回报"
        trader->on_err_rtn_query_bank_balance_by_future,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField,
        pRspInfo: &CThostFtdcRspInfoField,
    );

    ctp_fn_wrapper!(
        "期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_bank_to_future_by_future,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
        trader->on_rtn_repeal_from_future_to_bank_by_future,
        pRspRepeal: &CThostFtdcRspRepealField,
    );

    ctp_fn_wrapper!(
        "期货发起银行资金转期货应答"
        trader->on_rsp_from_bank_to_future_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "期货发起期货资金转银行应答"
        trader->on_rsp_from_future_to_bank_by_future,
        pReqTransfer: &CThostFtdcReqTransferField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "期货发起查询银行余额应答"
        trader->on_rsp_query_bank_account_money_by_future,
        pReqQueryAccount: &CThostFtdcReqQueryAccountField,
        pRspInfo: &CThostFtdcRspInfoField,
        nRequestID: i32,
        bIsLast: bool,
    );

    ctp_fn_wrapper!(
        "银行发起银期开户通知"
        trader->on_rtn_open_account_by_bank,
        pOpenAccount: &CThostFtdcOpenAccountField,
    );

    ctp_fn_wrapper!(
        "银行发起银期销户通知"
        trader->on_rtn_cancel_account_by_bank,
        pCancelAccount: &CThostFtdcCancelAccountField,
    );

    ctp_fn_wrapper!(
        "银行发起变更银行账号通知"
        trader->on_rtn_change_account_by_bank,
        pChangeAccount: &CThostFtdcChangeAccountField,
    );
}

impl Drop for CTPTraderSpi {
    fn drop(&mut self) {
        tracing::debug!("Drop CTPTraderSpi");

        ctp_fn_dropper!(
            "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
            self,
            trader->on_front_connected,
        );

        ctp_fn_dropper!(
            "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。"
            "@param nReason 错误原因"
            "        0x1001 网络读失败"
            "        0x1002 网络写失败"
            "        0x2001 接收心跳超时"
            "        0x2002 发送心跳失败"
            "        0x2003 收到错误报文"
            self,
            trader->on_front_disconnected,
            reason: i32,
        );

        ctp_fn_dropper!(
            "心跳超时警告。当长时间未收到报文时，该方法被调用。"
            "@param nTimeLapse 距离上次接收报文的时间"
            "注意: 这个方法永远不会被调用到"
            self,
            trader->on_heartbeat_warning,
            i32,
        );

        ctp_fn_dropper!(
            "错误应答"
            self,
            trader->on_rsp_error,
            &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "客户端认证响应"
            self,
            trader->on_rsp_authenticate,
            &CThostFtdcRspAuthenticateField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "用户口令更新请求响应"
            self,
            trader->on_rsp_user_password_update,
            &CThostFtdcUserPasswordUpdateField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "资金账户口令更新请求响应"
            self,
            trader->on_rsp_trading_account_password_update,
            &CThostFtdcTradingAccountPasswordUpdateField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "查询用户当前支持的认证模式的回复"
            self,
            trader->on_rsp_user_auth_method,
            &CThostFtdcRspUserAuthMethodField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "获取图形验证码请求的回复"
            self,
            trader->on_rsp_gen_user_captcha,
            &CThostFtdcRspGenUserCaptchaField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "获取短信验证码请求的回复"
            self,
            trader->on_rsp_gen_user_text,
            &CThostFtdcRspGenUserTextField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "登录请求响应"
            self,
            trader->on_rsp_user_login,
            &CThostFtdcRspUserLoginField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "登出请求响应"
            self,
            trader->on_rsp_user_logout,
            &CThostFtdcUserLogoutField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "投资者结算结果确认响应"
            self,
            trader->on_rsp_settlement_info_confirm,
            &CThostFtdcSettlementInfoConfirmField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询合约响应"
            self,
            trader->on_rsp_qry_instrument,
            Option<&CThostFtdcInstrumentField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询行情响应"
            self,
            trader->on_rsp_qry_depth_market_data,
            Option<&CThostFtdcDepthMarketDataField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询投资者结算结果响应"
            self,
            trader->on_rsp_qry_settlement_info,
            Option<&CThostFtdcSettlementInfoField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询转帐银行响应"
            self,
            trader->on_rsp_qry_transfer_bank,
            Option<&CThostFtdcTransferBankField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询投资者持仓明细响应"
            self,
            trader->on_rsp_qry_investor_position_detail,
            Option<&CThostFtdcInvestorPositionDetailField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询客户通知响应"
            self,
            trader->on_rsp_qry_notice,
            Option<&CThostFtdcNoticeField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询资金账户响应"
            self,
            trader->on_rsp_qry_trading_account,
            Option<&CThostFtdcTradingAccountField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询投资者响应"
            self,
            trader->on_rsp_qry_investor,
            &CThostFtdcInvestorField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询交易编码响应"
            self,
            trader->on_rsp_qry_trading_code,
            &CThostFtdcTradingCodeField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询合约保证金率响应"
            self,
            trader->on_rsp_qry_instrument_margin_rate,
            Option<&CThostFtdcInstrumentMarginRateField>,
            Option<&CThostFtdcRspInfoField>,
            i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询合约手续费率响应"
            self,
            trader->on_rsp_qry_instrument_commission_rate,
            Option<&CThostFtdcInstrumentCommissionRateField>,
            Option<&CThostFtdcRspInfoField>,
            i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询投资者持仓响应"
            self,
            trader->on_rsp_qry_investor_position,
            Option<&CThostFtdcInvestorPositionField>, Option<&CThostFtdcRspInfoField>, i32, bool,
        );

        ctp_fn_dropper!(
            "报单录入请求响应"
            self,
            trader->on_rsp_order_insert,
            &CThostFtdcInputOrderField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "报单操作请求响应"
            self,
            trader->on_rsp_order_action,
            &CThostFtdcInputOrderActionField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "报单通知"
            self,
            trader->on_rtn_order,
            &CThostFtdcOrderField,
        );

        ctp_fn_dropper!(
            "成交通知"
            self,
            trader->on_rtn_trade,
            &CThostFtdcTradeField,
        );

        ctp_fn_dropper!(
            "报单通知"
            self,
            trader->on_rtn_quote,
            &CThostFtdcQuoteField,
        );

        ctp_fn_dropper!(
            "报单录入错误回报"
            self,
            trader->on_err_rtn_order_insert,
            &CThostFtdcInputOrderField,
            &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "报单操作错误回报"
            self,
            trader->on_err_rtn_order_action,
            &CThostFtdcOrderActionField,
            &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "合约交易状态通知"
            self,
            trader->on_rtn_instrument_status,
            &CThostFtdcInstrumentStatusField,
        );

        ctp_fn_dropper!(
            "交易所公告通知"
            self,
            trader->on_rtn_bulletin,
            &CThostFtdcBulletinField,
        );

        ctp_fn_dropper!(
            "交易通知"
            self,
            trader->on_rtn_trading_notice,
            &CThostFtdcTradingNoticeInfoField,
        );

        ctp_fn_dropper!(
            "提示条件单校验错误"
            self,
            trader->on_rtn_error_conditional_order,
            &CThostFtdcErrorConditionalOrderField,
        );

        ctp_fn_dropper!(
            "执行宣告通知"
            self,
            trader->on_rtn_exec_order,
            &CThostFtdcExecOrderField,
        );

        ctp_fn_dropper!(
            "执行宣告录入错误回报"
            self,
            trader->on_err_rtn_exec_order_insert,
            &CThostFtdcInputExecOrderField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "执行宣告操作错误回报"
            self,
            trader->on_err_rtn_exec_order_action,
            &CThostFtdcExecOrderActionField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "报价录入错误回报"
            self,
            trader->on_err_rtn_quote_insert,
            &CThostFtdcInputQuoteField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "报价操作错误回报"
            self,
            trader->on_err_rtn_quote_action,
            &CThostFtdcQuoteActionField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "询价录入错误回报"
            self,
            trader->on_err_rtn_for_quote_insert,
            &CThostFtdcInputForQuoteField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "询价通知"
            self,
            trader->on_rtn_for_quote_rsp,
            &CThostFtdcForQuoteRspField,
        );

        ctp_fn_dropper!(
            "保证金监控中心用户令牌"
            self,
            trader->on_rtn_cfmmc_trading_account_token,
            &CThostFtdcCFMMCTradingAccountTokenField,
        );

        ctp_fn_dropper!(
            "批量报单操作错误回报"
            self,
            trader->on_err_rtn_batch_order_action,
            &CThostFtdcBatchOrderActionField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "期权自对冲通知"
            self,
            trader->on_rtn_option_self_close,
            &CThostFtdcOptionSelfCloseField,
        );

        ctp_fn_dropper!(
            "期权自对冲录入错误回报"
            self,
            trader->on_err_rtn_option_self_close_insert,
            &CThostFtdcInputOptionSelfCloseField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "期权自对冲操作错误回报"
            self,
            trader->on_err_rtn_option_self_close_action,
            &CThostFtdcOptionSelfCloseActionField, &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "申请组合录入错误回报"
            self,
            trader->on_err_rtn_comb_action_insert,
            pInputCombAction: &CThostFtdcInputCombActionField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "请求查询签约银行响应"
            self,
            trader->on_rsp_qry_contract_bank,
            pContractBank: &CThostFtdcContractBankField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询预埋单响应"
            self,
            trader->on_rsp_qry_parked_order,
            pParkedOrder: &CThostFtdcParkedOrderField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询预埋撤单响应"
            self,
            trader->on_rsp_qry_parked_order_action,
            pParkedOrderAction: &CThostFtdcParkedOrderActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询交易通知响应"
            self,
            trader->on_rsp_qry_trading_notice,
            pTradingNotice: &CThostFtdcTradingNoticeField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询经纪公司交易参数响应"
            self,
            trader->on_rsp_qry_broker_trading_params,
            pBrokerTradingParams: &CThostFtdcBrokerTradingParamsField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询经纪公司交易算法响应"
            self,
            trader->on_rsp_qry_broker_trading_algos,
            pBrokerTradingParams: &CThostFtdcBrokerTradingAlgosField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询监控中心用户令牌"
            self,
            trader->on_rsp_query_cfmmc_trading_account_token,
            pQueryCFMMCTradingAccountToken: &CThostFtdcQueryCFMMCTradingAccountTokenField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "银行发起银行资金转期货通知"
            self,
            trader->on_rtn_from_bank_to_future_by_bank,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "银行发起期货资金转银行通知"
            self,
            trader->on_rtn_from_future_to_bank_by_bank,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "银行发起冲正银行转期货通知"
            self,
            trader->on_rtn_repeal_from_bank_to_future_by_bank,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "银行发起冲正期货转银行通知"
            self,
            trader->on_rtn_repeal_from_future_to_bank_by_bank,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "申请组合通知"
            self,
            trader->on_rtn_comb_action,
            &CThostFtdcCombActionField,
        );

        ctp_fn_dropper!(
            "预埋单录入请求响应"
            self,
            trader->on_rsp_parked_order_insert,
            pParkedOrder: &CThostFtdcParkedOrderField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "预埋撤单录入请求响应"
            self,
            trader->on_rsp_parked_order_action,
            pParkedOrder: &CThostFtdcParkedOrderActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "查询最大报单数量响应"
            self,
            trader->on_rsp_query_max_order_volume,
            pQueryMaxOrderVolume: &CThostFtdcQueryMaxOrderVolumeField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "删除预埋单响应"
            self,
            trader->on_rsp_remove_parked_order,
            pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "删除预埋撤单响应"
            self,
            trader->on_rsp_remove_parked_order_action,
            pRemoveParkedOrder: &CThostFtdcRemoveParkedOrderActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "执行宣告录入请求响应"
            self,
            trader->on_rsp_exec_order_insert,
            pInputExecOrder: &CThostFtdcInputExecOrderField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询报单响应"
            self,
            trader->on_rsp_qry_order,
            &CThostFtdcOrderField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "请求查询成交响应"
            self,
            trader->on_rsp_qry_trade,
            &CThostFtdcTradeField,
            &CThostFtdcRspInfoField,
            i32, bool,
        );

        ctp_fn_dropper!(
            "执行宣告操作请求响应"
            self,
            trader->on_rsp_exec_order_action,
            pInputExecOrderAction: &CThostFtdcInputExecOrderActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "询价录入请求响应"
            self,
            trader->on_rsp_for_quote_insert,
            pInputForQuote: &CThostFtdcInputForQuoteField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "报价录入请求响应"
            self,
            trader->on_rsp_quote_insert,
            pInputQuote: &CThostFtdcInputQuoteField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "报价操作请求响应"
            self,
            trader->on_rsp_quote_action,
            pInputQuoteAction: &CThostFtdcInputQuoteActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "批量报单操作请求响应"
            self,
            trader->on_rsp_batch_order_action,
            pInputBatchOrderAction: &CThostFtdcInputBatchOrderActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "期权自对冲录入请求响应"
            self,
            trader->on_rsp_option_self_close_insert,
            pInputOptionSelfClose: &CThostFtdcInputOptionSelfCloseField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "期权自对冲操作请求响应"
            self,
            trader->on_rsp_option_self_close_action,
            pInputOptionSelfCloseAction: &CThostFtdcInputOptionSelfCloseActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "申请组合录入请求响应"
            self,
            trader->on_rsp_comb_action_insert,
            pInputCombAction: &CThostFtdcInputCombActionField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询交易所响应"
            self,
            trader->on_rsp_qry_exchange,
            pExchange: &CThostFtdcExchangeField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "请求查询产品响应"
            self,
            trader->on_rsp_qry_product,
            pProduct: &CThostFtdcProductField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "期货发起银行资金转期货通知"
            self,
            trader->on_rtn_from_bank_to_future_by_future,
            pRspTransfer: &CThostFtdcRspTransferField,
        );

        ctp_fn_dropper!(
            "期货发起期货资金转银行通知"
            self,
            trader->on_rtn_from_future_to_bank_by_future,
            pRspTransfer: &CThostFtdcRspTransferField,
        );

        ctp_fn_dropper!(
            "系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
            self,
            trader->on_rtn_repeal_from_bank_to_future_by_future_manual,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
            self,
            trader->on_rtn_repeal_from_future_to_bank_by_future_manual,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "期货发起查询银行余额通知"
            self,
            trader->on_rtn_query_bank_balance_by_future,
            pNotifyQueryAccount: &CThostFtdcNotifyQueryAccountField,
        );

        ctp_fn_dropper!(
            "期货发起银行资金转期货错误回报"
            self,
            trader->on_err_rtn_bank_to_future_by_future,
            pReqTransfer: &CThostFtdcReqTransferField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "期货发起期货资金转银行错误回报"
            self,
            trader->on_err_rtn_future_to_bank_by_future,
            pReqTransfer: &CThostFtdcReqTransferField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "系统运行时期货端手工发起冲正银行转期货错误回报"
            self,
            trader->on_err_rtn_repeal_bank_to_future_by_future_manual,
            pReqRepeal: &CThostFtdcReqRepealField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "系统运行时期货端手工发起冲正期货转银行错误回报"
            self,
            trader->on_err_rtn_repeal_future_to_bank_by_future_manual,
            pReqRepeal: &CThostFtdcReqRepealField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "期货发起查询银行余额错误回报"
            self,
            trader->on_err_rtn_query_bank_balance_by_future,
            pReqQueryAccount: &CThostFtdcReqQueryAccountField,
            pRspInfo: &CThostFtdcRspInfoField,
        );

        ctp_fn_dropper!(
            "期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知"
            self,
            trader->on_rtn_repeal_from_bank_to_future_by_future,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知"
            self,
            trader->on_rtn_repeal_from_future_to_bank_by_future,
            pRspRepeal: &CThostFtdcRspRepealField,
        );

        ctp_fn_dropper!(
            "期货发起银行资金转期货应答"
            self,
            trader->on_rsp_from_bank_to_future_by_future,
            pReqTransfer: &CThostFtdcReqTransferField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "期货发起期货资金转银行应答"
            self,
            trader->on_rsp_from_future_to_bank_by_future,
            pReqTransfer: &CThostFtdcReqTransferField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "期货发起查询银行余额应答"
            self,
            trader->on_rsp_query_bank_account_money_by_future,
            pReqQueryAccount: &CThostFtdcReqQueryAccountField,
            pRspInfo: &CThostFtdcRspInfoField,
            nRequestID: i32,
            bIsLast: bool,
        );

        ctp_fn_dropper!(
            "银行发起银期开户通知"
            self,
            trader->on_rtn_open_account_by_bank,
            pOpenAccount: &CThostFtdcOpenAccountField,
        );

        ctp_fn_dropper!(
            "银行发起银期销户通知"
            self,
            trader->on_rtn_cancel_account_by_bank,
            pCancelAccount: &CThostFtdcCancelAccountField,
        );

        ctp_fn_dropper!(
            "银行发起变更银行账号通知"
            self,
            trader->on_rtn_change_account_by_bank,
            pChangeAccount: &CThostFtdcChangeAccountField,
        );
    }
}
