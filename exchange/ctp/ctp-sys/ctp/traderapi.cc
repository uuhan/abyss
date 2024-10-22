#include <ctp.h>

extern "C" {
// 删除接口对象本身
// @remark 不再使用本接口对象时,调用该函数删除接口对象
void
TraderApi_Release(CThostFtdcTraderApi* p)
{
    return p->Release();
}

// 初始化
// @remark 初始化运行环境,只有调用后,接口才开始工作
void
TraderApi_Init(CThostFtdcTraderApi* p)
{
    return p->Init();
}

// 等待接口线程结束运行
// @return 线程退出代码
int
TraderApi_Join(CThostFtdcTraderApi* p)
{
    return p->Join();
}

// 获取当前交易日
// @retrun 获取到的交易日
// @remark 只有登录成功后,才能得到正确的交易日
const char*
TraderApi_GetTradingDay(CThostFtdcTraderApi* p)
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
TraderApi_RegisterFront(CThostFtdcTraderApi* p, char* pszFrontAddress)
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
TraderApi_RegisterNameServer(CThostFtdcTraderApi* p, char* pszNsAddress)
{
    return p->RegisterNameServer(pszNsAddress);
}

// 注册名字服务器用户信息
// @param pFensUserInfo：用户信息。
void
TraderApi_RegisterFensUserInfo(CThostFtdcTraderApi* p,
                               CThostFtdcFensUserInfoField* pFensUserInfo)
{
    return p->RegisterFensUserInfo(pFensUserInfo);
}

// 注册回调接口
// @param pSpi 派生自回调接口类的实例
void
TraderApi_RegisterSpi(CThostFtdcTraderApi* p, CTPTraderSpi* pSpi)
{
    return p->RegisterSpi(pSpi);
}

// 订阅私有流。
// @param nResumeType 私有流重传方式
//         THOST_TERT_RESTART:从本交易日开始重传
//         THOST_TERT_RESUME:从上次收到的续传
//         THOST_TERT_QUICK:只传送登录后私有流的内容
// @remark 该方法要在Init方法前调用。若不调用则不会收到私有流的数据。
void
TraderApi_SubscribePrivateTopic(CThostFtdcTraderApi* p,
                                THOST_TE_RESUME_TYPE nResumeType)
{
    return p->SubscribePrivateTopic(nResumeType);
}

// 订阅公共流。
// @param nResumeType 公共流重传方式
//         THOST_TERT_RESTART:从本交易日开始重传
//         THOST_TERT_RESUME:从上次收到的续传
//         THOST_TERT_QUICK:只传送登录后公共流的内容
// @remark 该方法要在Init方法前调用。若不调用则不会收到公共流的数据。
void
TraderApi_SubscribePublicTopic(CThostFtdcTraderApi* p,
                               THOST_TE_RESUME_TYPE nResumeType)
{
    return p->SubscribePublicTopic(nResumeType);
}

// 客户端认证请求
int
TraderApi_ReqAuthenticate(CThostFtdcTraderApi* p,
                          CThostFtdcReqAuthenticateField* pReqAuthenticateField,
                          int nRequestID)
{
    return p->ReqAuthenticate(pReqAuthenticateField, nRequestID);
}

// 用户登录请求
int
TraderApi_ReqUserLogin(CThostFtdcTraderApi* p,
                       CThostFtdcReqUserLoginField* pReqUserLoginField,
                       int nRequestID)
{
#if (TARGET == 0)
    // TODO: ios compatible
    return p->ReqUserLogin(pReqUserLoginField, nRequestID, 0, nullptr);
#else
    return p->ReqUserLogin(pReqUserLoginField, nRequestID);
#endif
}


// 登出请求
int
TraderApi_ReqUserLogout(CThostFtdcTraderApi* p,
                        CThostFtdcUserLogoutField* pUserLogout, int nRequestID)
{
    return p->ReqUserLogout(pUserLogout, nRequestID);
}

// 用户口令更新请求
int
TraderApi_ReqUserPasswordUpdate(
    CThostFtdcTraderApi* p,
    CThostFtdcUserPasswordUpdateField* pUserPasswordUpdate, int nRequestID)
{
    return p->ReqUserPasswordUpdate(pUserPasswordUpdate, nRequestID);
}

// 资金账户口令更新请求
int
TraderApi_ReqTradingAccountPasswordUpdate(
    CThostFtdcTraderApi* p,
    CThostFtdcTradingAccountPasswordUpdateField* pTradingAccountPasswordUpdate,
    int nRequestID)
{
    return p->ReqTradingAccountPasswordUpdate(pTradingAccountPasswordUpdate,
                                              nRequestID);
}

// 报单录入请求
int
TraderApi_ReqOrderInsert(CThostFtdcTraderApi* p,
                         CThostFtdcInputOrderField* pInputOrder, int nRequestID)
{
    return p->ReqOrderInsert(pInputOrder, nRequestID);
}

// 预埋单录入请求
int
TraderApi_ReqParkedOrderInsert(CThostFtdcTraderApi* p,
                               CThostFtdcParkedOrderField* pParkedOrder,
                               int nRequestID)
{
    return p->ReqParkedOrderInsert(pParkedOrder, nRequestID);
}

// 预埋撤单录入请求
int
TraderApi_ReqParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcParkedOrderActionField* pParkedOrderAction, int nRequestID)
{
    return p->ReqParkedOrderAction(pParkedOrderAction, nRequestID);
}

