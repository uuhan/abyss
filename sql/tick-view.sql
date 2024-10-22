select
  (case when ts::time > '20:00:00' then ts - interval '1 day' else ts end) ts,
  id,
  (tick->'LastPrice')::float price,

  (tick->'AskPrice5')::float askp5,
  (tick->'AskPrice4')::float askp4,
  (tick->'AskPrice3')::float askp3,
  (tick->'AskPrice2')::float askp2,
  (tick->'AskPrice1')::float askp1,
  (tick->'BidPrice1')::float bidp1,
  (tick->'BidPrice2')::float bidp2,
  (tick->'BidPrice3')::float bidp3,
  (tick->'BidPrice4')::float bidp4,
  (tick->'BidPrice5')::float bidp5,

  (tick->'AskVolume5')::float askv5,
  (tick->'AskVolume4')::float askv4,
  (tick->'AskVolume3')::float askv3,
  (tick->'AskVolume2')::float askv2,
  (tick->'AskVolume1')::float askv1,
  (tick->'BidVolume1')::float bidv1,
  (tick->'BidVolume2')::float bidv2,
  (tick->'BidVolume3')::float bidv3,
  (tick->'BidVolume4')::float bidv4,
  (tick->'BidVolume5')::float bidv5,

  (tick->'OpenInterest')::int open_interest,
  (tick->'Volume')::int volume,

  serial
from ticks;
