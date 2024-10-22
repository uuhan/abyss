create materialized view kline_1m with (timescaledb.continuous) as   select

  time_bucket('1 minute', time) as time_1m,
  id,

  max((tick->'LastPrice')::float) as high,
  first((tick->'LastPrice')::float, ts) as open,
  last((tick->'LastPrice')::float, ts) as close,
  min((tick->'LastPrice')::float) as low,
  max((tick->'Volume')::int) - min((tick->'Volume')::int) as volume,

  first(ts, time) as ts

from ticks
group by time_1m, id

with no data