// 报单操作请求
int
TraderApi_ReqOrderAction(CThostFtdcTraderApi* p,
                         CThostFtdcInputOrderActionField* pInputOrderAction,
                         int nRequestID)
{
    return p->ReqOrderAction(pInputOrderAction, nRequestID);
}

// 查询最大报单数量请求
#ifdef v6_7_0
int
TraderApi_ReqQryMaxOrderVolume(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMaxOrderVolumeField* pQryMaxOrderVolume, int nRequestID)
{
    return p->ReqQryMaxOrderVolume(pQryMaxOrderVolume, nRequestID);
}
#else
int
TraderApi_ReqQueryMaxOrderVolume(
    CThostFtdcTraderApi* p,
    CThostFtdcQueryMaxOrderVolumeField* pQueryMaxOrderVolume, int nRequestID)
{
    return p->ReqQueryMaxOrderVolume(pQueryMaxOrderVolume, nRequestID);
}
#endif

// 投资者结算结果确认
int
TraderApi_ReqSettlementInfoConfirm(
    CThostFtdcTraderApi* p,
    CThostFtdcSettlementInfoConfirmField* pSettlementInfoConfirm,
    int nRequestID)
{
    return p->ReqSettlementInfoConfirm(pSettlementInfoConfirm, nRequestID);
}

// 请求删除预埋单
int
TraderApi_ReqRemoveParkedOrder(
    CThostFtdcTraderApi* p,
    CThostFtdcRemoveParkedOrderField* pRemoveParkedOrder, int nRequestID)
{
    return p->ReqRemoveParkedOrder(pRemoveParkedOrder, nRequestID);
}

// 请求删除预埋撤单
int
TraderApi_ReqRemoveParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcRemoveParkedOrderActionField* pRemoveParkedOrderAction,
    int nRequestID)
{
    return p->ReqRemoveParkedOrderAction(pRemoveParkedOrderAction, nRequestID);
}

// 执行宣告录入请求
int
TraderApi_ReqExecOrderInsert(CThostFtdcTraderApi* p,
                             CThostFtdcInputExecOrderField* pInputExecOrder,
                             int nRequestID)
{
    return p->ReqExecOrderInsert(pInputExecOrder, nRequestID);
}

// 执行宣告操作请求
int
TraderApi_ReqExecOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcInputExecOrderActionField* pInputExecOrderAction, int nRequestID)
{
    return p->ReqExecOrderAction(pInputExecOrderAction, nRequestID);
}

// 询价录入请求
int
TraderApi_ReqForQuoteInsert(CThostFtdcTraderApi* p,
                            CThostFtdcInputForQuoteField* pInputForQuote,
                            int nRequestID)
{
    return p->ReqForQuoteInsert(pInputForQuote, nRequestID);
}

