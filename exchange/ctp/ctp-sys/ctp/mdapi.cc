#include "ThostFtdcUserApiStruct.h"
#include <ctp.h>

extern "C" {

// 删除接口对象本身
// @remark 不再使用本接口对象时,调用该函数删除接口对象
void
MdApi_Release(CThostFtdcMdApi* p)
{
    return p->Release();
}

// 初始化
// @remark 初始化运行环境,只有调用后,接口才开始工作
void
MdApi_Init(CThostFtdcMdApi* p)
{
    return p->Init();
}

// 等待接口线程结束运行
// @return 线程退出代码
int
MdApi_Join(CThostFtdcMdApi* p)
{
    return p->Join();
}

// 获取当前交易日
// @retrun 获取到的交易日
// @remark 只有登录成功后,才能得到正确的交易日
const char*
MdApi_GetTradingDay(CThostFtdcMdApi* p)
{
    return p->GetTradingDay();
}

// 注册前置机网络地址
// @param pszFrontAddress：前置机网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:17001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”17001”代表服务器端口号。
void
MdApi_RegisterFront(CThostFtdcMdApi* p, char* pszFrontAddress)
{
    return p->RegisterFront(pszFrontAddress);
}

// 注册名字服务器网络地址
// @param pszNsAddress：名字服务器网络地址。
// @remark
// 网络地址的格式为：“protocol://ipaddress:port”，如：”tcp://127.0.0.1:12001”。
// @remark
// “tcp”代表传输协议，“127.0.0.1”代表服务器地址。”12001”代表服务器端口号。
// @remark RegisterNameServer优先于RegisterFront
void
MdApi_RegisterNameServer(CThostFtdcMdApi* p, char* pszNsAddress)
{
    return p->RegisterNameServer(pszNsAddress);
}

// 注册名字服务器用户信息
// @param pFensUserInfo：用户信息。
void
MdApi_RegisterFensUserInfo(CThostFtdcMdApi* p, CThostFtdcFensUserInfoField* pi)
{
    return p->RegisterFensUserInfo(pi);
}

// 注册回调接口
// @param pSpi 派生自回调接口类的实例
void
MdApi_RegisterSpi(CThostFtdcMdApi* p, CTPMarketSpi* pSpi)
{
    return p->RegisterSpi(pSpi);
}

// 订阅行情。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int
MdApi_SubscribeMarketData(CThostFtdcMdApi* p, char* ppInstrumentID[],
                          int nCount)
{
    return p->SubscribeMarketData(ppInstrumentID, nCount);
}

// 退订行情。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int
MdApi_UnSubscribeMarketData(CThostFtdcMdApi* p, char* ppInstrumentID[],
                            int nCount)
{
    return p->UnSubscribeMarketData(ppInstrumentID, nCount);
}

// 订阅询价。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int
MdApi_SubscribeForQuoteRsp(CThostFtdcMdApi* p, char* ppInstrumentID[],
                           int nCount)
{
    return p->SubscribeForQuoteRsp(ppInstrumentID, nCount);
}

// 退订询价。
// @param ppInstrumentID 合约ID
// @param nCount 要订阅/退订行情的合约个数
// @remark
int
MdApi_UnSubscribeForQuoteRsp(CThostFtdcMdApi* p, char* ppInstrumentID[],
                             int nCount)
{
    return p->UnSubscribeForQuoteRsp(ppInstrumentID, nCount);
}

// 用户登录请求
int
MdApi_ReqUserLogin(CThostFtdcMdApi* p,
                   CThostFtdcReqUserLoginField* pReqUserLoginField,
                   int nRequestID)
{
    return p->ReqUserLogin(pReqUserLoginField, nRequestID);
}

// 登出请求
int
MdApi_ReqUserLogout(CThostFtdcMdApi* p, CThostFtdcUserLogoutField* pUserLogout,
                    int nRequestID)
{
    return p->ReqUserLogout(pUserLogout, nRequestID);
}

#ifdef v6_3_19
// 请求查询组播合约
int
MdApi_ReqQryMulticastInstrument(
    CThostFtdcMdApi* p,
    CThostFtdcQryMulticastInstrumentField* pQryMulticastInstrument,
    int nRequestID)
{
    return p->ReqQryMulticastInstrument(pQryMulticastInstrument, nRequestID);
}
#endif

}
