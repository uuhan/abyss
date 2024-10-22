use crate::config::CONFIG;
use crate::endpoints::prometheus::init_prometheus;
use crate::error::AppResult;
use crate::metrics::*;
use abyss_common::task::*;
use tokio::runtime::Runtime;

use futures::{future, future::FutureExt, pin_mut, select, Future};

pub fn build_runtime() -> AppResult<Runtime> {
    Ok(tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(|| {
            TOKIO_THREADS_ALIVE.inc();
            TOKIO_THREADS_TOTAL.inc();
        })
        .on_thread_stop(|| {
            TOKIO_THREADS_ALIVE.dec();
        })
        .enable_all()
        .build()?)
}

#[must_use]
pub struct Runner {
    runtime: tokio::runtime::Runtime,
    factory: TaskManagerFactory,
    // 运行一些基础任务
    handler: SpawnTaskHandle,
}

impl Runner {
    pub fn new() -> AppResult<Self> {
        // create prometheus registry
        // create async task runner
        let runtime = build_runtime().expect("build_runtime() failed.");
        let handler = runtime.handle().clone();

        let executor = move |fut, task_type| match task_type {
            TaskType::Async => handler.spawn(fut).map(drop),
            TaskType::Blocking => handler
                .spawn_blocking(move || futures::executor::block_on(fut))
                .map(drop),
        };

        let prometheus = CONFIG
            .endpoints
            .prometheus
            .as_ref()
            .and_then(|conf| build_prometheus_registry(&conf, |_| Ok(())).ok())
            .flatten();
        let factory = TaskManagerFactory::new("sail", prometheus, executor.into())
            .expect("TaskManagerFactory::new() failed.");

        // Create a task spawn handler
        let handler = factory.instance.spawn_handle();
        if let Some(ref conf) = factory.prometheus_conf() {
            let address = conf.address;
            let registry = conf.registry.clone();
            // Initialize prometheus metric service
            handler.spawn("prometheus-endpoint", move || {
                let address = address.clone();
                let registry = registry.clone();

                async move {
                    tracing::info!(?address, "Prometheus 采集服务启动");
                    init_prometheus(address, registry).await
                }
            });
        }

        Ok(Self {
            runtime,
            factory,
            handler,
        })
    }

    pub fn print_system_info(&self) {}

    pub fn block_after_init(
        mut self,
        init: impl FnOnce(&mut TaskManagerFactory) -> AppResult<()>,
    ) -> AppResult<()> {
        self.print_system_info();

        // Do some initialization
        init(&mut self.factory)?;

        // Catch system signal
        let res = self
            .runtime
            .block_on(signal_wrapper(self.factory.instance.future().fuse()));

        // Do some cleanup
        let _ = self
            .runtime
            .block_on(self.factory.instance.clean_shutdown());

        Ok(res?)
    }
}

async fn signal_wrapper<F, E>(func: F) -> Result<(), E>
where
    E: std::error::Error,
    F: Future<Output = Result<(), E>> + future::FusedFuture,
{
    use tokio::signal::unix::{signal, SignalKind};

    let mut stream_int = signal(SignalKind::interrupt()).unwrap();
    let mut stream_term = signal(SignalKind::terminate()).unwrap();

    let t1 = stream_int.recv().fuse();
    let t2 = stream_term.recv().fuse();
    let t3 = func;

    pin_mut!(t1, t2, t3);

    select! {
        _ = t1 => {},
        _ = t2 => {},
        res = t3 => res?,
    }

    Ok(())
}