// 报价录入请求
int
TraderApi_ReqQuoteInsert(CThostFtdcTraderApi* p,
                         CThostFtdcInputQuoteField* pInputQuote, int nRequestID)
{
    return p->ReqQuoteInsert(pInputQuote, nRequestID);
}

// 报价操作请求
int
TraderApi_ReqQuoteAction(CThostFtdcTraderApi* p,
                         CThostFtdcInputQuoteActionField* pInputQuoteAction,
                         int nRequestID)
{
    return p->ReqQuoteAction(pInputQuoteAction, nRequestID);
}

// 批量报单操作请求
int
TraderApi_ReqBatchOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcInputBatchOrderActionField* pInputBatchOrderAction,
    int nRequestID)
{
    return p->ReqBatchOrderAction(pInputBatchOrderAction, nRequestID);
}

// 申请组合录入请求
int
TraderApi_ReqCombActionInsert(CThostFtdcTraderApi* p,
                              CThostFtdcInputCombActionField* pInputCombAction,
                              int nRequestID)
{
    return p->ReqCombActionInsert(pInputCombAction, nRequestID);
}

// 请求查询报单
int
TraderApi_ReqQryOrder(CThostFtdcTraderApi* p,
                      CThostFtdcQryOrderField* pQryOrder, int nRequestID)
{
    return p->ReqQryOrder(pQryOrder, nRequestID);
}

// 请求查询成交
int
TraderApi_ReqQryTrade(CThostFtdcTraderApi* p,
                      CThostFtdcQryTradeField* pQryTrade, int nRequestID)
{
    return p->ReqQryTrade(pQryTrade, nRequestID);
}

// 请求查询投资者持仓
int
TraderApi_ReqQryInvestorPosition(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionField* pQryInvestorPosition, int nRequestID)
{
    return p->ReqQryInvestorPosition(pQryInvestorPosition, nRequestID);
}

// 请求查询资金账户
int
TraderApi_ReqQryTradingAccount(
    CThostFtdcTraderApi* p,
    CThostFtdcQryTradingAccountField* pQryTradingAccount, int nRequestID)
{
    return p->ReqQryTradingAccount(pQryTradingAccount, nRequestID);
}

// 请求查询投资者
int
TraderApi_ReqQryInvestor(CThostFtdcTraderApi* p,
                         CThostFtdcQryInvestorField* pQryInvestor,
                         int nRequestID)
{
    return p->ReqQryInvestor(pQryInvestor, nRequestID);
}

// 请求查询交易编码
int
TraderApi_ReqQryTradingCode(CThostFtdcTraderApi* p,
                            CThostFtdcQryTradingCodeField* pQryTradingCode,
                            int nRequestID)
{
    return p->ReqQryTradingCode(pQryTradingCode, nRequestID);
}

// 请求查询合约保证金率
int
TraderApi_ReqQryInstrumentMarginRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentMarginRateField* pQryInstrumentMarginRate,
    int nRequestID)
{
    return p->ReqQryInstrumentMarginRate(pQryInstrumentMarginRate, nRequestID);
}

// 请求查询合约手续费率
int
TraderApi_ReqQryInstrumentCommissionRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentCommissionRateField* pQryInstrumentCommissionRate,
    int nRequestID)
{
    return p->ReqQryInstrumentCommissionRate(pQryInstrumentCommissionRate, nRequestID);
}

// 请求查询交易所
int
TraderApi_ReqQryExchange(CThostFtdcTraderApi* p,
                         CThostFtdcQryExchangeField* pQryExchange,
                         int nRequestID)
{
    return p->ReqQryExchange(pQryExchange, nRequestID);
}

// 请求查询产品
int
TraderApi_ReqQryProduct(CThostFtdcTraderApi* p,
                        CThostFtdcQryProductField* pQryProduct, int nRequestID)
{
    return p->ReqQryProduct(pQryProduct, nRequestID);
}

// 请求查询合约
int
TraderApi_ReqQryInstrument(CThostFtdcTraderApi* p,
                           CThostFtdcQryInstrumentField* pQryInstrument,
                           int nRequestID)
{
    return p->ReqQryInstrument(pQryInstrument, nRequestID);
}

