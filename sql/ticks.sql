-- CREATE ROLE abyss SUPERUSER LOGIN PASSWORD 'abyss';
-- CREATE DATABASE abyss WITH OWNER abyss;
-- ALTER DATABASE abyss SET TIMEZONE TO 'Asia/Shanghai';

CREATE EXTENSION IF NOT EXISTS timescaledb;

CREATE TABLE IF NOT EXISTS ticks (
    id      text not null,
    time    timestamp with time zone not null,
    tick    jsonb,
    ts      timestamp with time zone not null default now()
);

-- 创建索引
create index ticks_id_idx on ticks using HASH(id);
create index ticks_ts_idx on ticks using BTREE(ts ASC);
-- NB: 下面这个需要创建完之后手动执行
-- create index ticks_ts_id_brin_idx on ticks using brin(ts, id) with (timescaledb.transaction_per_chunk, pages_per_range=1024, autosummarize=on);

-- 创建合约ID视图
create materialized view instruments as (select id from ticks group by id) with no data;

-- 创建超表
SELECT create_hypertable('ticks', 'time', chunk_time_interval => interval '7 day');
-- SELECT add_dimension('ticks', 'id', chunk_time_interval => 4);

-- 启用列压缩
ALTER TABLE ticks SET(
  timescaledb.compress,
  timescaledb.compress_segmentby = 'id',
  timescaledb.compress_orderby = 'ts ASC'
);

-- 设置压缩策略
SELECT add_compression_policy('ticks', INTERVAL '7 days');

