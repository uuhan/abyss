create or replace function get_spread(id1 text, id2 text)
  returns table(
    "time" timestamp with time zone,
    "spread" float
  ) as
$$
  select f1.time, (f1.price - f2.price) as spread
    from (select time, (tick->'LastPrice')::float as price, ts from ticks where id = id1) f1
      join (select time, (tick->'LastPrice')::float as price, ts from ticks where id = id2) f2 on ((f1.time = f2.time))
    order by f1.ts;
$$ language sql;

create materialized view spread_fu2209_fu2301 as (
  with
    f1 as (
      select time, (tick->'LastPrice')::float price, ts from ticks where id = 'fu2209'
    ),

    f2 as (
      select time, (tick->'LastPrice')::float price, ts from ticks where id = 'fu2301'
    )

  select
    f1.time,
    (f1.price - f2.price) as spread,
    f1.ts
  from
    f1 join f2 on ((f1.time = f2.time))
  order by f1.ts
) with no data;
