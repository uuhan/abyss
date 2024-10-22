use crate::ctp::api::*;

ctp_extern!(
    "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
    md->on_front_connected,
);

ctp_extern!(
    "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。"
    "@param nReason 错误原因"
    "        0x1001 网络读失败"
    "        0x1002 网络写失败"
    "        0x2001 接收心跳超时"
    "        0x2002 发送心跳失败"
    "        0x2003 收到错误报文"
    md->on_front_disconnected,
    nReason: i32,
);

ctp_extern!(
    "心跳超时警告。当长时间未收到报文时，该方法被调用。"
    "@param nTimeLapse 距离上次接收报文的时间"
    "注意: 这个函数永远不会被调用到"
    md->on_heartbeat_warning,
    nTimeLapse: i32,
);

ctp_extern!(
    "错误应答"
    md->on_rsp_error,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

#[cfg(feature = "v6_3_19")]
ctp_extern!(
    "请求查询组播合约响应"
    md->on_rsp_qry_multicast_instrument,
    pMulticastInstrument: &CThostFtdcMulticastInstrumentField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "订阅询价应答"
    md->on_rsp_sub_for_quote_rsp,
    pSpecificInstrument: &CThostFtdcSpecificInstrumentField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "订阅行情应答"
    md->on_rsp_sub_market_data,
    pSpecificInstrument: &CThostFtdcSpecificInstrumentField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "取消订阅询价应答"
    md->on_rsp_unsub_for_quote_rsp,
    pSpecificInstrument: &CThostFtdcSpecificInstrumentField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "取消订阅行情应答"
    md->on_rsp_unsub_market_data,
    pSpecificInstrument: &CThostFtdcSpecificInstrumentField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "登录请求响应"
    md->on_rsp_user_login,
    pRspUserLogin: &CThostFtdcRspUserLoginField,
    pRspInfo: &CThostFtdcRspInfoField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "登出请求响应"
    md->on_rsp_user_logout,
    pUserLogout: &CThostFtdcUserLogoutField,
    nRequestID: i32,
    bIsLast: bool,
);

ctp_extern!(
    "深度行情通知"
    md->on_rtn_depth_market_data,
    pDepthMarketData: &CThostFtdcDepthMarketDataField,
);

ctp_extern!(
    "询价通知"
    md->on_rtn_for_quote_rsp,
    pForQuoteRsp: &CThostFtdcForQuoteRspField,
);
