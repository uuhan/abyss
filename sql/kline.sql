create materialized view k1m(id, time, open, high, low, close, volume, open_interest, ts) with (timescaledb.continuous) as
  select

    id,
    time_bucket(interval '1 minute', time),

    first((tick->'LastPrice')::float, ts),
    max((tick->'LastPrice')::float),
    min((tick->'LastPrice')::float),
    last((tick->'LastPrice')::float, ts),

    last((tick->'Volume')::float, ts) - first((tick->'Volume')::float, ts),
    last((tick->'OpenInterest')::float, ts) - first((tick->'OpenInterest')::float, ts),

    first(ts, ts) as ts

  from ticks
  group by 1, 2

with no data;

select add_continuous_aggregate_policy(
  'k1m',
  start_offset => interval '1 day',
  end_offset => null,
  schedule_interval => interval '1 day'
);

create materialized view k1d(id, day, open, high, low, close, volume, open_interest) with (timescaledb.continuous) as
  select

    id,
    time_bucket(interval '1 day', time) as day,

    first((tick->'LastPrice')::float, ts) as open,
    max((tick->'LastPrice')::float) as high,
    min((tick->'LastPrice')::float) as low,
    last((tick->'LastPrice')::float, ts) as close,

    last((tick->'Volume')::float, ts) as volume,
    last((tick->'OpenInterest')::float, ts) as open_interest

  from ticks
  group by day, id

with no data;

select add_continuous_aggregate_policy(
  'k1d',
  start_offset => interval '1 week',
  end_offset => interval '1 day',
  schedule_interval => interval '1 day'
);
