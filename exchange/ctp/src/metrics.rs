use lazy_static::lazy_static;
use prometheus::{
    core::{
        AtomicF64, AtomicU64, Collector, GenericCounter, GenericCounterVec as CounterVec,
        GenericCounterVec, GenericGauge, GenericGaugeVec as GaugeVec, Opts,
    },
    exponential_buckets, Error as PrometheusError, Gauge, Histogram, HistogramOpts, HistogramVec,
    Registry,
};

lazy_static! {
    pub(crate) static ref CTP_REQ_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts {
            common_opts: Opts::new(
                "ctp_request_function",
                "Duration of ctp request function call."
            ),
            buckets: exponential_buckets(0.001, 4.0, 9)
                .expect("function parameters are constant and always valid."),
        },
        &["function"]
    )
    .expect("ctp metrics failed");
}

/// 注册 Prometheus 度量
pub fn register(registry: &Registry) -> Result<(), PrometheusError> {
    registry.register(Box::new(CTP_REQ_DURATION.clone()))?;

    Ok(())
}
