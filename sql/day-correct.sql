-- 新 ticks 表不需要这种方式修正, ts 列即按照时间先后排序
create or replace function get_correct_price(selected text)
  returns table (
    id text,
    ts timestamp with time zone,
    price float
  )
language sql as $$
  select
    id,
    (case when ts::time > '20:00:00' then ts - interval '1 day' else ts end) ts,
    (tick->'LastPrice')::float price
  from ticks
  where
    id = selected
  order by ts
$$;

-- select f1.ts, (f1.price - f2.price) spread from get_correct_price('fu2209') f1 join get_correct_price('fu2301') f2 on f1.ts = f2.ts;
