SELECT prom_api.set_default_retention_period(INTERVAL '1 month');

select prom_api.set_metric_retention_period('catfish_process_resident_memory_bytes', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_polling_duration_sum', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_polling_duration_count', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_polling_started_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_virtual_memory_bytes', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_ctp_request_function_bucket', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_tokio_threads_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_max_fds', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_ctp_request_function_count', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_ended_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_trader_account_balance', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_blocking_threads_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_trader_account_available', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_polling_duration_bucket', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_open_fds', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_ctp_request_function_sum', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_threads', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_start_time_seconds', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_trader_account_commission', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_trader_account_position_profit', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_process_cpu_seconds_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_app_tasks_spawned_total', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_blocking_threads_idle', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_trader_account_profit', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_tokio_threads_alive', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_promise_thread_pool_size', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_pg_tick_write_duration_count', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_pg_tick_write_duration_bucket', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_pg_tick_write_duration_sum', INTERVAL '2 weeks');

select prom_api.set_metric_retention_period('catfish_market_data_rtn_duration_sum', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_market_data_rtn_duration_count', INTERVAL '2 weeks');
select prom_api.set_metric_retention_period('catfish_market_data_rtn_duration_bucket', INTERVAL '2 weeks');

select prom_api.set_metric_retention_period('catfish_market_data_tick_interest', INTERVAL '100 years');
select prom_api.set_metric_retention_period('catfish_market_data_tick_total', INTERVAL '100 years');
select prom_api.set_metric_retention_period('catfish_market_data_tick_price', INTERVAL '100 years');
select prom_api.set_metric_retention_period('catfish_market_data_tick_volume', INTERVAL '100 years');

