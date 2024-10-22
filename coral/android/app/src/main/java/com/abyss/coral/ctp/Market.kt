package com.abyss.coral.ctp

import com.abyss.coral.data.database.entities.Account
import com.sfit.ctp.thostmduserapi.*
import timber.log.Timber

class Market constructor(
        private val path: String,
        private val account: Account,
    ) {
    private val sMdApi: CThostFtdcMdApi by lazy {
        CThostFtdcMdApi.CreateFtdcMdApi(path)
    }

    init {
        sMdApi.RegisterSpi(object: CThostFtdcMdSpi() {
            override fun OnFrontConnected() {
                super.OnFrontConnected()
                Timber.i("OnFrontConnected")
                val field = CThostFtdcReqUserLoginField()
                field.brokerID = account.brokerID
                field.userID = account.userID
                field.password = account.password
                sMdApi.ReqUserLogin(field, 0)
            }

            override fun OnFrontDisconnected(reason: Int) {
                super.OnFrontDisconnected(reason)
                Timber.i("Disconnected: $reason")
            }

            override fun OnRspUserLogin(user: CThostFtdcRspUserLoginField, error: CThostFtdcRspInfoField, requestId: Int, isLast: Boolean) {
                super.OnRspUserLogin(user, error, requestId, isLast)
                Timber.i("OnRspUserLogin")
                val instruments = arrayOf("ag2102")
                sMdApi.SubscribeMarketData(instruments, instruments.size)
            }

            override fun OnRspError(error: CThostFtdcRspInfoField, requestId: Int, isLast: Boolean) {
                super.OnRspError(error, requestId, isLast)
                Timber.e(error.errorMsg)
            }

            override fun OnRspSubMarketData(
                    instrument: CThostFtdcSpecificInstrumentField,
                    error: CThostFtdcRspInfoField,
                    requestId: Int, isLast: Boolean)
            {
                super.OnRspSubMarketData(instrument, error, requestId, isLast)
                Timber.i("subscribe ${instrument.instrumentID}")
            }

            override fun OnRtnDepthMarketData(data: CThostFtdcDepthMarketDataField) {
                super.OnRtnDepthMarketData(data)
                Timber.i("${data.instrumentID}: price ${data.lastPrice}")
            }
        })
        sMdApi.RegisterFront("")
        sMdApi.Init()
    }

    fun tradingDay(): String {
        return sMdApi.GetTradingDay()
    }
}
