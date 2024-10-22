with t1 as (select * from ticks where id = 'fu2305' and time > DATE '2023-03-16' order by ts)
   , t2 as (select * from ticks where id = 'fu2309' and time > DATE '2023-03-16' order by ts)

select
  t1.id || '-' || t2.id, t1.time, (t1.tick->'LastPrice')::float - (t2.tick->'LastPrice')::float diff

from t1 inner join t2 on t1.time = t2.time

order by t1.ts