// 请求查询行情
int
TraderApi_ReqQryDepthMarketData(
    CThostFtdcTraderApi* p,
    CThostFtdcQryDepthMarketDataField* pQryDepthMarketData, int nRequestID)
{
    return p->ReqQryDepthMarketData(pQryDepthMarketData, nRequestID);
}

// 请求查询投资者结算结果
int
TraderApi_ReqQrySettlementInfo(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySettlementInfoField* pQrySettlementInfo, int nRequestID)
{
    return p->ReqQrySettlementInfo(pQrySettlementInfo, nRequestID);
}

// 请求查询转帐银行
int
TraderApi_ReqQryTransferBank(CThostFtdcTraderApi* p,
                             CThostFtdcQryTransferBankField* pQryTransferBank,
                             int nRequestID)
{
    return p->ReqQryTransferBank(pQryTransferBank, nRequestID);
}

// 请求查询投资者持仓明细
int
TraderApi_ReqQryInvestorPositionDetail(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionDetailField* pQryInvestorPositionDetail,
    int nRequestID)
{
    return p->ReqQryInvestorPositionDetail(pQryInvestorPositionDetail, nRequestID);
}

// 请求查询客户通知
int
TraderApi_ReqQryNotice(CThostFtdcTraderApi* p,
                       CThostFtdcQryNoticeField* pQryNotice, int nRequestID)
{
    return p->ReqQryNotice(pQryNotice, nRequestID);
}

// 请求查询结算信息确认
int
TraderApi_ReqQrySettlementInfoConfirm(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySettlementInfoConfirmField* pQrySettlementInfoConfirm,
    int nRequestID)
{
    return p->ReqQrySettlementInfoConfirm(pQrySettlementInfoConfirm, nRequestID);
}

// 请求查询投资者持仓明细
int
TraderApi_ReqQryInvestorPositionCombineDetail(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorPositionCombineDetailField*
        pQryInvestorPositionCombineDetail,
    int nRequestID)
{
    return p->ReqQryInvestorPositionCombineDetail(pQryInvestorPositionCombineDetail, nRequestID);
}

// 请求查询保证金监管系统经纪公司资金账户密钥
int
TraderApi_ReqQryCFMMCTradingAccountKey(
    CThostFtdcTraderApi* p,
    CThostFtdcQryCFMMCTradingAccountKeyField* pQryCFMMCTradingAccountKey,
    int nRequestID)
{
    return p->ReqQryCFMMCTradingAccountKey(pQryCFMMCTradingAccountKey, nRequestID);
}

// 请求查询仓单折抵信息
int
TraderApi_ReqQryEWarrantOffset(
    CThostFtdcTraderApi* p,
    CThostFtdcQryEWarrantOffsetField* pQryEWarrantOffset, int nRequestID)
{
    return p->ReqQryEWarrantOffset(pQryEWarrantOffset, nRequestID);
}

// 请求查询投资者品种/跨品种保证金
int
TraderApi_ReqQryInvestorProductGroupMargin(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInvestorProductGroupMarginField*
        pQryInvestorProductGroupMargin,
    int nRequestID)
{
    return p->ReqQryInvestorProductGroupMargin(pQryInvestorProductGroupMargin, nRequestID);
}

// 请求查询交易所保证金率
int
TraderApi_ReqQryExchangeMarginRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryExchangeMarginRateField* pQryExchangeMarginRate,
    int nRequestID)
{
    return p->ReqQryExchangeMarginRate(pQryExchangeMarginRate, nRequestID);
}

// 请求查询交易所调整保证金率
int
TraderApi_ReqQryExchangeMarginRateAdjust(
    CThostFtdcTraderApi* p,
    CThostFtdcQryExchangeMarginRateAdjustField* pQryExchangeMarginRateAdjust,
    int nRequestID)
{
    return p->ReqQryExchangeMarginRateAdjust(pQryExchangeMarginRateAdjust, nRequestID);
}

// 请求查询汇率
int
TraderApi_ReqQryExchangeRate(CThostFtdcTraderApi* p,
                             CThostFtdcQryExchangeRateField* pQryExchangeRate,
                             int nRequestID)
{
    return p->ReqQryExchangeRate(pQryExchangeRate, nRequestID);
}

