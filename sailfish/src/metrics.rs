use lazy_static::lazy_static;
use prometheus::{
    core::{
        AtomicF64, AtomicU64, Collector, GenericCounter, GenericCounterVec as CounterVec,
        GenericCounterVec, GenericGauge, GenericGaugeVec as GaugeVec, Opts,
    },
    exponential_buckets, Error as PrometheusError, Gauge, Histogram, HistogramOpts, Registry,
};

pub fn register<T: Clone + Collector + 'static>(
    metric: T,
    registry: &Registry,
) -> Result<T, PrometheusError> {
    registry.register(Box::new(metric.clone()))?;
    Ok(metric)
}

lazy_static! {
    // Tokio Metrics
    pub static ref TOKIO_THREADS_TOTAL: GenericCounter<AtomicU64> =
        GenericCounter::new("tokio_threads_total", "Total number of threads created")
        .expect("Creating of statics doesn't fail. qed");
    pub static ref TOKIO_THREADS_ALIVE: GenericGauge<AtomicU64> =
        GenericGauge::new("tokio_threads_alive", "Number of threads alive right now")
        .expect("Creating of statics doesn't fail. qed");
    pub static ref UNBOUNDED_CHANNELS_COUNTER : GenericCounterVec<AtomicU64> = GenericCounterVec::new(
        Opts::new("unbounded_channel_len", "Items in each mpsc::unbounded instance"),
        &["entity", "action"] // 'name of channel, send|received|dropped
    ).expect("Creating of statics doesn't fail. qed");

}

/// Register the statics to report to registry
pub fn register_globals(registry: &Registry) -> Result<(), PrometheusError> {
    registry.register(Box::new(TOKIO_THREADS_ALIVE.clone()))?;
    registry.register(Box::new(TOKIO_THREADS_TOTAL.clone()))?;
    registry.register(Box::new(UNBOUNDED_CHANNELS_COUNTER.clone()))?;

    Ok(())
}
