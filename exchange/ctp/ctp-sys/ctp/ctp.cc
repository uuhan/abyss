#include "ThostFtdcMdApi.h"
#include <ctp.h>

extern "C" {
CTPMarketSpi *
CTPMarketSpi_New()
{
    return new CTPMarketSpi;
}

CTPTraderSpi *
CTPTraderSpi_New()
{
    return new CTPTraderSpi;
}
}
