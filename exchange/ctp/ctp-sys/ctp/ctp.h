// CTP CXX WRAPPER
#pragma once

#include <ThostFtdcMdApi.h>
#include <ThostFtdcTraderApi.h>
#include <ThostFtdcUserApiDataType.h>
#include <ThostFtdcUserApiStruct.h>

extern "C" {
/* rust log fns */
void rust_log_trace(const char* msg);
void rust_log_debug(const char* msg);
void rust_log_info(const char* msg);
void rust_log_warn(const char* msg);
void rust_log_error(const char* msg);
};

#define rust_market_cb(fn) void* _mdspi_##fn##_cb
#define rust_trader_cb(fn) void* _traderspi_##fn##_cb

class CTPMarketSpi : public CThostFtdcMdSpi
{
public:
    CTPMarketSpi()
        : _mdspi_on_front_connected_cb(nullptr)
        , _mdspi_on_front_disconnected_cb(nullptr)
        , _mdspi_on_heartbeat_warning_cb(nullptr)
        , _mdspi_on_rsp_error_cb(nullptr)
        , _mdspi_on_rsp_qry_multicast_instrument_cb(nullptr)
        , _mdspi_on_rsp_sub_for_quote_rsp_cb(nullptr)
        , _mdspi_on_rsp_sub_market_data_cb(nullptr)
        , _mdspi_on_rsp_unsub_for_quote_rsp_cb(nullptr)
        , _mdspi_on_rsp_unsub_market_data_cb(nullptr)
        , _mdspi_on_rsp_user_login_cb(nullptr)
        , _mdspi_on_rsp_user_logout_cb(nullptr)
        , _mdspi_on_rtn_depth_market_data_cb(nullptr)
        , _mdspi_on_rtn_for_quote_rsp_cb(nullptr)
    {
    }

    ~CTPMarketSpi()
    {
        rust_log_debug("~CTPMarketSpi() 析构");
        delete _mdspi_on_front_connected_cb;
        delete _mdspi_on_front_disconnected_cb;
        delete _mdspi_on_heartbeat_warning_cb;
        delete _mdspi_on_rsp_error_cb;
        delete _mdspi_on_rsp_qry_multicast_instrument_cb;
        delete _mdspi_on_rsp_sub_for_quote_rsp_cb;
        delete _mdspi_on_rsp_sub_market_data_cb;
        delete _mdspi_on_rsp_unsub_for_quote_rsp_cb;
        delete _mdspi_on_rsp_unsub_market_data_cb;
        delete _mdspi_on_rsp_user_login_cb;
        delete _mdspi_on_rsp_user_logout_cb;
        delete _mdspi_on_rtn_depth_market_data_cb;
        delete _mdspi_on_rtn_for_quote_rsp_cb;
    }

    // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    void OnFrontConnected();

    // 当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
    // @param nReason 错误原因
    //         0x1001 网络读失败
    //         0x1002 网络写失败
    //         0x2001 接收心跳超时
    //         0x2002 发送心跳失败
    //         0x2003 收到错误报文
    void OnFrontDisconnected(int nReason);

    // 心跳超时警告。当长时间未收到报文时，该方法被调用。
    // @param nTimeLapse 距离上次接收报文的时间
    void OnHeartBeatWarning(int nTimeLapse);


    // 登录请求响应
    void OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin,
                        CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                        bool bIsLast);

    // 登出请求响应
    void OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout,
                         CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                         bool bIsLast);

#ifdef v6_3_19
    // 请求查询组播合约响应
    void OnRspQryMulticastInstrument(
        CThostFtdcMulticastInstrumentField* pMulticastInstrument,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);
