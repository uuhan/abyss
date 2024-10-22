use super::{MarketApi, MarketSpi};
use crate::ctp::api::*;
use crate::ctp::error::*;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;

impl MarketApi {
    pub unsafe fn new(lp: impl AsRef<str>, bIsUsingUdp: bool, bIsMulticast: bool) -> Self {
        let path = CString::new(lp.as_ref())
            .expect("wrong log dir path")
            .into_raw();
        let inner = CThostFtdcMdApi::CreateFtdcMdApi(path, bIsUsingUdp, bIsMulticast);
        Self(&mut *inner)
    }

    // 删除接口对象本身
    // @remark 不再使用本接口对象时,调用该函数删除接口对象
    #[inline]
    fn release(self) {
        unsafe { MdApi_Release(self.0) }
    }

    // 初始化
    // @remark 初始化运行环境,只有调用后,接口才开始工作
    #[inline]
    pub fn init(&mut self) {
        unsafe { MdApi_Init(self.0) }
    }

    // 等待接口线程结束运行
    // @return 线程退出代码
    #[inline]
    pub fn join(&self) -> CTPApiResult {
        ret_value_wrapper!(unsafe { MdApi_Join(self.0) })
    }

    // 获取当前交易日
    // @retrun 获取到的交易日
    // @remark 只有登录成功后,才能得到正确的交易日
    #[inline]
    pub fn get_trading_day(&self) -> &str {
        unsafe {
            let p = MdApi_GetTradingDay(self.0);
            CStr::from_ptr(p as _).to_str().unwrap()
        }
    }

    // 注册前置机网络地址
    // @param pszFrontAddress：前置机网络地址。
    // @remark
    // 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
    // @remark
    // “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
    #[inline]
    pub fn register_front(&self, front: impl Into<String>) {
        unsafe {
            let front: String = front.into();
            let front = CString::new(front).unwrap();
            MdApi_RegisterFront(self.0, front.as_ptr() as _)
        }
    }

    // 注册名字服务器网络地址
    // @param pszNsAddress：名字服务器网络地址。
    // @remark
    // 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
    // @remark
    // “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
    // @remark RegisterNameServer优先于RegisterFront
    #[inline]
    pub fn register_name_server(&self, ns: impl Into<String>) {
        unsafe {
            let ns: String = ns.into();
            let ns = CString::new(ns).unwrap();
            MdApi_RegisterNameServer(self.0, ns.as_ptr() as _)
        }
    }

    // 注册名字服务器用户信息
    // @param pFensUserInfo：用户信息。
    #[inline]
    pub fn register_fens_user_info(&self, info: &CThostFtdcFensUserInfoField) {
        unsafe { MdApi_RegisterFensUserInfo(self.0, info) }
    }

    // 注册回调接口
    // @param pSpi 派生自回调接口类的实例
    #[inline]
    pub fn register_spi(&self, spi: &MarketSpi) {
        unsafe {
            MdApi_RegisterSpi(self.0, spi.0);
        }
    }

    // 订阅行情。
    // @param ppInstrumentID 合约ID
    // @param nCount 要订阅/退订行情的合约个数
    // @remark
    #[inline]
    pub fn subscribe_market_data<S: ToString>(&self, ppInstrumentID: &[S]) -> CTPApiResult {
        let mut ppInstrumentID: Vec<*mut c_char> = ppInstrumentID
            .iter()
            .map(|ins| CString::new(ins.to_string()).unwrap().into_raw())
            .collect();

        ret_value_wrapper!(unsafe {
            MdApi_SubscribeMarketData(
                self.0,
                ppInstrumentID.as_mut_ptr(),
                ppInstrumentID.len() as _,
            )
        })
    }

    // 退订行情。
    // @param ppInstrumentID 合约ID
    // @param nCount 要订阅/退订行情的合约个数
    // @remark
    #[inline]
    pub fn unsubscribe_market_data<S: ToString>(&self, ppInstrumentID: &[S]) -> CTPApiResult {
        let mut ppInstrumentID: Vec<*mut c_char> = ppInstrumentID
            .iter()
            .map(|ins| CString::new(ins.to_string()).unwrap().into_raw())
            .collect();

        ret_value_wrapper!(unsafe {
            MdApi_UnSubscribeMarketData(
                self.0,
                ppInstrumentID.as_mut_ptr(),
                ppInstrumentID.len() as _,
            )
        })
    }

    // 订阅询价。
    // @param ppInstrumentID 合约ID
    // @param nCount 要订阅/退订行情的合约个数
    // @remark
    #[inline]
    pub fn subscribe_for_quote_rsp<S: ToString>(&self, ppInstrumentID: &[S]) -> CTPApiResult {
        let mut ppInstrumentID: Vec<*mut c_char> = ppInstrumentID
            .iter()
            .map(|ins| CString::new(ins.to_string()).unwrap().into_raw())
            .collect();

        ret_value_wrapper!(unsafe {
            MdApi_SubscribeForQuoteRsp(
                self.0,
                ppInstrumentID.as_mut_ptr(),
                ppInstrumentID.len() as _,
            )
        })
    }

    // 退订询价。
    // @param ppInstrumentID 合约ID
    // @param nCount 要订阅/退订行情的合约个数
    // @remark
    #[inline]
    pub fn unsubscribe_for_quote_rsp<S: ToString>(&self, ppInstrumentID: &[S]) -> CTPApiResult {
        let mut ppInstrumentID: Vec<*mut c_char> = ppInstrumentID
            .iter()
            .map(|ins| CString::new(ins.to_string()).unwrap().into_raw())
            .collect();

        ret_value_wrapper!(unsafe {
            MdApi_UnSubscribeForQuoteRsp(
                self.0,
                ppInstrumentID.as_mut_ptr(),
                ppInstrumentID.len() as _,
            )
        })
    }

    // 用户登录请求
    #[inline]
    pub fn req_user_login(
        &self,
        pReqUserLoginField: &CThostFtdcReqUserLoginField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { MdApi_ReqUserLogin(self.0, pReqUserLoginField, nRequestID) })
    }

    // 登出请求
    #[inline]
    pub fn req_user_logout(
        &self,
        pUserLogout: &CThostFtdcUserLogoutField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe { MdApi_ReqUserLogout(self.0, pUserLogout, nRequestID) })
    }

    // 请求查询组播合约
    #[cfg(feature = "v6_3_19")]
    #[inline]
    pub fn req_qry_multicast_instrument(
        &self,
        pQryMulticastInstrument: &CThostFtdcQryMulticastInstrumentField,
        nRequestID: i32,
    ) -> CTPApiResult {
        ret_value_wrapper!(unsafe {
            MdApi_ReqQryMulticastInstrument(self.0, pQryMulticastInstrument, nRequestID)
        })
    }
}