// 请求查询二级代理操作员银期权限
int
TraderApi_ReqQrySecAgentACIDMap(
    CThostFtdcTraderApi* p,
    CThostFtdcQrySecAgentACIDMapField* pQrySecAgentACIDMap, int nRequestID)
{
    return p->ReqQrySecAgentACIDMap(pQrySecAgentACIDMap, nRequestID);
}

// 请求查询产品报价汇率
int
TraderApi_ReqQryProductExchRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryProductExchRateField* pQryProductExchRate, int nRequestID)
{
    return p->ReqQryProductExchRate(pQryProductExchRate, nRequestID);
}

// 请求查询产品组
int
TraderApi_ReqQryProductGroup(CThostFtdcTraderApi* p,
                             CThostFtdcQryProductGroupField* pQryProductGroup,
                             int nRequestID)
{
    return p->ReqQryProductGroup(pQryProductGroup, nRequestID);
}

// 请求查询做市商合约手续费率
int
TraderApi_ReqQryMMInstrumentCommissionRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMMInstrumentCommissionRateField*
        pQryMMInstrumentCommissionRate,
    int nRequestID)
{
    return p->ReqQryMMInstrumentCommissionRate(pQryMMInstrumentCommissionRate, nRequestID);
}

// 请求查询做市商期权合约手续费
int
TraderApi_ReqQryMMOptionInstrCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryMMOptionInstrCommRateField* pQryMMOptionInstrCommRate,
    int nRequestID)
{
    return p->ReqQryMMOptionInstrCommRate(pQryMMOptionInstrCommRate, nRequestID);
}

// 请求查询报单手续费
int
TraderApi_ReqQryInstrumentOrderCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryInstrumentOrderCommRateField* pQryInstrumentOrderCommRate,
    int nRequestID)
{
    return p->ReqQryInstrumentOrderCommRate(pQryInstrumentOrderCommRate, nRequestID);
}

// 请求查询期权交易成本
int
TraderApi_ReqQryOptionInstrTradeCost(
    CThostFtdcTraderApi* p,
    CThostFtdcQryOptionInstrTradeCostField* pQryOptionInstrTradeCost,
    int nRequestID)
{
    return p->ReqQryOptionInstrTradeCost(pQryOptionInstrTradeCost, nRequestID);
}

// 请求查询期权合约手续费
int
TraderApi_ReqQryOptionInstrCommRate(
    CThostFtdcTraderApi* p,
    CThostFtdcQryOptionInstrCommRateField* pQryOptionInstrCommRate,
    int nRequestID)
{
    return p->ReqQryOptionInstrCommRate(pQryOptionInstrCommRate, nRequestID);
}

// 请求查询执行宣告
int
TraderApi_ReqQryExecOrder(CThostFtdcTraderApi* p,
                          CThostFtdcQryExecOrderField* pQryExecOrder,
                          int nRequestID)
{
    return p->ReqQryExecOrder(pQryExecOrder, nRequestID);
}

// 请求查询询价
int
TraderApi_ReqQryForQuote(CThostFtdcTraderApi* p,
                         CThostFtdcQryForQuoteField* pQryForQuote,
                         int nRequestID)
{
    return p->ReqQryForQuote(pQryForQuote, nRequestID);
}

// 请求查询报价
int
TraderApi_ReqQryQuote(CThostFtdcTraderApi* p,
                      CThostFtdcQryQuoteField* pQryQuote, int nRequestID)
{
    return p->ReqQryQuote(pQryQuote, nRequestID);
}

// 请求查询组合合约安全系数
int
TraderApi_ReqQryCombInstrumentGuard(
    CThostFtdcTraderApi* p,
    CThostFtdcQryCombInstrumentGuardField* pQryCombInstrumentGuard,
    int nRequestID)
{
    return p->ReqQryCombInstrumentGuard(pQryCombInstrumentGuard, nRequestID);
}

// 请求查询申请组合
int
TraderApi_ReqQryCombAction(CThostFtdcTraderApi* p,
                           CThostFtdcQryCombActionField* pQryCombAction,
                           int nRequestID)
{
    return p->ReqQryCombAction(pQryCombAction, nRequestID);
}

