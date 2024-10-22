use super::super::api::*;
use super::MarketSpi;
use std::ffi::{c_void, CStr, CString};

impl MarketSpi {
    pub unsafe fn new() -> Self {
        Self(&mut *CTPMarketSpi_New())
    }

    pub fn release(&mut self) {
        unsafe { CTPMarketSpi_Release(self.0) }
    }
}

impl MarketSpi {
    ctp_fn_wrapper!(
        "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
        md->on_front_connected,
    );

    ctp_fn_wrapper!(
        "当客户端与交易后台通信连接断开时，该方法被调用。当发生这个情况后，API会自动重新连接，客户端可不做处理。"
        "@param nReason 错误原因"
        "        0x1001 网络读失败"
        "        0x1002 网络写失败"
        "        0x2001 接收心跳超时"
        "        0x2002 发送心跳失败"
        "        0x2003 收到错误报文"
        md->on_front_disconnected,
        i32,
    );

    ctp_fn_wrapper!(
        "心跳超时警告。当长时间未收到报文时，该方法被调用。"
        "@param nTimeLapse 距离上次接收报文的时间"
        "注意: 这个函数永远不会被调用到"
        md->on_heartbeat_warning,
        i32,
    );

    ctp_fn_wrapper!(
        "错误应答"
        md->on_rsp_error,
        &CThostFtdcRspInfoField, i32, bool,
    );

    #[cfg(feature = "v6_3_19")]
    ctp_fn_wrapper!(
        "请求查询组播合约响应"
        md->on_rsp_qry_multicast_instrument,
        &CThostFtdcMulticastInstrumentField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "订阅询价应答"
        md->on_rsp_sub_for_quote_rsp,
        &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "订阅行情应答"
        md->on_rsp_sub_market_data,
        &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "取消订阅询价应答"
        md->on_rsp_unsub_for_quote_rsp,
        &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "取消订阅行情应答"
        md->on_rsp_unsub_market_data,
        &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "登录请求响应"
        md->on_rsp_user_login,
        &CThostFtdcRspUserLoginField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "登出请求响应"
        md->on_rsp_user_logout,
        &CThostFtdcUserLogoutField, &CThostFtdcRspInfoField, i32, bool,
    );

    ctp_fn_wrapper!(
        "深度行情通知"
        md->on_rtn_depth_market_data,
        &CThostFtdcDepthMarketDataField,
    );

    ctp_fn_wrapper!(
        "询价通知"
        md->on_rtn_for_quote_rsp,
        &CThostFtdcForQuoteRspField,
    );
}

impl Drop for CTPMarketSpi {
    fn drop(&mut self) {
        tracing::debug!("Drop CTPMarketSpi");

        ctp_fn_dropper!(
            "当客户端与交易后台建立起通信连接时（还未登录前），该方法被调用。"
            self,
            md->on_front_connected,
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
            md->on_front_disconnected,
            i32,
        );

        ctp_fn_dropper!(
            "心跳超时警告。当长时间未收到报文时，该方法被调用。"
            "@param nTimeLapse 距离上次接收报文的时间"
            "注意: 这个函数永远不会被调用到"
            self,
            md->on_heartbeat_warning,
            i32,
        );

        ctp_fn_dropper!(
            "错误应答"
            self,
            md->on_rsp_error,
            &CThostFtdcRspInfoField, i32, bool,
        );

        #[cfg(feature = "v6_3_19")]
        ctp_fn_dropper!(
            "请求查询组播合约响应"
            self,
            md->on_rsp_qry_multicast_instrument,
            &CThostFtdcMulticastInstrumentField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "订阅询价应答"
            self,
            md->on_rsp_sub_for_quote_rsp,
            &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "订阅行情应答"
            self,
            md->on_rsp_sub_market_data,
            &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "取消订阅询价应答"
            self,
            md->on_rsp_unsub_for_quote_rsp,
            &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "取消订阅行情应答"
            self,
            md->on_rsp_unsub_market_data,
            &CThostFtdcSpecificInstrumentField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "登录请求响应"
            self,
            md->on_rsp_user_login,
            &CThostFtdcRspUserLoginField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "登出请求响应"
            self,
            md->on_rsp_user_logout,
            &CThostFtdcUserLogoutField, &CThostFtdcRspInfoField, i32, bool,
        );

        ctp_fn_dropper!(
            "深度行情通知"
            self,
            md->on_rtn_depth_market_data,
            &CThostFtdcDepthMarketDataField,
        );

        ctp_fn_dropper!(
            "询价通知"
            self,
            md->on_rtn_for_quote_rsp,
            &CThostFtdcForQuoteRspField,
        );
    }
}
