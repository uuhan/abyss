package com.abyss.coral.ctp

import com.sfit.ctp.thostmduserapi.*
import timber.log.Timber

class MdSpi: CThostFtdcMdSpi() {
    override fun OnFrontConnected() {
        super.OnFrontConnected()
        Timber.i("connected")
    }

    override fun OnRspError(error: CThostFtdcRspInfoField, requestId: Int, isLast: Boolean) {
        super.OnRspError(error, requestId, isLast)
    }

    override fun OnRspUserLogin(login: CThostFtdcRspUserLoginField, error: CThostFtdcRspInfoField, requestId: Int, isLast: Boolean) {
        super.OnRspUserLogin(login, error, requestId, isLast)
    }

    override fun OnRspSubMarketData(data: CThostFtdcSpecificInstrumentField, error: CThostFtdcRspInfoField, requestId: Int, isLast: Boolean) {
        super.OnRspSubMarketData(data, error, requestId, isLast)
    }

    override fun OnRtnDepthMarketData(data: CThostFtdcDepthMarketDataField) {
        super.OnRtnDepthMarketData(data)

        Timber.i(data.lastPrice.toString())
    }
}