#endif

    // 错误应答
    void OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                    bool bIsLast);

    // 订阅行情应答
    void OnRspSubMarketData(
        CThostFtdcSpecificInstrumentField* pSpecificInstrument,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 取消订阅行情应答
    void OnRspUnSubMarketData(
        CThostFtdcSpecificInstrumentField* pSpecificInstrument,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 订阅询价应答
    void OnRspSubForQuoteRsp(
        CThostFtdcSpecificInstrumentField* pSpecificInstrument,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 取消订阅询价应答
    void OnRspUnSubForQuoteRsp(
        CThostFtdcSpecificInstrumentField* pSpecificInstrument,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 深度行情通知
    void OnRtnDepthMarketData(CThostFtdcDepthMarketDataField* pDepthMarketData);

    // 询价通知
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp);

    // 清理
    void Release();

private:
    rust_market_cb(on_front_connected);
    rust_market_cb(on_front_disconnected);
    rust_market_cb(on_heartbeat_warning);
    rust_market_cb(on_rsp_error);
    rust_market_cb(on_rsp_qry_multicast_instrument);
    rust_market_cb(on_rsp_sub_for_quote_rsp);
    rust_market_cb(on_rsp_sub_market_data);
    rust_market_cb(on_rsp_unsub_for_quote_rsp);
    rust_market_cb(on_rsp_unsub_market_data);
    rust_market_cb(on_rsp_user_login);
    rust_market_cb(on_rsp_user_logout);
    rust_market_cb(on_rtn_depth_market_data);
    rust_market_cb(on_rtn_for_quote_rsp);
};

class CTPTraderSpi : public CThostFtdcTraderSpi
{
public:
    CTPTraderSpi()
        : _traderspi_on_front_connected_cb(nullptr)
        , _traderspi_on_front_disconnected_cb(nullptr)
        , _traderspi_on_heartbeat_warning_cb(nullptr)
        , _traderspi_on_rsp_authenticate_cb(nullptr)
        , _traderspi_on_rsp_user_login_cb(nullptr)
        , _traderspi_on_rsp_user_logout_cb(nullptr)
        , _traderspi_on_rsp_user_password_update_cb(nullptr)
        , _traderspi_on_rsp_trading_account_password_update_cb(nullptr)
        , _traderspi_on_rsp_user_auth_method_cb(nullptr)
        , _traderspi_on_rsp_gen_user_captcha_cb(nullptr)
        , _traderspi_on_rsp_gen_user_text_cb(nullptr)
        , _traderspi_on_rsp_order_insert_cb(nullptr)
        , _traderspi_on_rsp_parked_order_insert_cb(nullptr)
        , _traderspi_on_rsp_parked_order_action_cb(nullptr)
        , _traderspi_on_rsp_order_action_cb(nullptr)
        , _traderspi_on_rsp_query_max_order_volume_cb(nullptr)
        , _traderspi_on_rsp_settlement_info_confirm_cb(nullptr)
        , _traderspi_on_rsp_remove_parked_order_cb(nullptr)
        , _traderspi_on_rsp_remove_parked_order_action_cb(nullptr)
        , _traderspi_on_rsp_exec_order_insert_cb(nullptr)
        , _traderspi_on_rsp_exec_order_action_cb(nullptr)
        , _traderspi_on_rsp_for_quote_insert_cb(nullptr)
        , _traderspi_on_rsp_quote_insert_cb(nullptr)
        , _traderspi_on_rsp_quote_action_cb(nullptr)
        , _traderspi_on_rsp_batch_order_action_cb(nullptr)
        , _traderspi_on_rsp_option_self_close_insert_cb(nullptr)
        , _traderspi_on_rsp_option_self_close_action_cb(nullptr)
        , _traderspi_on_rsp_comb_action_insert_cb(nullptr)
        , _traderspi_on_rsp_qry_order_cb(nullptr)
        , _traderspi_on_rsp_qry_trade_cb(nullptr)
        , _traderspi_on_rsp_qry_investor_position_cb(nullptr)
        , _traderspi_on_rsp_qry_trading_account_cb(nullptr)
        , _traderspi_on_rsp_qry_investor_cb(nullptr)
        , _traderspi_on_rsp_qry_trading_code_cb(nullptr)
        , _traderspi_on_rsp_qry_instrument_margin_rate_cb(nullptr)
        , _traderspi_on_rsp_qry_instrument_commission_rate_cb(nullptr)
        , _traderspi_on_rsp_qry_exchange_cb(nullptr)
        , _traderspi_on_rsp_qry_product_cb(nullptr)
        , _traderspi_on_rsp_qry_instrument_cb(nullptr)
        , _traderspi_on_rsp_qry_depth_market_data_cb(nullptr)
        , _traderspi_on_rsp_qry_settlement_info_cb(nullptr)
        , _traderspi_on_rsp_qry_transfer_bank_cb(nullptr)
        , _traderspi_on_rsp_qry_investor_position_detail_cb(nullptr)
        , _traderspi_on_rsp_qry_notice_cb(nullptr)
        , _traderspi_on_rsp_error_cb(nullptr)
        , _traderspi_on_rsp_qry_contract_bank_cb(nullptr)
        , _traderspi_on_rsp_qry_parked_order_cb(nullptr)
        , _traderspi_on_rsp_qry_parked_order_action_cb(nullptr)
        , _traderspi_on_rsp_qry_trading_notice_cb(nullptr)
        , _traderspi_on_rsp_qry_broker_trading_params_cb(nullptr)
        , _traderspi_on_rsp_qry_broker_trading_algos_cb(nullptr)
        , _traderspi_on_rsp_query_cfmmc_trading_account_token_cb(nullptr)
        , _traderspi_on_rsp_from_bank_to_future_by_future_cb(nullptr)
        , _traderspi_on_rsp_from_future_to_bank_by_future_cb(nullptr)
        , _traderspi_on_rsp_query_bank_account_money_by_future_cb(nullptr)

        , _traderspi_on_rtn_order_cb(nullptr)
        , _traderspi_on_rtn_exec_order_cb(nullptr)
        , _traderspi_on_rtn_trade_cb(nullptr)
        , _traderspi_on_rtn_quote_cb(nullptr)
        , _traderspi_on_rtn_instrument_status_cb(nullptr)
        , _traderspi_on_rtn_bulletin_cb(nullptr)
        , _traderspi_on_rtn_trading_notice_cb(nullptr)
        , _traderspi_on_rtn_error_conditional_order_cb(nullptr)
        , _traderspi_on_rtn_for_quote_rsp_cb(nullptr)
        , _traderspi_on_rtn_cfmmc_trading_account_token_cb(nullptr)
        , _traderspi_on_rtn_option_self_close_cb(nullptr)
        , _traderspi_on_rtn_comb_action_cb(nullptr)
        , _traderspi_on_rtn_from_bank_to_future_by_bank_cb(nullptr)
        , _traderspi_on_rtn_from_future_to_bank_by_bank_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_bank_to_future_by_bank_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_future_to_bank_by_bank_cb(nullptr)
        , _traderspi_on_rtn_from_bank_to_future_by_future_cb(nullptr)
        , _traderspi_on_rtn_from_future_to_bank_by_future_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_bank_to_future_by_future_manual_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_future_to_bank_by_future_manual_cb(nullptr)
        , _traderspi_on_rtn_query_bank_balance_by_future_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_bank_to_future_by_future_cb(nullptr)
        , _traderspi_on_rtn_repeal_from_future_to_bank_by_future_cb(nullptr)
        , _traderspi_on_rtn_open_account_by_bank_cb(nullptr)
        , _traderspi_on_rtn_cancel_account_by_bank_cb(nullptr)
        , _traderspi_on_rtn_change_account_by_bank_cb(nullptr)

        , _traderspi_on_err_rtn_order_insert_cb(nullptr)
        , _traderspi_on_err_rtn_exec_order_insert_cb(nullptr)
        , _traderspi_on_err_rtn_exec_order_action_cb(nullptr)
        , _traderspi_on_err_rtn_order_action_cb(nullptr)
        , _traderspi_on_err_rtn_quote_insert_cb(nullptr)
        , _traderspi_on_err_rtn_quote_action_cb(nullptr)
        , _traderspi_on_err_rtn_for_quote_insert_cb(nullptr)
        , _traderspi_on_err_rtn_batch_order_action_cb(nullptr)
        , _traderspi_on_err_rtn_option_self_close_insert_cb(nullptr)
        , _traderspi_on_err_rtn_option_self_close_action_cb(nullptr)
        , _traderspi_on_err_rtn_comb_action_insert_cb(nullptr)
        , _traderspi_on_err_rtn_bank_to_future_by_future_cb(nullptr)
        , _traderspi_on_err_rtn_future_to_bank_by_future_cb(nullptr)
        , _traderspi_on_err_rtn_repeal_bank_to_future_by_future_manual_cb(nullptr)
        , _traderspi_on_err_rtn_repeal_future_to_bank_by_future_manual_cb(nullptr)
        , _traderspi_on_err_rtn_query_bank_balance_by_future_cb(nullptr)
    {
    }

    ~CTPTraderSpi()
    {
        rust_log_debug("~CTPTraderSpi() 析构");
        delete _traderspi_on_front_connected_cb;
        delete _traderspi_on_front_disconnected_cb;
        delete _traderspi_on_heartbeat_warning_cb;

        delete _traderspi_on_rsp_authenticate_cb;
        delete _traderspi_on_rsp_user_login_cb;
        delete _traderspi_on_rsp_user_logout_cb;
        delete _traderspi_on_rsp_user_password_update_cb;
        delete _traderspi_on_rsp_trading_account_password_update_cb;
        delete _traderspi_on_rsp_user_auth_method_cb;
        delete _traderspi_on_rsp_gen_user_captcha_cb;
        delete _traderspi_on_rsp_gen_user_text_cb;
        delete _traderspi_on_rsp_order_insert_cb;
        delete _traderspi_on_rsp_parked_order_insert_cb;
        delete _traderspi_on_rsp_parked_order_action_cb;
        delete _traderspi_on_rsp_order_action_cb;
        delete _traderspi_on_rsp_query_max_order_volume_cb;
        delete _traderspi_on_rsp_settlement_info_confirm_cb;
        delete _traderspi_on_rsp_remove_parked_order_cb;
        delete _traderspi_on_rsp_remove_parked_order_action_cb;
        delete _traderspi_on_rsp_exec_order_insert_cb;
        delete _traderspi_on_rsp_exec_order_action_cb;
        delete _traderspi_on_rsp_for_quote_insert_cb;
        delete _traderspi_on_rsp_quote_insert_cb;
        delete _traderspi_on_rsp_quote_action_cb;
        delete _traderspi_on_rsp_batch_order_action_cb;
        delete _traderspi_on_rsp_option_self_close_insert_cb;
        delete _traderspi_on_rsp_option_self_close_action_cb;
        delete _traderspi_on_rsp_comb_action_insert_cb;
        delete _traderspi_on_rsp_qry_order_cb;
        delete _traderspi_on_rsp_qry_trade_cb;
        delete _traderspi_on_rsp_qry_investor_position_cb;
        delete _traderspi_on_rsp_qry_trading_account_cb;
        delete _traderspi_on_rsp_qry_investor_cb;
        delete _traderspi_on_rsp_qry_trading_code_cb;
        delete _traderspi_on_rsp_qry_instrument_margin_rate_cb;
        delete _traderspi_on_rsp_qry_instrument_commission_rate_cb;
        delete _traderspi_on_rsp_qry_exchange_cb;
        delete _traderspi_on_rsp_qry_product_cb;
        delete _traderspi_on_rsp_qry_instrument_cb;
        delete _traderspi_on_rsp_qry_depth_market_data_cb;
        delete _traderspi_on_rsp_qry_settlement_info_cb;
        delete _traderspi_on_rsp_qry_transfer_bank_cb;
        delete _traderspi_on_rsp_qry_investor_position_detail_cb;
        delete _traderspi_on_rsp_qry_notice_cb;
        delete _traderspi_on_rsp_error_cb;
        delete _traderspi_on_rsp_qry_contract_bank_cb;
        delete _traderspi_on_rsp_qry_parked_order_cb;
        delete _traderspi_on_rsp_qry_parked_order_action_cb;
        delete _traderspi_on_rsp_qry_trading_notice_cb;
        delete _traderspi_on_rsp_qry_broker_trading_params_cb;
        delete _traderspi_on_rsp_qry_broker_trading_algos_cb;
        delete _traderspi_on_rsp_query_cfmmc_trading_account_token_cb;
        delete _traderspi_on_rsp_from_bank_to_future_by_future_cb;
        delete _traderspi_on_rsp_from_future_to_bank_by_future_cb;
        delete _traderspi_on_rsp_query_bank_account_money_by_future_cb;

        delete _traderspi_on_rtn_order_cb;
        delete _traderspi_on_rtn_exec_order_cb;
        delete _traderspi_on_rtn_trade_cb;
        delete _traderspi_on_rtn_quote_cb;
        delete _traderspi_on_rtn_instrument_status_cb;
        delete _traderspi_on_rtn_bulletin_cb;
        delete _traderspi_on_rtn_trading_notice_cb;
        delete _traderspi_on_rtn_error_conditional_order_cb;
        delete _traderspi_on_rtn_for_quote_rsp_cb;
        delete _traderspi_on_rtn_cfmmc_trading_account_token_cb;
        delete _traderspi_on_rtn_option_self_close_cb;
        delete _traderspi_on_rtn_comb_action_cb;
        delete _traderspi_on_rtn_from_bank_to_future_by_bank_cb;
        delete _traderspi_on_rtn_from_future_to_bank_by_bank_cb;
        delete _traderspi_on_rtn_repeal_from_bank_to_future_by_bank_cb;
        delete _traderspi_on_rtn_repeal_from_future_to_bank_by_bank_cb;
        delete _traderspi_on_rtn_from_bank_to_future_by_future_cb;
        delete _traderspi_on_rtn_from_future_to_bank_by_future_cb;
        delete _traderspi_on_rtn_repeal_from_bank_to_future_by_future_manual_cb;
        delete _traderspi_on_rtn_repeal_from_future_to_bank_by_future_manual_cb;
        delete _traderspi_on_rtn_query_bank_balance_by_future_cb;
        delete _traderspi_on_rtn_repeal_from_bank_to_future_by_future_cb;
        delete _traderspi_on_rtn_repeal_from_future_to_bank_by_future_cb;
        delete _traderspi_on_rtn_open_account_by_bank_cb;
        delete _traderspi_on_rtn_cancel_account_by_bank_cb;
        delete _traderspi_on_rtn_change_account_by_bank_cb;

        delete _traderspi_on_err_rtn_order_insert_cb;
        delete _traderspi_on_err_rtn_exec_order_insert_cb;
        delete _traderspi_on_err_rtn_exec_order_action_cb;
        delete _traderspi_on_err_rtn_order_action_cb;
        delete _traderspi_on_err_rtn_quote_insert_cb;
        delete _traderspi_on_err_rtn_quote_action_cb;
        delete _traderspi_on_err_rtn_for_quote_insert_cb;
        delete _traderspi_on_err_rtn_batch_order_action_cb;
        delete _traderspi_on_err_rtn_option_self_close_insert_cb;
        delete _traderspi_on_err_rtn_option_self_close_action_cb;
        delete _traderspi_on_err_rtn_comb_action_insert_cb;
        delete _traderspi_on_err_rtn_bank_to_future_by_future_cb;
        delete _traderspi_on_err_rtn_future_to_bank_by_future_cb;
        delete _traderspi_on_err_rtn_repeal_bank_to_future_by_future_manual_cb;
        delete _traderspi_on_err_rtn_repeal_future_to_bank_by_future_manual_cb;
        delete _traderspi_on_err_rtn_query_bank_balance_by_future_cb;
    }

    // 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
    void OnFrontConnected();

    // 当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
    // @param nReason 错误原因
    //         0x1001 网络读失败
    //         0x1002 网络写失败
    //         0x2001 接收心跳超时
    //         0x2002 发送心跳失败
    //         0x2003 收到错误报文
    void OnFrontDisconnected(int nReason);

    // 心跳超时警告。当长时间未收到报文时，该方法被调用。
    void OnHeartBeatWarning(int nTimeLapse);

    // 客户端认证响应
    void OnRspAuthenticate(
        CThostFtdcRspAuthenticateField* pRspAuthenticateField,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);


    // 登录请求响应
    void OnRspUserLogin(CThostFtdcRspUserLoginField* pRspUserLogin,
                        CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                        bool bIsLast);

    // 登出请求响应
    void OnRspUserLogout(CThostFtdcUserLogoutField* pUserLogout,
                         CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                         bool bIsLast);

    // 用户口令更新请求响应
    void OnRspUserPasswordUpdate(
        CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 资金账户口令更新请求响应
    void OnRspTradingAccountPasswordUpdate(
        CThostFtdcTradingAccountPasswordUpdateField*
            pTradingAccountPasswordUpdate,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 查询用户当前支持的认证模式的回复
    void OnRspUserAuthMethod(
        CThostFtdcRspUserAuthMethodField* pRspUserAuthMethod,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 获取图形验证码请求的回复
    void OnRspGenUserCaptcha(
        CThostFtdcRspGenUserCaptchaField* pRspGenUserCaptcha,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 获取短信验证码请求的回复
    void OnRspGenUserText(CThostFtdcRspGenUserTextField* pRspGenUserText,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 报单录入请求响应
    void OnRspOrderInsert(CThostFtdcInputOrderField* pInputOrder,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 预埋单录入请求响应
    void OnRspParkedOrderInsert(CThostFtdcParkedOrderField* pParkedOrder,
                                CThostFtdcRspInfoField* pRspInfo,
                                int nRequestID, bool bIsLast);

    // 预埋撤单录入请求响应
    void OnRspParkedOrderAction(
        CThostFtdcParkedOrderActionField* pParkedOrderAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);


    // 报单操作请求响应
    void OnRspOrderAction(CThostFtdcInputOrderActionField* pInputOrderAction,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

#ifdef v6_7_0
    // 查询最大报单数量响应
    void OnRspQryMaxOrderVolume(
        CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);
#else
    // 查询最大报单数量响应
    void OnRspQueryMaxOrderVolume(
        CThostFtdcQueryMaxOrderVolumeField* pQueryMaxOrderVolume,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);
#endif

    // 投资者结算结果确认响应
    void OnRspSettlementInfoConfirm(
        CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 删除预埋单响应
    void OnRspRemoveParkedOrder(
        CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 删除预埋撤单响应
    void OnRspRemoveParkedOrderAction(
        CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 执行宣告录入请求响应
    void OnRspExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder,
                              CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                              bool bIsLast);

    // 执行宣告操作请求响应
    void OnRspExecOrderAction(
        CThostFtdcInputExecOrderActionField* pInputExecOrderAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 询价录入请求响应
    void OnRspForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote,
                             CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                             bool bIsLast);

    // 报价录入请求响应
    void OnRspQuoteInsert(CThostFtdcInputQuoteField* pInputQuote,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 报价操作请求响应
    void OnRspQuoteAction(CThostFtdcInputQuoteActionField* pInputQuoteAction,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 批量报单操作请求响应
    void OnRspBatchOrderAction(
        CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 期权自对冲录入请求响应
    void OnRspOptionSelfCloseInsert(
        CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 期权自对冲操作请求响应
    void OnRspOptionSelfCloseAction(
        CThostFtdcInputOptionSelfCloseActionField* pInputOptionSelfCloseAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 申请组合录入请求响应
    void OnRspCombActionInsert(CThostFtdcInputCombActionField* pInputCombAction,
                               CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                               bool bIsLast);

    // 请求查询报单响应
    void OnRspQryOrder(CThostFtdcOrderField* pOrder,
                       CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                       bool bIsLast);

    // 请求查询成交响应
    void OnRspQryTrade(CThostFtdcTradeField* pTrade,
                       CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                       bool bIsLast);

    // 请求查询投资者持仓响应
    void OnRspQryInvestorPosition(
        CThostFtdcInvestorPositionField* pInvestorPosition,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询资金账户响应
    void OnRspQryTradingAccount(CThostFtdcTradingAccountField* pTradingAccount,
                                CThostFtdcRspInfoField* pRspInfo,
                                int nRequestID, bool bIsLast);

    // 请求查询投资者响应
    void OnRspQryInvestor(CThostFtdcInvestorField* pInvestor,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 请求查询交易编码响应
    void OnRspQryTradingCode(CThostFtdcTradingCodeField* pTradingCode,
                             CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                             bool bIsLast);

    // 请求查询合约保证金率响应
    void OnRspQryInstrumentMarginRate(
        CThostFtdcInstrumentMarginRateField* pInstrumentMarginRate,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询合约手续费率响应
    void OnRspQryInstrumentCommissionRate(
        CThostFtdcInstrumentCommissionRateField* pInstrumentCommissionRate,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询交易所响应
    void OnRspQryExchange(CThostFtdcExchangeField* pExchange,
                          CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                          bool bIsLast);

    // 请求查询产品响应
    void OnRspQryProduct(CThostFtdcProductField* pProduct,
                         CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                         bool bIsLast);

    // 请求查询合约响应
    void OnRspQryInstrument(CThostFtdcInstrumentField* pInstrument,
                            CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                            bool bIsLast);
    // 请求查询行情响应
    void OnRspQryDepthMarketData(
        CThostFtdcDepthMarketDataField* pDepthMarketData,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询投资者结算结果响应
    void OnRspQrySettlementInfo(CThostFtdcSettlementInfoField* pSettlementInfo,
                                CThostFtdcRspInfoField* pRspInfo,
                                int nRequestID, bool bIsLast);

    // 请求查询转帐银行响应
    void OnRspQryTransferBank(CThostFtdcTransferBankField* pTransferBank,
                              CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                              bool bIsLast);

    // 请求查询投资者持仓明细响应
    void OnRspQryInvestorPositionDetail(
        CThostFtdcInvestorPositionDetailField* pInvestorPositionDetail,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询客户通知响应
    void OnRspQryNotice(CThostFtdcNoticeField* pNotice,
                        CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                        bool bIsLast);

    // 错误应答
    void OnRspError(CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                    bool bIsLast);

    // 报单通知
    void OnRtnOrder(CThostFtdcOrderField* pOrder);

    // 成交通知
    void OnRtnTrade(CThostFtdcTradeField* pTrade);

    // 报单录入错误回报
    void OnErrRtnOrderInsert(CThostFtdcInputOrderField* pInputOrder,
                             CThostFtdcRspInfoField* pRspInfo);

    // 报单操作错误回报
    void OnErrRtnOrderAction(CThostFtdcOrderActionField* pOrderAction,
                             CThostFtdcRspInfoField* pRspInfo);

    // 合约交易状态通知
    void OnRtnInstrumentStatus(
        CThostFtdcInstrumentStatusField* pInstrumentStatus);

    // 交易所公告通知
    void OnRtnBulletin(CThostFtdcBulletinField* pBulletin);

    // 交易通知
    void OnRtnTradingNotice(
        CThostFtdcTradingNoticeInfoField* pTradingNoticeInfo);

    // 提示条件单校验错误
    void OnRtnErrorConditionalOrder(
        CThostFtdcErrorConditionalOrderField* pErrorConditionalOrder);

    // 执行宣告通知
    void OnRtnExecOrder(CThostFtdcExecOrderField* pExecOrder);

    // 执行宣告录入错误回报
    void OnErrRtnExecOrderInsert(CThostFtdcInputExecOrderField* pInputExecOrder,
                                 CThostFtdcRspInfoField* pRspInfo);

    // 执行宣告操作错误回报
    void OnErrRtnExecOrderAction(
        CThostFtdcExecOrderActionField* pExecOrderAction,
        CThostFtdcRspInfoField* pRspInfo);

    // 询价录入错误回报
    void OnErrRtnForQuoteInsert(CThostFtdcInputForQuoteField* pInputForQuote,
                                CThostFtdcRspInfoField* pRspInfo);

    // 报价通知
    void OnRtnQuote(CThostFtdcQuoteField* pQuote);

    // 报价录入错误回报
    void OnErrRtnQuoteInsert(CThostFtdcInputQuoteField* pInputQuote,
                             CThostFtdcRspInfoField* pRspInfo);

    // 报价操作错误回报
    void OnErrRtnQuoteAction(CThostFtdcQuoteActionField* pQuoteAction,
                             CThostFtdcRspInfoField* pRspInfo);

    // 询价通知
    void OnRtnForQuoteRsp(CThostFtdcForQuoteRspField* pForQuoteRsp);

    // 保证金监控中心用户令牌
    void OnRtnCFMMCTradingAccountToken(
        CThostFtdcCFMMCTradingAccountTokenField* pCFMMCTradingAccountToken);

    // 批量报单操作错误回报
    void OnErrRtnBatchOrderAction(
        CThostFtdcBatchOrderActionField* pBatchOrderAction,
        CThostFtdcRspInfoField* pRspInfo);

    // 期权自对冲通知
    void OnRtnOptionSelfClose(CThostFtdcOptionSelfCloseField* pOptionSelfClose);

    // 期权自对冲录入错误回报
    void OnErrRtnOptionSelfCloseInsert(
        CThostFtdcInputOptionSelfCloseField* pInputOptionSelfClose,
        CThostFtdcRspInfoField* pRspInfo);

    // 期权自对冲操作错误回报
    void OnErrRtnOptionSelfCloseAction(
        CThostFtdcOptionSelfCloseActionField* pOptionSelfCloseAction,
        CThostFtdcRspInfoField* pRspInfo);

    // 申请组合通知
    void OnRtnCombAction(CThostFtdcCombActionField* pCombAction);

    // 申请组合录入错误回报
    void OnErrRtnCombActionInsert(
        CThostFtdcInputCombActionField* pInputCombAction,
        CThostFtdcRspInfoField* pRspInfo);

    // 请求查询签约银行响应
    void OnRspQryContractBank(CThostFtdcContractBankField* pContractBank,
                              CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                              bool bIsLast);

    // 请求查询预埋单响应
    void OnRspQryParkedOrder(CThostFtdcParkedOrderField* pParkedOrder,
                             CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                             bool bIsLast);

    // 请求查询预埋撤单响应
    void OnRspQryParkedOrderAction(
        CThostFtdcParkedOrderActionField* pParkedOrderAction,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询交易通知响应
    void OnRspQryTradingNotice(CThostFtdcTradingNoticeField* pTradingNotice,
                               CThostFtdcRspInfoField* pRspInfo, int nRequestID,
                               bool bIsLast);

    // 请求查询经纪公司交易参数响应
    void OnRspQryBrokerTradingParams(
        CThostFtdcBrokerTradingParamsField* pBrokerTradingParams,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询经纪公司交易算法响应
    void OnRspQryBrokerTradingAlgos(
        CThostFtdcBrokerTradingAlgosField* pBrokerTradingAlgos,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 请求查询监控中心用户令牌
    void OnRspQueryCFMMCTradingAccountToken(
        CThostFtdcQueryCFMMCTradingAccountTokenField*
            pQueryCFMMCTradingAccountToken,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 银行发起银行资金转期货通知
    void OnRtnFromBankToFutureByBank(CThostFtdcRspTransferField* pRspTransfer);

    // 银行发起期货资金转银行通知
    void OnRtnFromFutureToBankByBank(CThostFtdcRspTransferField* pRspTransfer);

    // 银行发起冲正银行转期货通知
    void OnRtnRepealFromBankToFutureByBank(
        CThostFtdcRspRepealField* pRspRepeal);

    // 银行发起冲正期货转银行通知
    void OnRtnRepealFromFutureToBankByBank(
        CThostFtdcRspRepealField* pRspRepeal);

    // 期货发起银行资金转期货通知
    void OnRtnFromBankToFutureByFuture(
        CThostFtdcRspTransferField* pRspTransfer);

    // 期货发起期货资金转银行通知
    void OnRtnFromFutureToBankByFuture(
        CThostFtdcRspTransferField* pRspTransfer);

    // 系统运行时期货端手工发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromBankToFutureByFutureManual(
        CThostFtdcRspRepealField* pRspRepeal);

    // 系统运行时期货端手工发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromFutureToBankByFutureManual(
        CThostFtdcRspRepealField* pRspRepeal);

    // 期货发起查询银行余额通知
    void OnRtnQueryBankBalanceByFuture(
        CThostFtdcNotifyQueryAccountField* pNotifyQueryAccount);

    // 期货发起银行资金转期货错误回报
    void OnErrRtnBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer,
                                      CThostFtdcRspInfoField* pRspInfo);

    // 期货发起期货资金转银行错误回报
    void OnErrRtnFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer,
                                      CThostFtdcRspInfoField* pRspInfo);

    // 系统运行时期货端手工发起冲正银行转期货错误回报
    void OnErrRtnRepealBankToFutureByFutureManual(
        CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo);

    // 系统运行时期货端手工发起冲正期货转银行错误回报
    void OnErrRtnRepealFutureToBankByFutureManual(
        CThostFtdcReqRepealField* pReqRepeal, CThostFtdcRspInfoField* pRspInfo);

    // 期货发起查询银行余额错误回报
    void OnErrRtnQueryBankBalanceByFuture(
        CThostFtdcReqQueryAccountField* pReqQueryAccount,
        CThostFtdcRspInfoField* pRspInfo);

    // 期货发起冲正银行转期货请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromBankToFutureByFuture(
        CThostFtdcRspRepealField* pRspRepeal);

    // 期货发起冲正期货转银行请求，银行处理完毕后报盘发回的通知
    void OnRtnRepealFromFutureToBankByFuture(
        CThostFtdcRspRepealField* pRspRepeal);

    // 期货发起银行资金转期货应答
    void OnRspFromBankToFutureByFuture(CThostFtdcReqTransferField* pReqTransfer,
                                       CThostFtdcRspInfoField* pRspInfo,
                                       int nRequestID, bool bIsLast);

    // 期货发起期货资金转银行应答
    void OnRspFromFutureToBankByFuture(CThostFtdcReqTransferField* pReqTransfer,
                                       CThostFtdcRspInfoField* pRspInfo,
                                       int nRequestID, bool bIsLast);

    // 期货发起查询银行余额应答
    void OnRspQueryBankAccountMoneyByFuture(
        CThostFtdcReqQueryAccountField* pReqQueryAccount,
        CThostFtdcRspInfoField* pRspInfo, int nRequestID, bool bIsLast);

    // 银行发起银期开户通知
    void OnRtnOpenAccountByBank(CThostFtdcOpenAccountField* pOpenAccount);

    // 银行发起银期销户通知
    void OnRtnCancelAccountByBank(CThostFtdcCancelAccountField* pCancelAccount);

    // 银行发起变更银行账号通知
    void OnRtnChangeAccountByBank(CThostFtdcChangeAccountField* pChangeAccount);

    // 清理
    void Release();

private:
    rust_trader_cb(on_front_connected);
    rust_trader_cb(on_front_disconnected);
    rust_trader_cb(on_heartbeat_warning);

    rust_trader_cb(on_rsp_authenticate);
    rust_trader_cb(on_rsp_user_login);
    rust_trader_cb(on_rsp_user_logout);
    rust_trader_cb(on_rsp_user_password_update);
    rust_trader_cb(on_rsp_trading_account_password_update);
    rust_trader_cb(on_rsp_user_auth_method);
    rust_trader_cb(on_rsp_gen_user_captcha);
    rust_trader_cb(on_rsp_gen_user_text);

    rust_trader_cb(on_rsp_order_insert);
    rust_trader_cb(on_rsp_parked_order_insert);
    rust_trader_cb(on_rsp_parked_order_action);
    rust_trader_cb(on_rsp_order_action);
    rust_trader_cb(on_rsp_query_max_order_volume);

    rust_trader_cb(on_rsp_settlement_info_confirm);
    rust_trader_cb(on_rsp_remove_parked_order);
    rust_trader_cb(on_rsp_remove_parked_order_action);
    rust_trader_cb(on_rsp_exec_order_insert);
    rust_trader_cb(on_rsp_exec_order_action);
    rust_trader_cb(on_rsp_for_quote_insert);
    rust_trader_cb(on_rsp_quote_insert);
    rust_trader_cb(on_rsp_quote_action);
    rust_trader_cb(on_rsp_batch_order_action);
    rust_trader_cb(on_rsp_option_self_close_insert);
    rust_trader_cb(on_rsp_option_self_close_action);
    rust_trader_cb(on_rsp_comb_action_insert);
    rust_trader_cb(on_rsp_qry_order);
    rust_trader_cb(on_rsp_qry_trade);

    rust_trader_cb(on_rsp_qry_investor_position);
    rust_trader_cb(on_rsp_qry_trading_account);
    rust_trader_cb(on_rsp_qry_investor);
    rust_trader_cb(on_rsp_qry_trading_code);

    rust_trader_cb(on_rsp_qry_instrument_margin_rate);
    rust_trader_cb(on_rsp_qry_instrument_commission_rate);
    rust_trader_cb(on_rsp_qry_exchange);
    rust_trader_cb(on_rsp_qry_product);
    rust_trader_cb(on_rsp_qry_instrument);

    rust_trader_cb(on_rsp_qry_depth_market_data);
    rust_trader_cb(on_rsp_qry_settlement_info);
    rust_trader_cb(on_rsp_qry_transfer_bank);
    rust_trader_cb(on_rsp_qry_investor_position_detail);
    rust_trader_cb(on_rsp_qry_notice);
    rust_trader_cb(on_rsp_error);
    rust_trader_cb(on_rsp_qry_contract_bank);
    rust_trader_cb(on_rsp_qry_parked_order);
    rust_trader_cb(on_rsp_qry_parked_order_action);
    rust_trader_cb(on_rsp_qry_trading_notice);
    rust_trader_cb(on_rsp_qry_broker_trading_params);
    rust_trader_cb(on_rsp_qry_broker_trading_algos);
    rust_trader_cb(on_rsp_query_cfmmc_trading_account_token);
    rust_trader_cb(on_rsp_from_bank_to_future_by_future);
    rust_trader_cb(on_rsp_from_future_to_bank_by_future);
    rust_trader_cb(on_rsp_query_bank_account_money_by_future);

    rust_trader_cb(on_rtn_order);
    rust_trader_cb(on_rtn_exec_order);
    rust_trader_cb(on_rtn_trade);
    rust_trader_cb(on_rtn_quote);
    rust_trader_cb(on_rtn_instrument_status);
    rust_trader_cb(on_rtn_bulletin);
    rust_trader_cb(on_rtn_trading_notice);
    rust_trader_cb(on_rtn_error_conditional_order);
    rust_trader_cb(on_rtn_for_quote_rsp);
    rust_trader_cb(on_rtn_cfmmc_trading_account_token);
    rust_trader_cb(on_rtn_option_self_close);
    rust_trader_cb(on_rtn_comb_action);
    rust_trader_cb(on_rtn_from_bank_to_future_by_bank);
    rust_trader_cb(on_rtn_from_future_to_bank_by_bank);
    rust_trader_cb(on_rtn_repeal_from_bank_to_future_by_bank);
    rust_trader_cb(on_rtn_repeal_from_future_to_bank_by_bank);
    rust_trader_cb(on_rtn_from_bank_to_future_by_future);
    rust_trader_cb(on_rtn_from_future_to_bank_by_future);
    rust_trader_cb(on_rtn_repeal_from_bank_to_future_by_future_manual);
    rust_trader_cb(on_rtn_repeal_from_future_to_bank_by_future_manual);
    rust_trader_cb(on_rtn_query_bank_balance_by_future);
    rust_trader_cb(on_rtn_repeal_from_bank_to_future_by_future);
    rust_trader_cb(on_rtn_repeal_from_future_to_bank_by_future);
    rust_trader_cb(on_rtn_open_account_by_bank);
    rust_trader_cb(on_rtn_cancel_account_by_bank);
    rust_trader_cb(on_rtn_change_account_by_bank);

    rust_trader_cb(on_err_rtn_order_insert);
    rust_trader_cb(on_err_rtn_order_action);
    rust_trader_cb(on_err_rtn_exec_order_insert);
    rust_trader_cb(on_err_rtn_exec_order_action);
    rust_trader_cb(on_err_rtn_quote_insert);
    rust_trader_cb(on_err_rtn_quote_action);
    rust_trader_cb(on_err_rtn_for_quote_insert);
    rust_trader_cb(on_err_rtn_batch_order_action);
    rust_trader_cb(on_err_rtn_option_self_close_insert);
    rust_trader_cb(on_err_rtn_option_self_close_action);
    rust_trader_cb(on_err_rtn_comb_action_insert);
    rust_trader_cb(on_err_rtn_bank_to_future_by_future);
    rust_trader_cb(on_err_rtn_future_to_bank_by_future);
    rust_trader_cb(on_err_rtn_repeal_bank_to_future_by_future_manual);
    rust_trader_cb(on_err_rtn_repeal_future_to_bank_by_future_manual);
    rust_trader_cb(on_err_rtn_query_bank_balance_by_future);
};

extern "C" {
/* MdApi */
// 删除接口对象本身
// @remark 不再使用本接口对象时,调用该函数删除接口对象
void MdApi_Release(CThostFtdcMdApi* p);

// 初始化
// @remark 初始化运行环境,只有调用后,接口才开始工作
void MdApi_Init(CThostFtdcMdApi* p);

// 等待接口线程结束运行
// @return 线程退出代码
int MdApi_Join(CThostFtdcMdApi* p);

// 获取当前交易日
// @retrun 获取到的交易日
// @remark 只有登录成功后,才能得到正确的交易日
const char* MdApi_GetTradingDay(CThostFtdcMdApi* p);

// 注册前置机网络地址
// @param pszFrontAddress：前置机网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
void MdApi_RegisterFront(CThostFtdcMdApi* p, char* pszFrontAddress);

// 注册名字服务器网络地址
// @param pszNsAddress：名字服务器网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
// @remark RegisterNameServer优先于RegisterFront
void MdApi_RegisterNameServer(CThostFtdcMdApi* p, char* pszNsAddress);

// 注册名字服务器用户信息
// @param pFensUserInfo：用户信息。
void MdApi_RegisterFensUserInfo(CThostFtdcMdApi* p,
                                CThostFtdcFensUserInfoField* pi);

// 注册回调接口
// @param pSpi 派生自回调接口类的实例
void MdApi_RegisterSpi(CThostFtdcMdApi* p, CTPMarketSpi* pSpi);

// 订阅行情。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int MdApi_SubscribeMarketData(CThostFtdcMdApi* p, char* ppInstrumentID[],
                              int nCount);

// 退订行情。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int MdApi_UnSubscribeMarketData(CThostFtdcMdApi* p, char* ppInstrumentID[],
                                int nCount);

// 订阅询价。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int MdApi_SubscribeForQuoteRsp(CThostFtdcMdApi* p, char* ppInstrumentID[],
                               int nCount);

// 退订询价。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int MdApi_UnSubscribeForQuoteRsp(CThostFtdcMdApi* p, char* ppInstrumentID[],
                                 int nCount);

// 用户登录请求
int MdApi_ReqUserLogin(CThostFtdcMdApi* p,
                       CThostFtdcReqUserLoginField* pReqUserLoginField,
                       int nRequestID);

// 登出请求
int MdApi_ReqUserLogout(CThostFtdcMdApi* p,
                        CThostFtdcUserLogoutField* pUserLogout, int nRequestID);

#ifdef v6_3_19
// 请求查询组播合约
int MdApi_ReqQryMulticastInstrument(
    CThostFtdcMdApi* p,
    CThostFtdcQryMulticastInstrumentField* pQryMulticastInstrument,
    int nRequestID);
#endif

/* TraderApi */

// 删除接口对象本身
// @remark 不再使用本接口对象时,调用该函数删除接口对象
void TraderApi_Release(CThostFtdcTraderApi* p);

// 初始化
// @remark 初始化运行环境,只有调用后,接口才开始工作
void TraderApi_Init(CThostFtdcTraderApi* p);

// 等待接口线程结束运行
// @return 线程退出代码
int TraderApi_Join(CThostFtdcTraderApi* p);

// 获取当前交易日
// @retrun 获取到的交易日
// @remark 只有登录成功后,才能得到正确的交易日
const char* TraderApi_GetTradingDay(CThostFtdcTraderApi* p);

// 注册前置机网络地址
// @param pszFrontAddress：前置机网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
void TraderApi_RegisterFront(CThostFtdcTraderApi* p, char* pszFrontAddress);

// 注册名字服务器网络地址
// @param pszNsAddress：名字服务器网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
// @remark RegisterNameServer优先于RegisterFront
void TraderApi_RegisterNameServer(CThostFtdcTraderApi* p, char* pszNsAddress);

// 注册名字服务器用户信息
// @param pFensUserInfo：用户信息。
void TraderApi_RegisterFensUserInfo(CThostFtdcTraderApi* p,
                                    CThostFtdcFensUserInfoField* pFensUserInfo);

// 注册回调接口
// @param pSpi 派生自回调接口类的实例
void TraderApi_RegisterSpi(CThostFtdcTraderApi* p, CTPTraderSpi* pSpi);

// 订阅私有流。
// @param nResumeType 私有流重传方式
//         THOST_TERT_RESTART:从本交易日开始重传
//         THOST_TERT_RESUME:从上次收到的续传
//         THOST_TERT_QUICK:只传送登录后私有流的内容
// @remark 该方法要在Init方法前调用。若不调用则不会收到私有流的数据。
void TraderApi_SubscribePrivateTopic(CThostFtdcTraderApi* p,
                                     THOST_TE_RESUME_TYPE nResumeType);

// 订阅公共流。
// @param nResumeType 公共流重传方式
//         THOST_TERT_RESTART:从本交易日开始重传
//         THOST_TERT_RESUME:从上次收到的续传
//         THOST_TERT_QUICK:只传送登录后公共流的内容
// @remark 该方法要在Init方法前调用。若不调用则不会收到公共流的数据。
void TraderApi_SubscribePublicTopic(CThostFtdcTraderApi* p,
                                    THOST_TE_RESUME_TYPE nResumeType);

// 客户端认证请求
int TraderApi_ReqAuthenticate(
    CThostFtdcTraderApi* p,
    CThostFtdcReqAuthenticateField* pReqAuthenticateField, int nRequestID);

// 用户登录请求
int TraderApi_ReqUserLogin(CThostFtdcTraderApi* p,
                           CThostFtdcReqUserLoginField* pReqUserLoginField,
                           int nRequestID);


// 登出请求
int TraderApi_ReqUserLogout(CThostFtdcTraderApi* p,
                            CThostFtdcUserLogoutField* pUserLogout,
                            int nRequestID);

// 用户口令更新请求
int TraderApi_ReqUserPasswordUpdate(
    CThostFtdcTraderApi* p,
    CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, int nRequestID);

// 资金账户口令更新请求
int TraderApi_ReqTradingAccountPasswordUpdate(
    CThostFtdcTraderApi* p,
    CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate,
    int nRequestID);

// 报单录入请求
int TraderApi_ReqOrderInsert(CThostFtdcTraderApi* p,
                             CThostFtdcInputOrderField* pInputOrder,
                             int nRequestID);

// 预埋单录入请求
int TraderApi_ReqParkedOrderInsert(CThostFtdcTraderApi* p,
                                   CThostFtdcParkedOrderField* pParkedOrder,
                                   int nRequestID);

// 预埋撤单录入请求
int TraderApi_ReqParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcParkedOrderActionField* pParkedOrderAction, int nRequestID);

// 报单操作请求
int TraderApi_ReqOrderAction(CThostFtdcTraderApi* p,
                             CThostFtdcInputOrderActionField* pInputOrderAction,
                             int nRequestID);

#ifdef v6_7_0
// 查询最大报单数量请求
int TraderApi_ReqQueryMaxOrderVolume(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMaxOrderVolumeField* pQueryMaxOrderVolume, int nRequestID);
#else
// 查询最大报单数量请求
int TraderApi_ReqQueryMaxOrderVolume(
    CThostFtdcTraderApi* p,
    CThostFtdcQueryMaxOrderVolumeField* pQueryMaxOrderVolume, int nRequestID);
#endif

// 投资者结算结果确认
int TraderApi_ReqSettlementInfoConfirm(
    CThostFtdcTraderApi* p,
    CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm,
    int nRequestID);

// 请求删除预埋单
int TraderApi_ReqRemoveParkedOrder(
    CThostFtdcTraderApi* p,
    CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, int nRequestID);

// 请求删除预埋撤单
int TraderApi_ReqRemoveParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction,
    int nRequestID);

// 执行宣告录入请求
int TraderApi_ReqExecOrderInsert(CThostFtdcTraderApi* p,
                                 CThostFtdcInputExecOrderField* pInputExecOrder,
                                 int nRequestID);

// 执行宣告操作请求
int TraderApi_ReqExecOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcInputExecOrderActionField* pInputExecOrderAction, int nRequestID);

// 询价录入请求
int TraderApi_ReqForQuoteInsert(CThostFtdcTraderApi* p,
                                CThostFtdcInputForQuoteField* pInputForQuote,
                                int nRequestID);

// 报价录入请求
int TraderApi_ReqQuoteInsert(CThostFtdcTraderApi* p,
                             CThostFtdcInputQuoteField* pInputQuote,
                             int nRequestID);

// 报价操作请求
int TraderApi_ReqQuoteAction(CThostFtdcTraderApi* p,
                             CThostFtdcInputQuoteActionField* pInputQuoteAction,
                             int nRequestID);

// 批量报单操作请求
int TraderApi_ReqBatchOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction,
    int nRequestID);

// 申请组合录入请求
int TraderApi_ReqCombActionInsert(
    CThostFtdcTraderApi* p, CThostFtdcInputCombActionField* pInputCombAction,
    int nRequestID);

// 请求查询报单
int TraderApi_ReqQryOrder(CThostFtdcTraderApi* p,
                          CThostFtdcQryOrderField* pQryOrder, int nRequestID);

// 请求查询成交
int TraderApi_ReqQryTrade(CThostFtdcTraderApi* p,
                          CThostFtdcQryTradeField* pQryTrade, int nRequestID);

// 请求查询投资者持仓
int TraderApi_ReqQryInvestorPosition(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionField* pQryInvestorPosition, int nRequestID);

// 请求查询资金账户
int TraderApi_ReqQryTradingAccount(
    CThostFtdcTraderApi* p,
    CThostFtdcQryTradingAccountField* pQryTradingAccount, int nRequestID);

// 请求查询投资者
int TraderApi_ReqQryInvestor(CThostFtdcTraderApi* p,
                             CThostFtdcQryInvestorField* pQryInvestor,
                             int nRequestID);

// 请求查询交易编码
int TraderApi_ReqQryTradingCode(CThostFtdcTraderApi* p,
                                CThostFtdcQryTradingCodeField* pQryTradingCode,
                                int nRequestID);

// 请求查询合约保证金率
int TraderApi_ReqQryInstrumentMarginRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentMarginRateField* pQryInstrumentMarginRate,
    int nRequestID);

// 请求查询合约手续费率
int TraderApi_ReqQryInstrumentCommissionRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentCommissionRateField* pQryInstrumentCommissionRate,
    int nRequestID);

// 请求查询交易所
int TraderApi_ReqQryExchange(CThostFtdcTraderApi* p,
                             CThostFtdcQryExchangeField* pQryExchange,
                             int nRequestID);

// 请求查询产品
int TraderApi_ReqQryProduct(CThostFtdcTraderApi* p,
                            CThostFtdcQryProductField* pQryProduct,
                            int nRequestID);

// 请求查询合约
int TraderApi_ReqQryInstrument(CThostFtdcTraderApi* p,
                               CThostFtdcQryInstrumentField* pQryInstrument,
                               int nRequestID);

// 请求查询行情
int TraderApi_ReqQryDepthMarketData(
    CThostFtdcTraderApi* p,
    CThostFtdcQryDepthMarketDataField* pQryDepthMarketData, int nRequestID);

// 请求查询投资者结算结果
int TraderApi_ReqQrySettlementInfo(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySettlementInfoField* pQrySettlementInfo, int nRequestID);

// 请求查询转帐银行
int TraderApi_ReqQryTransferBank(
    CThostFtdcTraderApi* p, CThostFtdcQryTransferBankField* pQryTransferBank,
    int nRequestID);

// 请求查询投资者持仓明细
int TraderApi_ReqQryInvestorPositionDetail(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionDetailField* pQryInvestorPositionDetail,
    int nRequestID);

// 请求查询客户通知
int TraderApi_ReqQryNotice(CThostFtdcTraderApi* p,
                           CThostFtdcQryNoticeField* pQryNotice,
                           int nRequestID);

// 请求查询结算信息确认
int TraderApi_ReqQrySettlementInfoConfirm(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySettlementInfoConfirmField* pQrySettlementInfoConfirm,
    int nRequestID);

// 请求查询投资者持仓明细
int TraderApi_ReqQryInvestorPositionCombineDetail(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionCombineDetailField*
        pQryInvestorPositionCombineDetail,
    int nRequestID);

// 请求查询保证金监管系统经纪公司资金账户密钥
int TraderApi_ReqQryCFMMCTradingAccountKey(
    CThostFtdcTraderApi* p,
    CThostFtdcQryCFMMCTradingAccountKeyField* pQryCFMMCTradingAccountKey,
    int nRequestID);

// 请求查询仓单折抵信息
int TraderApi_ReqQryEWarrantOffset(
    CThostFtdcTraderApi* p,
    CThostFtdcQryEWarrantOffsetField* pQryEWarrantOffset, int nRequestID);

// 请求查询投资者品种/跨品种保证金
int TraderApi_ReqQryInvestorProductGroupMargin(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorProductGroupMarginField*
        pQryInvestorProductGroupMargin,
    int nRequestID);

// 请求查询交易所保证金率
int TraderApi_ReqQryExchangeMarginRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryExchangeMarginRateField* pQryExchangeMarginRate,
    int nRequestID);

// 请求查询交易所调整保证金率
int TraderApi_ReqQryExchangeMarginRateAdjust(
    CThostFtdcTraderApi* p,
    CThostFtdcQryExchangeMarginRateAdjustField* pQryExchangeMarginRateAdjust,
    int nRequestID);

// 请求查询汇率
int TraderApi_ReqQryExchangeRate(
    CThostFtdcTraderApi* p, CThostFtdcQryExchangeRateField* pQryExchangeRate,
    int nRequestID);

// 请求查询二级代理操作员银期权限
int TraderApi_ReqQrySecAgentACIDMap(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySecAgentACIDMapField* pQrySecAgentACIDMap, int nRequestID);

// 请求查询产品报价汇率
int TraderApi_ReqQryProductExchRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryProductExchRateField* pQryProductExchRate, int nRequestID);

// 请求查询产品组
int TraderApi_ReqQryProductGroup(
    CThostFtdcTraderApi* p, CThostFtdcQryProductGroupField* pQryProductGroup,
    int nRequestID);

// 请求查询做市商合约手续费率
int TraderApi_ReqQryMMInstrumentCommissionRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMMInstrumentCommissionRateField*
        pQryMMInstrumentCommissionRate,
    int nRequestID);

// 请求查询做市商期权合约手续费
int TraderApi_ReqQryMMOptionInstrCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMMOptionInstrCommRateField* pQryMMOptionInstrCommRate,
    int nRequestID);

// 请求查询报单手续费
int TraderApi_ReqQryInstrumentOrderCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentOrderCommRateField* pQryInstrumentOrderCommRate,
    int nRequestID);

// 请求查询资金账户
int TraderApi_ReqQrySecAgentTradingAccount(
    CThostFtdcTraderApi* p,
    CThostFtdcQryTradingAccountField* pQryTradingAccount, int nRequestID);

// 请求查询二级代理商资金校验模式
int TraderApi_ReqQrySecAgentCheckMode(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySecAgentCheckModeField* pQrySecAgentCheckMode, int nRequestID);

// 请求查询二级代理商信息
int TraderApi_ReqQrySecAgentTradeInfo(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySecAgentTradeInfoField* pQrySecAgentTradeInfo, int nRequestID);

// 请求查询期权交易成本
int TraderApi_ReqQryOptionInstrTradeCost(
    CThostFtdcTraderApi* p,
    CThostFtdcQryOptionInstrTradeCostField* pQryOptionInstrTradeCost,
    int nRequestID);

// 请求查询期权合约手续费
int TraderApi_ReqQryOptionInstrCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryOptionInstrCommRateField* pQryOptionInstrCommRate,
    int nRequestID);

// 请求查询执行宣告
int TraderApi_ReqQryExecOrder(CThostFtdcTraderApi* p,
                              CThostFtdcQryExecOrderField* pQryExecOrder,
                              int nRequestID);

// 请求查询询价
int TraderApi_ReqQryForQuote(CThostFtdcTraderApi* p,
                             CThostFtdcQryForQuoteField* pQryForQuote,
                             int nRequestID);

// 请求查询报价
int TraderApi_ReqQryQuote(CThostFtdcTraderApi* p,
                          CThostFtdcQryQuoteField* pQryQuote, int nRequestID);

// 请求查询期权自对冲
int TraderApi_ReqQryOptionSelfClose(
    CThostFtdcTraderApi* p,
    CThostFtdcQryOptionSelfCloseField* pQryOptionSelfClose, int nRequestID);

// 请求查询投资单元
int TraderApi_ReqQryInvestUnit(CThostFtdcTraderApi* p,
                               CThostFtdcQryInvestUnitField* pQryInvestUnit,
                               int nRequestID);

// 请求查询组合合约安全系数
int TraderApi_ReqQryCombInstrumentGuard(
    CThostFtdcTraderApi* p,
    CThostFtdcQryCombInstrumentGuardField* pQryCombInstrumentGuard,
    int nRequestID);

// 请求查询申请组合
int TraderApi_ReqQryCombAction(CThostFtdcTraderApi* p,
                               CThostFtdcQryCombActionField* pQryCombAction,
                               int nRequestID);

// 请求查询转帐流水
int TraderApi_ReqQryTransferSerial(
    CThostFtdcTraderApi* p,
    CThostFtdcQryTransferSerialField* pQryTransferSerial, int nRequestID);

// 请求查询银期签约关系
int TraderApi_ReqQryAccountregister(
    CThostFtdcTraderApi* p,
    CThostFtdcQryAccountregisterField* pQryAccountregister, int nRequestID);

// 请求查询签约银行
int TraderApi_ReqQryContractBank(
    CThostFtdcTraderApi* p, CThostFtdcQryContractBankField* pQryContractBank,
    int nRequestID);

// 请求查询预埋单
int TraderApi_ReqQryParkedOrder(CThostFtdcTraderApi* p,
                                CThostFtdcQryParkedOrderField* pQryParkedOrder,
                                int nRequestID);

// 请求查询预埋撤单
int TraderApi_ReqQryParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcQryParkedOrderActionField* pQryParkedOrderAction, int nRequestID);

// 请求查询交易通知
int TraderApi_ReqQryTradingNotice(
    CThostFtdcTraderApi* p, CThostFtdcQryTradingNoticeField* pQryTradingNotice,
    int nRequestID);

// 请求查询经纪公司交易参数
int TraderApi_ReqQryBrokerTradingParams(
    CThostFtdcTraderApi* p,
    CThostFtdcQryBrokerTradingParamsField* pQryBrokerTradingParams,
    int nRequestID);

// 请求查询经纪公司交易算法
int TraderApi_ReqQryBrokerTradingAlgos(
    CThostFtdcTraderApi* p,
    CThostFtdcQryBrokerTradingAlgosField* pQryBrokerTradingAlgos,
    int nRequestID);

// 请求查询监控中心用户令牌
int TraderApi_ReqQueryCFMMCTradingAccountToken(
    CThostFtdcTraderApi* p,
    CThostFtdcQueryCFMMCTradingAccountTokenField*
        pQueryCFMMCTradingAccountToken,
    int nRequestID);

// 期货发起银行资金转期货请求
int TraderApi_ReqFromBankToFutureByFuture(
    CThostFtdcTraderApi* p, CThostFtdcReqTransferField* pReqTransfer,
    int nRequestID);

// 期货发起期货资金转银行请求
int TraderApi_ReqFromFutureToBankByFuture(
    CThostFtdcTraderApi* p, CThostFtdcReqTransferField* pReqTransfer,
    int nRequestID);

// 期货发起查询银行余额请求
int TraderApi_ReqQueryBankAccountMoneyByFuture(
    CThostFtdcTraderApi* p, CThostFtdcReqQueryAccountField* pReqQueryAccount,
    int nRequestID);

/* Utils */
CTPMarketSpi* CTPMarketSpi_New();
CTPTraderSpi* CTPTraderSpi_New();
}
