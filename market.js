const {
  ctp_version,
  NodeCTPMarket,
} = require('./ctp/node');

const market = new NodeCTPMarket()

market.register_front("")

market.on('rtn_depth_market_data', ({ LastPrice: price, AskPrice1: ask, AskVolume1: ask_vol, BidPrice1: bid, BidVolume1: bid_vol, ...other }) => {
  // console.log(other)
  // console.log(new TextDecoder().decode(new Uint8Array(other.UpdateTime)))
  console.log(new Date(),
    "价:", price,
    "买:", bid === Number.MAX_VALUE ? "无" : bid, bid_vol,
    "卖:", ask === Number.MAX_VALUE ? "无" : ask, ask_vol,
  )
})

market.on('front_connected', v => {
  console.log("Market Front Connected")

  market.req_user_login({ broker_id: "9999" }, e => {
    if (!e) {
      console.log("CTP VERSION:", ctp_version());
      console.log("Trading Day:", market.get_trading_day())

      market.subscribe_market_data(["bu2412", "bu2501"], e => {
        if (!e) {
          console.log("Subscribe Market Data: bu2412, bu2501")

          // market.unsubscribe_market_data(["ag2012"], e => {
          //   if (!e) {
          //     console.log("Unsubscribe Success")
          //   }
          // })
        }
      })
    }
  })
})

market.init()