// 请求查询转帐流水
int
TraderApi_ReqQryTransferSerial(
    CThostFtdcTraderApi* p,
    CThostFtdcQryTransferSerialField* pQryTransferSerial, int nRequestID)
{
    return p->ReqQryTransferSerial(pQryTransferSerial, nRequestID);
}

// 请求查询银期签约关系
int
TraderApi_ReqQryAccountregister(
    CThostFtdcTraderApi* p,
    CThostFtdcQryAccountregisterField* pQryAccountregister, int nRequestID)
{
    return p->ReqQryAccountregister(pQryAccountregister, nRequestID);
}

// 请求查询签约银行
int
TraderApi_ReqQryContractBank(CThostFtdcTraderApi* p,
                             CThostFtdcQryContractBankField* pQryContractBank,
                             int nRequestID)
{
    return p->ReqQryContractBank(pQryContractBank, nRequestID);
}

// 请求查询预埋单
int
TraderApi_ReqQryParkedOrder(CThostFtdcTraderApi* p,
                            CThostFtdcQryParkedOrderField* pQryParkedOrder,
                            int nRequestID)
{
    return p->ReqQryParkedOrder(pQryParkedOrder, nRequestID);
}

// 请求查询预埋撤单
int
TraderApi_ReqQryParkedOrderAction(
    CThostFtdcTraderApi* p,
    CThostFtdcQryParkedOrderActionField* pQryParkedOrderAction, int nRequestID)
{
    return p->ReqQryParkedOrderAction(pQryParkedOrderAction, nRequestID);
}

// 请求查询交易通知
int
TraderApi_ReqQryTradingNotice(
    CThostFtdcTraderApi* p, CThostFtdcQryTradingNoticeField* pQryTradingNotice,
    int nRequestID)
{
    return p->ReqQryTradingNotice(pQryTradingNotice, nRequestID);
}

// 请求查询经纪公司交易参数
int
TraderApi_ReqQryBrokerTradingParams(
    CThostFtdcTraderApi* p,
    CThostFtdcQryBrokerTradingParamsField* pQryBrokerTradingParams,
    int nRequestID)
{
    return p->ReqQryBrokerTradingParams(pQryBrokerTradingParams, nRequestID);
}

// 请求查询经纪公司交易算法
int
TraderApi_ReqQryBrokerTradingAlgos(
    CThostFtdcTraderApi* p,
    CThostFtdcQryBrokerTradingAlgosField* pQryBrokerTradingAlgos,
    int nRequestID)
{
    return p->ReqQryBrokerTradingAlgos(pQryBrokerTradingAlgos, nRequestID);
}

// 请求查询监控中心用户令牌
int
TraderApi_ReqQueryCFMMCTradingAccountToken(
    CThostFtdcTraderApi* p,
    CThostFtdcQueryCFMMCTradingAccountTokenField*
        pQueryCFMMCTradingAccountToken,
    int nRequestID)
{
    return p->ReqQueryCFMMCTradingAccountToken(pQueryCFMMCTradingAccountToken, nRequestID);
}

// 期货发起银行资金转期货请求
int
TraderApi_ReqFromBankToFutureByFuture(CThostFtdcTraderApi* p,
                                      CThostFtdcReqTransferField* pReqTransfer,
                                      int nRequestID)
{
    return p->ReqFromBankToFutureByFuture(pReqTransfer, nRequestID);
}

// 期货发起期货资金转银行请求
int
TraderApi_ReqFromFutureToBankByFuture(CThostFtdcTraderApi* p,
                                      CThostFtdcReqTransferField* pReqTransfer,
                                      int nRequestID)
{
    return p->ReqFromFutureToBankByFuture(pReqTransfer, nRequestID);
}

// 期货发起查询银行余额请求
int
TraderApi_ReqQueryBankAccountMoneyByFuture(
    CThostFtdcTraderApi* p, CThostFtdcReqQueryAccountField* pReqQueryAccount,
    int nRequestID)
{
    return p->ReqQueryBankAccountMoneyByFuture(pReqQueryAccount, nRequestID);
}
}
