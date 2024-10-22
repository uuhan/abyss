#include "ThostFtdcUserApiStruct.h"
#include <iostream>
#include <ctp.h>

#define RUST_CALL0(fn)                                                   \
    if (_mdspi_##fn##_cb) {                                              \
        rust_mdspi_##fn(_mdspi_##fn##_cb);                               \
    } else {                                                             \
        rust_log_warn("[mdspi.cc] rust_mdspi_" #fn " not implemented."); \
    }
#define RUST_CALL(fn, ...)                                               \
    if (_mdspi_##fn##_cb) {                                              \
        rust_mdspi_##fn(_mdspi_##fn##_cb, __VA_ARGS__);                  \
    } else {                                                             \
        rust_log_warn("[mdspi.cc] rust_mdspi_" #fn " not implemented."); \
    }

// CTP 回调
// CTPMarketSpi 实现

extern "C" {
void rust_mdspi_on_front_connected(void *data);
}
// 当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。
void
CTPMarketSpi::OnFrontConnected()
{
    RUST_CALL0(on_front_connected);
}

extern "C" {
void rust_mdspi_on_front_disconnected(void *data, int nReason);
}
// 当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。
// @param nReason 错误原因
//         0x1001 网络读失败
//         0x1002 网络写失败
//         0x2001 接收心跳超时
//         0x2002 发送心跳失败
//         0x2003 收到错误报文
void
CTPMarketSpi::OnFrontDisconnected(int nReason)
{
    RUST_CALL(on_front_disconnected, nReason);
}

extern "C" {
void rust_mdspi_on_heartbeat_warning(void *data, int nTimeLapse);
}

// 心跳超时警告。当长时间未收到报文时，该方法被调用。
// @param nTimeLapse 距离上次接收报文的时间
void
CTPMarketSpi::OnHeartBeatWarning(int nTimeLapse)
{
    RUST_CALL(on_heartbeat_warning, nTimeLapse);
}


extern "C" {
void rust_mdspi_on_rsp_user_login(void *data,
                                  CThostFtdcRspUserLoginField *pRspUserLogin,
                                  CThostFtdcRspInfoField *pRspInfo,
                                  int nRequestID, bool bIsLast);
}
// 登录请求响应
void
CTPMarketSpi::OnRspUserLogin(CThostFtdcRspUserLoginField *pRspUserLogin,
                             CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                             bool bIsLast)
{
    RUST_CALL(on_rsp_user_login, pRspUserLogin, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_mdspi_on_rsp_user_logout(void *data,
                                   CThostFtdcUserLogoutField *pUserLogout,
                                   CThostFtdcRspInfoField *pRspInfo,
                                   int nRequestID, bool bIsLast);
}
// 登出请求响应
void
CTPMarketSpi::OnRspUserLogout(CThostFtdcUserLogoutField *pUserLogout,
                              CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                              bool bIsLast)
{
    RUST_CALL(on_rsp_user_logout, pUserLogout, pRspInfo, nRequestID, bIsLast);
}

#ifdef v6_3_19
extern "C" {
void rust_mdspi_on_rsp_qry_multicast_instrument(
    void *data, CThostFtdcMulticastInstrumentField *pMulticastInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
// 请求查询组播合约响应
void
CTPMarketSpi::OnRspQryMulticastInstrument(
    CThostFtdcMulticastInstrumentField *pMulticastInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_qry_multicast_instrument, pMulticastInstrument, pRspInfo,
              nRequestID, bIsLast);
}
#endif

extern "C" {
void rust_mdspi_on_rsp_error(void *data, CThostFtdcRspInfoField *pRspInfo,
                             int nRequestID, bool bIsLast);
}
// 错误应答
void
CTPMarketSpi::OnRspError(CThostFtdcRspInfoField *pRspInfo, int nRequestID,
                         bool bIsLast)
{
    RUST_CALL(on_rsp_error, pRspInfo, nRequestID, bIsLast);
}

extern "C" {
void rust_mdspi_on_rsp_sub_market_data(
    void *data, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, int bIsLast);
}
// 订阅行情应答
void
CTPMarketSpi::OnRspSubMarketData(
    CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_sub_market_data, pSpecificInstrument, pRspInfo, nRequestID,
              bIsLast);
}

extern "C" {
void rust_mdspi_on_rsp_unsub_market_data(
    void *data, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
// 取消订阅行情应答
void
CTPMarketSpi::OnRspUnSubMarketData(
    CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_unsub_market_data, pSpecificInstrument, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_mdspi_on_rsp_sub_for_quote_rsp(
    void *data, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
// 订阅询价应答
void
CTPMarketSpi::OnRspSubForQuoteRsp(
    CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_sub_for_quote_rsp, pSpecificInstrument, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_mdspi_on_rsp_unsub_for_quote_rsp(
    void *data, CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast);
}
// 取消订阅询价应答
void
CTPMarketSpi::OnRspUnSubForQuoteRsp(
    CThostFtdcSpecificInstrumentField *pSpecificInstrument,
    CThostFtdcRspInfoField *pRspInfo, int nRequestID, bool bIsLast)
{
    RUST_CALL(on_rsp_unsub_for_quote_rsp, pSpecificInstrument, pRspInfo,
              nRequestID, bIsLast);
}

extern "C" {
void rust_mdspi_on_rtn_depth_market_data(
    void *data, CThostFtdcDepthMarketDataField *pDepthMarketData);
}
// 深度行情通知
void
CTPMarketSpi::OnRtnDepthMarketData(
    CThostFtdcDepthMarketDataField *pDepthMarketData)
{
    RUST_CALL(on_rtn_depth_market_data, pDepthMarketData);
}

extern "C" {
void rust_mdspi_on_rtn_for_quote_rsp(void *data,
                                     CThostFtdcForQuoteRspField *pForQuoteRsp);
}
// 询价通知
void
CTPMarketSpi::OnRtnForQuoteRsp(CThostFtdcForQuoteRspField *pForQuoteRsp)
{
    RUST_CALL(on_rtn_for_quote_rsp, pForQuoteRsp);
}

// 清理
void
CTPMarketSpi::Release()
{
    this->~CTPMarketSpi();
}
