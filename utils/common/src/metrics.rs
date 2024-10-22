use lazy_static::lazy_static;
use prometheus::{
    core::{AtomicU64, GenericCounterVec, GenericGauge, Opts},
    Error as PrometheusError, Registry,
};

lazy_static! {
    pub static ref UNBOUNDED_CHANNELS_COUNTER : GenericCounterVec<AtomicU64> = GenericCounterVec::new(
        Opts::new("unbounded_channel_len", "Items in each mpsc::unbounded instance"),
        &["entity", "action"] // 'name of channel, send|received|dropped
    ).expect("Creating of statics doesn't fail. qed");

    // Blocking Task Metrics
    pub static ref BLOCKING_THREADS_TOTAL: GenericGauge<AtomicU64> =
        GenericGauge::new("blocking_threads_total", "Total number of blocking threads created")
        .expect("Creating of statics doesn't fail. qed");
    pub static ref BLOCKING_THREADS_IDLE: GenericGauge<AtomicU64> =
        GenericGauge::new("blocking_threads_idle", "Total number of idle threads")
        .expect("Creating of statics doesn't fail. qed");
}

/// Register the statics to report to registry
pub fn register_globals(registry: &Registry) -> Result<(), PrometheusError> {
    registry.register(Box::new(UNBOUNDED_CHANNELS_COUNTER.clone()))?;

    registry.register(Box::new(BLOCKING_THREADS_TOTAL.clone()))?;
    registry.register(Box::new(BLOCKING_THREADS_IDLE.clone()))?;

    Ok(())
}
