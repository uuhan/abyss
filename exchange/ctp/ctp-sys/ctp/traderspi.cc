#include <ctp.h>
#include <iostream>

/// NB: CTP 很多返回 pRspInfo 为空, 可能导致段错误
#define RUST_CALL0(fn)                                     \
    if (_traderspi_##fn##_cb) {                            \
        rust_traderspi_##fn(_traderspi_##fn##_cb);         \
    } else {                                               \
        rust_log_warn("[traderspi.cc] rust_traderspi_" #fn \
                      " not implemented.");                \
    }
#define RUST_CALL(fn, ...)                                      \
    if (_traderspi_##fn##_cb) {                                 \
        rust_traderspi_##fn(_traderspi_##fn##_cb, __VA_ARGS__); \
    } else {                                                    \
        rust_log_warn("[traderspi.cc] rust_traderspi_" #fn      \
                      " not implemented.");                     \
    }

extern "C" {
void rust_traderspi_on_front_connected(void *data);
}
/// 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
void
CTPTraderSpi::OnFrontConnected()
{
    RUST_CALL0(on_front_connected);
}

extern "C" {
void rust_traderspi_on_rsp_user_login(
    void *data, CThostFtdcRspUserLoginField *pRspUserLogin,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 登录请求响应
void
CTPTraderSpi::OnRspUserLogin(CThostFtdcRspUserLoginField *pRspField,
                             CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                             bool bIsLast)
{
    RUST_CALL(on_rsp_user_login, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_error(void *data, CThostFtdcRspInfoField *pRspInfo,
                                 int nRequestID, bool bIsLast);
}
/// 错误应答
void
CTPTraderSpi::OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast)
{
    RUST_CALL(on_rsp_error, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_front_disconnected(void *data, int nReason);
}
/// 当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
void
CTPTraderSpi::OnFrontDisconnected(int nReason)
{
    RUST_CALL(on_front_disconnected, nReason);
}

extern "C" {
void rust_traderspi_on_heartbeat_warning(void *data, int nTimeLapse);
}
/// 心跳超时警告。当长时间未收到报文时，该方法被调用。
void
CTPTraderSpi::OnHeartBeatWarning(int nTimeLapse)
{
    RUST_CALL(on_heartbeat_warning, nTimeLapse);
}

extern "C" {
void rust_traderspi_on_rsp_authenticate(
    void *data, CThostFtdcRspAuthenticateField *pRspAuthenticateField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 客户端认证响应
void
CTPTraderSpi::OnRspAuthenticate(CThostFtdcRspAuthenticateField *pRspField,
                                CThostFtdcRspInfoField *pRspInfo,
                                int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_authenticate, pRspField, pRspInfo, nRequestID, bIsLast);
};


extern "C" {
void rust_traderspi_on_rsp_user_logout(void *data,
                                       CThostFtdcUserLogoutField *pUserLogout,
                                       CThostFtdcRspInfoField *pRspInfo,
                                       int nRequestID, bool bIsLast);
}
/// 登出请求响应
void
CTPTraderSpi::OnRspUserLogout(CThostFtdcUserLogoutField *pRspField,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast)
{
    RUST_CALL(on_rsp_user_logout, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_user_password_update(
    void *data, CThostFtdcUserPasswordUpdateField *pUserPasswordUpdate,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 用户口令更新请求响应
void
CTPTraderSpi::OnRspUserPasswordUpdate(
    CThostFtdcUserPasswordUpdateField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_user_password_update, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_trading_account_password_update(
    void *data,
    CThostFtdcTradingAccountPasswordUpdateField *pTradingAccountPasswordUpdate,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 资金账户口令更新请求响应
void
CTPTraderSpi::OnRspTradingAccountPasswordUpdate(
    CThostFtdcTradingAccountPasswordUpdateField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_trading_account_password_update, pRspField, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_user_auth_method(
    void *data, CThostFtdcRspUserAuthMethodField *pRspUserAuthMethod,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 查询用户当前支持的认证模式的回复
void
CTPTraderSpi::OnRspUserAuthMethod(CThostFtdcRspUserAuthMethodField *pRspField,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_user_auth_method, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_gen_user_captcha(
    void *data, CThostFtdcRspGenUserCaptchaField *pRspGenUserText,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 获取图形验证码请求的回复
void
CTPTraderSpi::OnRspGenUserCaptcha(CThostFtdcRspGenUserCaptchaField *pRspField,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_gen_user_captcha, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_gen_user_text(
    void *data, CThostFtdcRspGenUserTextField *pRspGenUserCaptcha,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 获取短信验证码请求的回复
void
CTPTraderSpi::OnRspGenUserText(CThostFtdcRspGenUserTextField *pRspGenUserText,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_gen_user_text, pRspGenUserText, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_parked_order_insert(
    void *data, CThostFtdcParkedOrderField *pParkedOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 预埋单录入请求响应
void
CTPTraderSpi::OnRspParkedOrderInsert(CThostFtdcParkedOrderField *pParkedOrder,
                                     CThostFtdcRspInfoField *pRspInfo,
                                     int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_parked_order_insert, pParkedOrder, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_parked_order_action(
    void *data, CThostFtdcParkedOrderActionField *pParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 预埋撤单录入请求响应
void
CTPTraderSpi::OnRspParkedOrderAction(
    CThostFtdcParkedOrderActionField *pParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_parked_order_action, pParkedOrderAction, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
#ifdef v6_7_0
void rust_traderspi_on_rsp_query_max_order_volume(
    void *data, CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
#else
void rust_traderspi_on_rsp_query_max_order_volume(
    void *data, CThostFtdcQueryMaxOrderVolumeField *pQueryMaxOrderVolume,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
#endif

/// 查询最大报单数量响应
#ifdef v6_7_0
void
CTPTraderSpi::OnRspQryMaxOrderVolume(
    CThostFtdcQryMaxOrderVolumeField *pQryMaxOrderVolume,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_query_max_order_volume, pQryMaxOrderVolume, pRspInfo,
              nRequestID, bIsLast);
}
#else
void
CTPTraderSpi::OnRspQueryMaxOrderVolume(
    CThostFtdcQueryMaxOrderVolumeField *pQueryMaxOrderVolume,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_query_max_order_volume, pQueryMaxOrderVolume, pRspInfo,
              nRequestID, bIsLast);
}
#endif

extern "C" {
void rust_traderspi_on_rsp_remove_parked_order(
    void *data, CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 删除预埋单响应
void
CTPTraderSpi::OnRspRemoveParkedOrder(
    CThostFtdcRemoveParkedOrderField *pRemoveParkedOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_remove_parked_order, pRemoveParkedOrder, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_remove_parked_order_action(
    void *data,
    CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 删除预埋撤单响应
void
CTPTraderSpi::OnRspRemoveParkedOrderAction(
    CThostFtdcRemoveParkedOrderActionField *pRemoveParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_remove_parked_order_action, pRemoveParkedOrderAction,
              pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_exec_order_insert(
    void *data, CThostFtdcInputExecOrderField *pInputExecOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 执行宣告录入请求响应
void
CTPTraderSpi::OnRspExecOrderInsert(
    CThostFtdcInputExecOrderField *pInputExecOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_exec_order_insert, pInputExecOrder, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_exec_order_action(
    void *data, CThostFtdcInputExecOrderActionField *pInputExecOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 执行宣告操作请求响应
void
CTPTraderSpi::OnRspExecOrderAction(
    CThostFtdcInputExecOrderActionField *pInputExecOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_exec_order_action, pInputExecOrderAction, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_for_quote_insert(
    void *data, CThostFtdcInputForQuoteField *pInputForQuote,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 询价录入请求响应
void
CTPTraderSpi::OnRspForQuoteInsert(CThostFtdcInputForQuoteField *pInputForQuote,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_for_quote_insert, pInputForQuote, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_quote_insert(void *data,
                                        CThostFtdcInputQuoteField *pInputQuote,
                                        CThostFtdcRspInfoField *pRspInfo,
                                        int nRequestID, bool bIsLast);
}
/// 报价录入请求响应
void
CTPTraderSpi::OnRspQuoteInsert(CThostFtdcInputQuoteField *pInputQuote,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_quote_insert, pInputQuote, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_quote_action(
    void *data, CThostFtdcInputQuoteActionField *pInputQuote,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 报价操作请求响应
void
CTPTraderSpi::OnRspQuoteAction(
    CThostFtdcInputQuoteActionField *pInputQuoteAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_quote_action, pInputQuoteAction, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_batch_order_action(
    void *data, CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 批量报单操作请求响应
void
CTPTraderSpi::OnRspBatchOrderAction(
    CThostFtdcInputBatchOrderActionField *pInputBatchOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_batch_order_action, pInputBatchOrderAction, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_option_self_close_insert(
    void *data, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 期权自对冲录入请求响应
void
CTPTraderSpi::OnRspOptionSelfCloseInsert(
    CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_option_self_close_insert, pInputOptionSelfClose, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_option_self_close_action(
    void *data,
    CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 期权自对冲操作请求响应
void
CTPTraderSpi::OnRspOptionSelfCloseAction(
    CThostFtdcInputOptionSelfCloseActionField *pInputOptionSelfCloseAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_option_self_close_action, pInputOptionSelfCloseAction,
              pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_comb_action_insert(
    void *data, CThostFtdcInputCombActionField *pInputCombAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 申请组合录入请求响应
void
CTPTraderSpi::OnRspCombActionInsert(
    CThostFtdcInputCombActionField *pInputCombAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_comb_action_insert, pInputCombAction, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_order(void *data, CThostFtdcOrderField *pOrder,
                                     CThostFtdcRspInfoField *pRspInfo,
                                     int nRequestID, bool bIsLast);
}
/// 请求查询报单响应
void
CTPTraderSpi::OnRspQryOrder(CThostFtdcOrderField *pOrder,
                            CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                            bool bIsLast)
{
    RUST_CALL(on_rsp_qry_order, pOrder, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_trade(void *data, CThostFtdcTradeField *pTrade,
                                     CThostFtdcRspInfoField *pRspInfo,
                                     int nRequestID, bool bIsLast);
}
/// 请求查询成交响应
void
CTPTraderSpi::OnRspQryTrade(CThostFtdcTradeField *pTrade,
                            CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                            bool bIsLast)
{
    RUST_CALL(on_rsp_qry_trade, pTrade, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_settlement_info_confirm(
    void *data, CThostFtdcSettlementInfoConfirmField *pSettlementInfoConfirm,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 投资者结算结果确认响应
void
CTPTraderSpi::OnRspSettlementInfoConfirm(
    CThostFtdcSettlementInfoConfirmField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_settlement_info_confirm, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_instrument(
    void *data, CThostFtdcInstrumentField *pInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询合约响应
void
CTPTraderSpi::OnRspQryInstrument(CThostFtdcInstrumentField *pRspField,
                                 CThostFtdcRspInfoField *pRspInfo,
                                 int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_instrument, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_depth_market_data(
    void *data, CThostFtdcDepthMarketDataField *pDepthMarketData,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询行情响应
void
CTPTraderSpi::OnRspQryDepthMarketData(CThostFtdcDepthMarketDataField *pRspField,
                                      CThostFtdcRspInfoField *pRspInfo,
                                      int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_depth_market_data, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_settlement_info(
    void *data, CThostFtdcSettlementInfoField *pSettlementInfo,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询投资者结算结果响应
void
CTPTraderSpi::OnRspQrySettlementInfo(CThostFtdcSettlementInfoField *pRspField,
                                     CThostFtdcRspInfoField *pRspInfo,
                                     int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_settlement_info, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_transfer_bank(
    void *data, CThostFtdcTransferBankField *pTransferBank,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询转帐银行响应
void
CTPTraderSpi::OnRspQryTransferBank(CThostFtdcTransferBankField *pRspField,
                                   CThostFtdcRspInfoField *pRspInfo,
                                   int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_transfer_bank, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_investor_position_detail(
    void *data, CThostFtdcInvestorPositionDetailField *pInvestorPositionDetail,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询投资者持仓明细响应
void
CTPTraderSpi::OnRspQryInvestorPositionDetail(
    CThostFtdcInvestorPositionDetailField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_investor_position_detail, pRspField, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_notice(void *data,
                                      CThostFtdcNoticeField *pNotice,
                                      CThostFtdcRspInfoField *pRspInfo,
                                      int nRequestID, bool bIsLast);
}
/// 请求查询客户通知响应
void
CTPTraderSpi::OnRspQryNotice(CThostFtdcNoticeField *pRspField,
                             CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                             bool bIsLast)
{
    RUST_CALL(on_rsp_qry_notice, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_trading_account(
    void *data, CThostFtdcTradingAccountField *pTradingAccount,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询资金账户响应
void
CTPTraderSpi::OnRspQryTradingAccount(CThostFtdcTradingAccountField *pRspField,
                                     CThostFtdcRspInfoField *pRspInfo,
                                     int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_trading_account, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_investor(void *data,
                                        CThostFtdcInvestorField *pInvestor,
                                        CThostFtdcRspInfoField *pRspInfo,
                                        int nRequestID, bool bIsLast);
}
/// 请求查询投资者响应
void
CTPTraderSpi::OnRspQryInvestor(CThostFtdcInvestorField *pRspField,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_qry_investor, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_trading_code(
    void *data, CThostFtdcTradingCodeField *pTradingCode,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询交易编码响应
void
CTPTraderSpi::OnRspQryTradingCode(CThostFtdcTradingCodeField *pRspField,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_trading_code, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_instrument_margin_rate(
    void *data, CThostFtdcInstrumentMarginRateField *pInstrumentMarginRate,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询合约保证金率响应
void
CTPTraderSpi::OnRspQryInstrumentMarginRate(
    CThostFtdcInstrumentMarginRateField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_instrument_margin_rate, pRspField, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_instrument_commission_rate(
    void *data,
    CThostFtdcInstrumentCommissionRateField *pInstrumentCommissionRate,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询合约手续费率响应
void
CTPTraderSpi::OnRspQryInstrumentCommissionRate(
    CThostFtdcInstrumentCommissionRateField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_instrument_commission_rate, pRspField, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_exchange(void *data,
                                        CThostFtdcExchangeField *pExchange,
                                        CThostFtdcRspInfoField *pRspInfo,
                                        int nRequestID, bool bIsLast);
}
/// 请求查询交易所响应
void
CTPTraderSpi::OnRspQryExchange(CThostFtdcExchangeField *pExchange,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_qry_exchange, pExchange, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_product(void *data,
                                       CThostFtdcProductField *pProduct,
                                       CThostFtdcRspInfoField *pRspInfo,
                                       int nRequestID, bool bIsLast);
}
/// 请求查询产品响应
void
CTPTraderSpi::OnRspQryProduct(CThostFtdcProductField *pProduct,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast)
{
    RUST_CALL(on_rsp_qry_product, pProduct, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_investor_position(
    void *data, CThostFtdcInvestorPositionField *pInvestorPosition,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询投资者持仓响应
void
CTPTraderSpi::OnRspQryInvestorPosition(
    CThostFtdcInvestorPositionField *pRspField,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_investor_position, pRspField, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_order_insert(void *data,
                                        CThostFtdcInputOrderField *pInputOrder,
                                        CThostFtdcRspInfoField *pRspInfo,
                                        int nRequestID, bool bIsLast);
}
/// 报单录入请求响应
void
CTPTraderSpi::OnRspOrderInsert(CThostFtdcInputOrderField *pRspField,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_order_insert, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_order_action(
    void *data, CThostFtdcInputOrderActionField *pInputOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 报单操作请求响应
void
CTPTraderSpi::OnRspOrderAction(CThostFtdcInputOrderActionField *pRspField,
                               CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                               bool bIsLast)
{
    RUST_CALL(on_rsp_order_action, pRspField, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rtn_order(void *data, CThostFtdcOrderField *pOrder);
}
/// 报单通知
void
CTPTraderSpi::OnRtnOrder(CThostFtdcOrderField *pOrder)
{
    RUST_CALL(on_rtn_order, pOrder);
}

extern "C" {
void rust_traderspi_on_rtn_trade(void *data, CThostFtdcTradeField *pTrade);
}
/// 成交通知
void
CTPTraderSpi::OnRtnTrade(CThostFtdcTradeField *pTrade)
{
    /// FIXME: 这里有坑, 服务端返回为空
    /// 若此字段为空, 赋值为0, 对应为未知类型
    if (_traderspi_on_rtn_trade_cb) {
        if (!pTrade->TradeType) {
            pTrade->TradeType = 0;
        }
        if (!pTrade->PriceSource) {
            pTrade->PriceSource = 0;
        }
        if (!pTrade->TradingRole) {
            pTrade->TradingRole = 0;
        }
    }

    RUST_CALL(on_rtn_trade, pTrade);
}

extern "C" {
void rust_traderspi_on_err_rtn_order_insert(
    void *data, CThostFtdcInputOrderField *pInputOrder,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 报单录入错误回报
void
CTPTraderSpi::OnErrRtnOrderInsert(CThostFtdcInputOrderField *pInputOrder,
                                  CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_order_insert, pInputOrder, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_order_action(
    void *data, CThostFtdcOrderActionField *pOrderAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 报单操作错误回报
void
CTPTraderSpi::OnErrRtnOrderAction(CThostFtdcOrderActionField *pOrderAction,
                                  CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_order_action, pOrderAction, pRspInfo);
}

extern "C" {
void rust_traderspi_on_rtn_instrument_status(
    void *data, CThostFtdcInstrumentStatusField *pInstrumentStatus);
}
/// 合约交易状态通知
void
CTPTraderSpi::OnRtnInstrumentStatus(
    CThostFtdcInstrumentStatusField *pInstrumentStatus)
{
    RUST_CALL(on_rtn_instrument_status, pInstrumentStatus);
}

extern "C" {
void rust_traderspi_on_rtn_bulletin(void *data,
                                    CThostFtdcBulletinField *pBulletin);
}
/// 交易所公告通知
void
CTPTraderSpi::OnRtnBulletin(CThostFtdcBulletinField *pBulletin)
{
    RUST_CALL(on_rtn_bulletin, pBulletin);
}

extern "C" {
void rust_traderspi_on_rtn_trading_notice(
    void *data, CThostFtdcTradingNoticeInfoField *pTradingNoticeInfo);
}
/// 交易通知
void
CTPTraderSpi::OnRtnTradingNotice(
    CThostFtdcTradingNoticeInfoField *pTradingNoticeInfo)
{
    RUST_CALL(on_rtn_trading_notice, pTradingNoticeInfo);
}

extern "C" {
void rust_traderspi_on_rtn_error_conditional_order(
    void *data, CThostFtdcErrorConditionalOrderField *pErrorConditionalOrder);
}
/// 提示条件单校验错误
void
CTPTraderSpi::OnRtnErrorConditionalOrder(
    CThostFtdcErrorConditionalOrderField *pErrorConditionalOrder)
{
    RUST_CALL(on_rtn_error_conditional_order, pErrorConditionalOrder);
}

extern "C" {
void rust_traderspi_on_rtn_exec_order(void *data,
                                      CThostFtdcExecOrderField *pExecOrder);
}
/// 执行宣告通知
void
CTPTraderSpi::OnRtnExecOrder(CThostFtdcExecOrderField *pExecOrder)
{
    RUST_CALL(on_rtn_exec_order, pExecOrder);
}

extern "C" {
void rust_traderspi_on_rtn_quote(void *data, CThostFtdcQuoteField *pQuote);
}
/// 报价通知
void
CTPTraderSpi::OnRtnQuote(CThostFtdcQuoteField *pQuote)
{
    RUST_CALL(on_rtn_quote, pQuote);
}

extern "C" {
void rust_traderspi_on_err_rtn_exec_order_insert(
    void *data, CThostFtdcInputExecOrderField *pInputExecOrder,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 执行宣告录入错误回报
void
CTPTraderSpi::OnErrRtnExecOrderInsert(
    CThostFtdcInputExecOrderField *pInputExecOrder,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_exec_order_insert, pInputExecOrder, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_exec_order_action(
    void *data, CThostFtdcExecOrderActionField *pExecOrderAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 执行宣告操作错误回报
void
CTPTraderSpi::OnErrRtnExecOrderAction(
    CThostFtdcExecOrderActionField *pExecOrderAction,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_exec_order_action, pExecOrderAction, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_for_quote_insert(
    void *data, CThostFtdcInputForQuoteField *pInputForQuote,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 询价录入错误回报
void
CTPTraderSpi::OnErrRtnForQuoteInsert(
    CThostFtdcInputForQuoteField *pInputForQuote,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_for_quote_insert, pInputForQuote, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_quote_insert(
    void *data, CThostFtdcInputQuoteField *pInputQuote,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 报价录入错误回报
void
CTPTraderSpi::OnErrRtnQuoteInsert(CThostFtdcInputQuoteField *pInputQuote,
                                  CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_quote_insert, pInputQuote, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_quote_action(
    void *data, CThostFtdcQuoteActionField *pQuoteAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 报价操作错误回报
void
CTPTraderSpi::OnErrRtnQuoteAction(CThostFtdcQuoteActionField *pQuoteAction,
                                  CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_quote_action, pQuoteAction, pRspInfo);
}

extern "C" {
void rust_traderspi_on_rtn_for_quote_rsp(
    void *data, CThostFtdcForQuoteRspField *pForQuoteRsp);
}
/// 询价通知
void
CTPTraderSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp)
{
    RUST_CALL(on_rtn_for_quote_rsp, pForQuoteRsp);
}

extern "C" {
void rust_traderspi_on_rtn_cfmmc_trading_account_token(
    void *data,
    CThostFtdcCFMMCTradingAccountTokenField *pCFMMCTradingAccountToken);
}
/// 保证金监控中心用户令牌
void
CTPTraderSpi::OnRtnCFMMCTradingAccountToken(
    CThostFtdcCFMMCTradingAccountTokenField *pCFMMCTradingAccountToken)
{
    RUST_CALL(on_rtn_cfmmc_trading_account_token, pCFMMCTradingAccountToken);
}

extern "C" {
void rust_traderspi_on_err_rtn_batch_order_action(
    void *data, CThostFtdcBatchOrderActionField *pBatchOrderAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 批量报单操作错误回报
void
CTPTraderSpi::OnErrRtnBatchOrderAction(
    CThostFtdcBatchOrderActionField *pBatchOrderAction,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_batch_order_action, pBatchOrderAction, pRspInfo);
}

extern "C" {
void rust_traderspi_on_rtn_option_self_close(
    void *data, CThostFtdcOptionSelfCloseField *pOptionSelfClose);
}
/// 期权自对冲通知
void
CTPTraderSpi::OnRtnOptionSelfClose(
    CThostFtdcOptionSelfCloseField *pOptionSelfClose)
{
    RUST_CALL(on_rtn_option_self_close, pOptionSelfClose);
}

extern "C" {
void rust_traderspi_on_err_rtn_option_self_close_insert(
    void *data, CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 期权自对冲录入错误回报
void
CTPTraderSpi::OnErrRtnOptionSelfCloseInsert(
    CThostFtdcInputOptionSelfCloseField *pInputOptionSelfClose,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_option_self_close_insert, pInputOptionSelfClose,
              pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_option_self_close_action(
    void *data, CThostFtdcOptionSelfCloseActionField *pOptionSelfCloseAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 期权自对冲操作错误回报
void
CTPTraderSpi::OnErrRtnOptionSelfCloseAction(
    CThostFtdcOptionSelfCloseActionField *pOptionSelfCloseAction,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_option_self_close_action, pOptionSelfCloseAction,
              pRspInfo);
}

extern "C" {
void rust_traderspi_on_rtn_comb_action(void *data,
                                       CThostFtdcCombActionField *pCombAction);
}
/// 申请组合通知
void
CTPTraderSpi::OnRtnCombAction(CThostFtdcCombActionField *pCombAction)
{
    RUST_CALL(on_rtn_comb_action, pCombAction);
}

extern "C" {
void rust_traderspi_on_err_rtn_comb_action_insert(
    void *data, CThostFtdcInputCombActionField *pInputCombAction,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 申请组合录入错误回报
void
CTPTraderSpi::OnErrRtnCombActionInsert(
    CThostFtdcInputCombActionField *pInputCombAction,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_comb_action_insert, pInputCombAction, pRspInfo);
}

extern "C" {
void rust_traderspi_on_rsp_qry_contract_bank(
    void *data, CThostFtdcContractBankField *pContractBank,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询签约银行响应
void
CTPTraderSpi::OnRspQryContractBank(CThostFtdcContractBankField *pContractBank,
                                   CThostFtdcRspInfoField *pRspInfo,
                                   int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_contract_bank, pContractBank, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_parked_order(
    void *data, CThostFtdcParkedOrderField *pParkedOrder,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询预埋单响应
void
CTPTraderSpi::OnRspQryParkedOrder(CThostFtdcParkedOrderField *pParkedOrder,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_parked_order, pParkedOrder, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_parked_order_action(
    void *data, CThostFtdcParkedOrderActionField *pParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询预埋撤单响应
void
CTPTraderSpi::OnRspQryParkedOrderAction(
    CThostFtdcParkedOrderActionField *pParkedOrderAction,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_parked_order_action, pParkedOrderAction, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_trading_notice(
    void *data, CThostFtdcTradingNoticeField *pTradingNotice,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询交易通知响应
void
CTPTraderSpi::OnRspQryTradingNotice(
    CThostFtdcTradingNoticeField *pTradingNotice,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_trading_notice, pTradingNotice, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_broker_trading_params(
    void *data, CThostFtdcBrokerTradingParamsField *pBrokerTradingParams,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询经纪公司交易参数响应
void
CTPTraderSpi::OnRspQryBrokerTradingParams(
    CThostFtdcBrokerTradingParamsField *pBrokerTradingParams,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_broker_trading_params, pBrokerTradingParams, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_qry_broker_trading_algos(
    void *data, CThostFtdcBrokerTradingAlgosField *pBrokerTradingAlgos,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询经纪公司交易算法响应
void
CTPTraderSpi::OnRspQryBrokerTradingAlgos(
    CThostFtdcBrokerTradingAlgosField *pBrokerTradingAlgos,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_broker_trading_algos, pBrokerTradingAlgos, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_query_cfmmc_trading_account_token(
    void *data,
    CThostFtdcQueryCFMMCTradingAccountTokenField
        *pQueryCFMMCTradingAccountToken,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 请求查询监控中心用户令牌
void
CTPTraderSpi::OnRspQueryCFMMCTradingAccountToken(
    CThostFtdcQueryCFMMCTradingAccountTokenField
        *pQueryCFMMCTradingAccountToken,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_query_cfmmc_trading_account_token,
              pQueryCFMMCTradingAccountToken, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rtn_from_bank_to_future_by_bank(
    void *data, CThostFtdcRspTransferField *pRspTransfer);
}
/// 银行发起银行资金转期货通知
void
CTPTraderSpi::OnRtnFromBankToFutureByBank(
    CThostFtdcRspTransferField *pRspTransfer)
{
    RUST_CALL(on_rtn_from_bank_to_future_by_bank, pRspTransfer);
}

extern "C" {
void rust_traderspi_on_rtn_from_future_to_bank_by_bank(
    void *data, CThostFtdcRspTransferField *pRspTransfer);
}
/// 银行发起期货资金转银行通知
void
CTPTraderSpi::OnRtnFromFutureToBankByBank(
    CThostFtdcRspTransferField *pRspTransfer)
{
    RUST_CALL(on_rtn_from_future_to_bank_by_bank, pRspTransfer);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_bank_to_future_by_bank(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 银行发起冲正银行转期货通知
void
CTPTraderSpi::OnRtnRepealFromBankToFutureByBank(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_bank_to_future_by_bank, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_future_to_bank_by_bank(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 银行发起冲正期货转银行通知
void
CTPTraderSpi::OnRtnRepealFromFutureToBankByBank(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_future_to_bank_by_bank, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rtn_from_bank_to_future_by_future(
    void *data, CThostFtdcRspTransferField *pRspTransfer);
}
/// 期货发起银行资金转期货通知
void
CTPTraderSpi::OnRtnFromBankToFutureByFuture(
    CThostFtdcRspTransferField *pRspTransfer)
{
    RUST_CALL(on_rtn_from_bank_to_future_by_future, pRspTransfer);
}

extern "C" {
void rust_traderspi_on_rtn_from_future_to_bank_by_future(
    void *data, CThostFtdcRspTransferField *pRspTransfer);
}
/// 期货发起期货资金转银行通知
void
CTPTraderSpi::OnRtnFromFutureToBankByFuture(
    CThostFtdcRspTransferField *pRspTransfer)
{
    RUST_CALL(on_rtn_from_future_to_bank_by_future, pRspTransfer);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_bank_to_future_by_future_manual(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
void
CTPTraderSpi::OnRtnRepealFromBankToFutureByFutureManual(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_bank_to_future_by_future_manual, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_future_to_bank_by_future_manual(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
void
CTPTraderSpi::OnRtnRepealFromFutureToBankByFutureManual(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_future_to_bank_by_future_manual, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rtn_query_bank_balance_by_future(
    void *data, CThostFtdcNotifyQueryAccountField *pNotifyQueryAccount);
}
/// 期货发起查询银行余额通知
void
CTPTraderSpi::OnRtnQueryBankBalanceByFuture(
    CThostFtdcNotifyQueryAccountField *pNotifyQueryAccount)
{
    RUST_CALL(on_rtn_query_bank_balance_by_future, pNotifyQueryAccount);
}

extern "C" {
void rust_traderspi_on_err_rtn_bank_to_future_by_future(
    void *data, CThostFtdcReqTransferField *pReqTransfer,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 期货发起银行资金转期货错误回报
void
CTPTraderSpi::OnErrRtnBankToFutureByFuture(
    CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_bank_to_future_by_future, pReqTransfer, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_future_to_bank_by_future(
    void *data, CThostFtdcReqTransferField *pReqTransfer,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 期货发起期货资金转银行错误回报
void
CTPTraderSpi::OnErrRtnFutureToBankByFuture(
    CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_future_to_bank_by_future, pReqTransfer, pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_repeal_bank_to_future_by_future_manual(
    void *data, CThostFtdcReqRepealField *pReqRepeal,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 系统运行时期货端手工发起冲正银行转期货错误回报
void
CTPTraderSpi::OnErrRtnRepealBankToFutureByFutureManual(
    CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_repeal_bank_to_future_by_future_manual, pReqRepeal,
              pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_repeal_future_to_bank_by_future_manual(
    void *data, CThostFtdcReqRepealField *pReqRepeal,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 系统运行时期货端手工发起冲正期货转银行错误回报
void
CTPTraderSpi::OnErrRtnRepealFutureToBankByFutureManual(
    CThostFtdcReqRepealField *pReqRepeal, CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_repeal_future_to_bank_by_future_manual, pReqRepeal,
              pRspInfo);
}

extern "C" {
void rust_traderspi_on_err_rtn_query_bank_balance_by_future(
    void *data, CThostFtdcReqQueryAccountField *pReqQueryAccount,
    CThostFtdcRspInfoField *pRspInfo);
}
/// 期货发起查询银行余额错误回报
void
CTPTraderSpi::OnErrRtnQueryBankBalanceByFuture(
    CThostFtdcReqQueryAccountField *pReqQueryAccount,
    CThostFtdcRspInfoField *pRspInfo)
{
    RUST_CALL(on_err_rtn_query_bank_balance_by_future, pReqQueryAccount,
              pRspInfo);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_bank_to_future_by_future(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
void
CTPTraderSpi::OnRtnRepealFromBankToFutureByFuture(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_bank_to_future_by_future, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rtn_repeal_from_future_to_bank_by_future(
    void *data, CThostFtdcRspRepealField *pRspRepeal);
}
/// 期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
void
CTPTraderSpi::OnRtnRepealFromFutureToBankByFuture(
    CThostFtdcRspRepealField *pRspRepeal)
{
    RUST_CALL(on_rtn_repeal_from_future_to_bank_by_future, pRspRepeal);
}

extern "C" {
void rust_traderspi_on_rsp_from_bank_to_future_by_future(
    void *data, CThostFtdcReqTransferField *pReqTransfer,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 期货发起银行资金转期货应答
void
CTPTraderSpi::OnRspFromBankToFutureByFuture(
    CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo,
    int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_from_bank_to_future_by_future, pReqTransfer, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_from_future_to_bank_by_future(
    void *data, CThostFtdcReqTransferField *pReqTransfer,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 期货发起期货资金转银行应答
void
CTPTraderSpi::OnRspFromFutureToBankByFuture(
    CThostFtdcReqTransferField *pReqTransfer, CThostFtdcRspInfoField *pRspInfo,
    int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_from_future_to_bank_by_future, pReqTransfer, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rsp_query_bank_account_money_by_future(
    void *data, CThostFtdcReqQueryAccountField *pReqQueryAccount,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
/// 期货发起查询银行余额应答
void
CTPTraderSpi::OnRspQueryBankAccountMoneyByFuture(
    CThostFtdcReqQueryAccountField *pReqQueryAccount,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_query_bank_account_money_by_future, pReqQueryAccount,
              pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_traderspi_on_rtn_open_account_by_bank(
    void *data, CThostFtdcOpenAccountField *pOpenAccount);
}
/// 银行发起银期开户通知
void
CTPTraderSpi::OnRtnOpenAccountByBank(CThostFtdcOpenAccountField *pOpenAccount)
{
    RUST_CALL(on_rtn_open_account_by_bank, pOpenAccount);
}

extern "C" {
void rust_traderspi_on_rtn_cancel_account_by_bank(
    void *data, CThostFtdcCancelAccountField *pOpenAccount);
}
/// 银行发起银期销户通知
void
CTPTraderSpi::OnRtnCancelAccountByBank(
    CThostFtdcCancelAccountField *pCancelAccount)
{
    RUST_CALL(on_rtn_cancel_account_by_bank, pCancelAccount);
}

extern "C" {
void rust_traderspi_on_rtn_change_account_by_bank(
    void *data, CThostFtdcChangeAccountField *pChangeAccount);
}
/// 银行发起变更银行账号通知
void
CTPTraderSpi::OnRtnChangeAccountByBank(
    CThostFtdcChangeAccountField *pChangeAccount)
{
    RUST_CALL(on_rtn_change_account_by_bank, pChangeAccount);
}

/// 清理
void
CTPTraderSpi::Release()
{
    this->~CTPTraderSpi();
}
