//! 任务管理器
//! 参考自: https://github.com/paritytech/substrate/blob/master/client/service/src/task_manager/mod.rs
mod executor;
mod prometheus_future;

pub mod signal;

use crate::{config::PrometheusConfig, error::*, metrics::*};
use signal::*;

use futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use futures::{
    future::{join_all, pending, select, try_join_all, Either},
    sink::SinkExt,
    Future, FutureExt, StreamExt,
};
use parking_lot::{Condvar, Mutex};
use prometheus::{
    self,
    core::{AtomicU64 as U64, Collector, GenericCounterVec as CounterVec},
    exponential_buckets, Error as PrometheusError, HistogramOpts, HistogramVec, Opts, Registry,
};
use std::{
    error::Error as StdError, net::SocketAddr, ops::Deref, panic, pin::Pin,
    result::Result as StdResult, sync::Arc,
};
use tracing_futures::Instrument;

use abyss_subscriber::*;
pub use executor::{TaskExecutor, TaskType};

// some type alias
pub(super) type SomeFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
pub(super) type JoinFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

pub type TracingUnboundedSender<T> = UnboundedSender<T>;
pub type TracingUnboundedReceiver<T> = UnboundedReceiver<T>;

#[derive(Debug, Clone)]
pub struct Prometheus {
    pub address: SocketAddr,
    pub registry: Registry,
}

/// Create Prometheus Registry
pub fn build_prometheus_registry(
    config: &PrometheusConfig,
    register_customs: impl FnOnce(&Registry) -> StdResult<(), PrometheusError>,
) -> Result<Option<Prometheus>> {
    if !config.enabled() {
        return Ok(None);
    }

    let address: SocketAddr = config
        .address
        .parse()
        .expect("[catfish.endpoints.prometheus.address] parse error.");

    let registry = Registry::new_custom(Some("catfish".into()), None)?;

    if let Err(error) = register_globals(&registry) {
        tracing::error!(?error, "register globals failed.");
    }

    if let Err(error) = register_customs(&registry) {
        tracing::error!(?error, "register globals failed.");
    }

    // process metrics for linux system
    #[cfg(target_os = "linux")]
    {
        use prometheus::process_collector::ProcessCollector;
        let pc = ProcessCollector::for_self();
        if let Err(error) = registry.register(Box::new(pc)) {
            tracing::error!(?error, "register linux metrics failed.");
        }
    }

    Ok(Some(Prometheus { address, registry }))
}

/// Alias `mpsc::unbounded`
pub fn tracing_unbounded<T>(
    _key: &'static str,
) -> (TracingUnboundedSender<T>, TracingUnboundedReceiver<T>) {
    mpsc::unbounded()
}

pub struct TaskCondition(Mutex<usize>, Condvar);

impl TaskCondition {
    pub fn new() -> Self {
        TaskCondition(Mutex::new(0), Condvar::new())
    }

    /// 运行数加一
    pub fn inc(&self) {
        let mut count = self.0.lock();
        *count = *count + 1;
        self.1.notify_all();
    }

    /// 运行数减一
    pub fn dec(&self) {
        let mut count = self.0.lock();
        *count = *count - 1;
        self.1.notify_all();
    }

    /// 同步阻塞线程
    pub fn check(&self, upper: usize) {
        let mut count = self.0.lock();

        while *count >= upper {
            self.1.wait(&mut count)
        }
    }
}

#[allow(unused)]
pub struct TaskHandler {
    condition: Arc<TaskCondition>,
    pool_size: Option<usize>,
    closed: bool,
}

impl TaskHandler {
    /// 阻塞等待任务执行结束
    /// 1. 当 `SpawnTaskHandle` 关闭的时候直接返回
    /// 2. 当 `SpawnTaskHandle.pool_size` 为空时直接返回
    /// 3. 当 `SpawnTaskHandle.condition` 满足当前任务数小于指定上限时返回
    pub fn join(&self) {
        if self.closed {
            return;
        }

        if let Some(upper) = self.pool_size {
            self.condition.check(upper);
        }
    }
}

/// 任务信号
#[derive(Clone, Debug)]
pub enum TaskSignal {
    /// 退出任务
    Exit,
    /// 重启任务
    Restart,
    /// 暂停任务
    Stop,
    /// 继续任务
    Continue,
}

/// An handle for spawning tasks in the service.
#[derive(Clone)]
pub struct SpawnTaskHandle {
    on_exit: signal::Exit,
    executor: TaskExecutor,
    metrics: Option<Metrics>,
    task_notifier: TracingUnboundedSender<JoinFuture<()>>,
    subscribers: Arc<Subscribers<TaskSignal>>,
    /// 这个 Handler 执行任务数状态
    condition: Arc<TaskCondition>,
    /// 如果非空，值为这个 Handler 可执行任务数上限
    pool_size: Option<usize>,
}

impl SpawnTaskHandle {
    /// Spawns the given task with the given name.
    ///
    /// Note that the `name` is a `&'static str`. The reason for this choice is that statistics
    /// about this task are getting reported to the Prometheus endpoint (if enabled), and that
    /// therefore the set of possible task names must be bounded.
    ///
    /// In other words, it would be a bad idea for someone to do for example
    /// `spawn(format!("{:?}", some_public_key))`.
    pub fn spawn<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner::<T, B, E>(name.as_ref(), task_builder, TaskType::Async, false)
    }

    /// Spawns scoped task, will be terminated when `self` is dropped.
    pub fn scoped<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner(name.as_ref(), task_builder, TaskType::Async, true)
    }

    /// Spawns the blocking task with the given name. See also `spawn`.
    pub fn spawn_blocking<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner(name.as_ref(), task_builder, TaskType::Blocking, false)
    }

    /// Helper function that implements the spawning logic. See `spawn` and `spawn_blocking`.
    fn spawn_inner<T, B, E>(
        &self,
        name: &str,
        task_builder: B,
        task_type: TaskType,
        scoped: bool,
    ) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        if self.task_notifier.is_closed() {
            tracing::debug!(
                task = name,
                "Attempt to spawn a new task has been prevented."
            );
            return TaskHandler {
                condition: self.condition.clone(),
                pool_size: self.pool_size.clone(),
                closed: true,
            };
        }

        tracing::info!("[{}] start to run...", name);
        self.condition.inc();

        // 发送终止信号，停止旧任务
        if let Some(broadcast) = self.subscribers.exact(name) {
            tracing::info!(task = name, "broadcast to terminate.");
            broadcast.complete(&TaskSignal::Exit);
        }

        // combined exit signal
        let mut on_signal = select(
            self.on_exit.clone(),
            self.subscribers.register(name.as_bytes()),
        )
        .fuse();

        // Note that we increase the started counter here and not within the future. This way,
        // we could properly visualize on Prometheus situations where the spawning doesn't work.
        if let Some(metrics) = &self.metrics {
            metrics.tasks_spawned.with_label_values(&[name]).inc();
            // We do a dummy increase in order for the task to show up in metrics.
            metrics
                .tasks_ended
                .with_label_values(&[name, "finished"])
                .inc_by(0);
        }
        let metrics = self.metrics.clone();
        let name1 = name.to_owned();

        let condition_to_finish = self.condition.clone();

        let future = async move {
            let name = &name1;
            tracing::info!(task = name, "task is running.");

            if let Some(metrics) = metrics {
                // Add some wrappers around `task`.
                let task = {
                    let poll_duration = metrics.poll_duration.with_label_values(&[name]);
                    let poll_start = metrics.poll_start.with_label_values(&[name]);
                    let inner = prometheus_future::with_poll_durations(
                        poll_duration,
                        poll_start,
                        task_builder(),
                    );
                    // The logic of `AssertUnwindSafe` here is ok considering that we throw
                    // away the `Future` after it has panicked.
                    panic::AssertUnwindSafe(inner).catch_unwind()
                };

                futures::pin_mut!(task);

                loop {
                    match select(&mut on_signal, &mut task).await {
                        Either::Right((Err(payload), _)) => {
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "panic"])
                                .inc();
                            eprintln!("[{}] panic occured.", name);
                            tracing::error!("[{}] panic occured.", name);
                            panic::resume_unwind(payload);
                        }
                        Either::Right((Ok(Ok(())), _)) => {
                            tracing::info!(task = name, "task finished.");
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "finished"])
                                .inc();
                        }
                        Either::Right((Ok(Err(e)), _)) => {
                            tracing::error!("[{}] {}", name, e);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "finished-exception"])
                                .inc();
                        }

                        // signal 事件触发
                        Either::Left((Either::Left(((), _)), _)) => {
                            // The `on_exit` has triggered.
                            tracing::warn!("[{}] 任务退出.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "interrupted"])
                                .inc();
                        }

                        // 任务退出, 由 subscribers 触发
                        Either::Left((Either::Right((Some(TaskSignal::Exit), _)), _)) => {
                            tracing::warn!("[{}] 任务停止.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "exited"])
                                .inc();
                        }

                        // 任务重启, 由 subscribers 触发
                        Either::Left((Either::Right((Some(TaskSignal::Restart), _)), _)) => {
                            tracing::warn!("[{}] 任务重启.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "restarted"])
                                .inc();
                        }

                        // 任务重启, 由 subscribers 触发
                        Either::Left((Either::Right((Some(TaskSignal::Stop), _)), _)) => {
                            tracing::warn!("[{}] 任务暂停.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "stopped"])
                                .inc();
                        }

                        // 任务继续, 由 subscribers 触发
                        Either::Left((Either::Right((Some(TaskSignal::Continue), _)), _)) => {
                            tracing::warn!("[{}] 任务重启.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "continued"])
                                .inc();
                        }

                        // SpawnTaskHandle.subscribers 发生析构
                        // 如果是 scoped 的任务, 则退出, 否则忽略此事件
                        Either::Left((Either::Right((None, _)), _)) => {
                            if !scoped {
                                continue;
                            }
                            tracing::info!("[{}] exited.", name);
                            metrics
                                .tasks_ended
                                .with_label_values(&[name, "interrupted"])
                                .inc();
                        }
                    }

                    // 正常退出
                    break;
                }
            } else {
                let task = task_builder();
                futures::pin_mut!(task);
                loop {
                    match select(&mut on_signal, &mut task).await {
                        Either::Left((Either::Right((None, _)), _)) => {
                            if !scoped {
                                continue;
                            }
                        }
                        _ => {}
                    }

                    break;
                }
            }

            condition_to_finish.dec();
        };

        let join_handle = self
            .executor
            .spawn(future.in_current_span().boxed(), task_type);
        let mut task_notifier = self.task_notifier.clone();

        let _ = self.executor.spawn(
            Box::pin(async move {
                if let Err(err) = task_notifier.send(join_handle).await {
                    // if start failed, remove the task signal
                    tracing::error!("Could not send spawned task handle to queue: {}", err);
                }
            }),
            TaskType::Async,
        );

        TaskHandler {
            condition: self.condition.clone(),
            pool_size: self.pool_size.clone(),
            closed: false,
        }
    }

    /// watch task status
    pub fn watch_task(&self, name: impl AsRef<str>) -> Subscriber<TaskSignal> {
        self.subscribers.register(name.as_ref().as_bytes())
    }

    /// list current tasks
    pub fn contains_task(&self, name: impl AsRef<str>) -> bool {
        self.subscribers.exact(name.as_ref()).is_some()
    }

    /// stop task by name
    pub fn stop_task(&self, name: impl AsRef<str>) {
        let name = name.as_ref();
        if let Some(broadcast) = self.subscribers.exact(name) {
            tracing::info!(task = name, "broadcast to terminate.");
            broadcast.complete(&TaskSignal::Exit);
        } else {
            tracing::warn!(task = name, "task not running.");
        }
    }

    /// list tasks
    pub fn tasks(&self) -> Vec<Vec<u8>> {
        self.subscribers.watched()
    }
}

/// A wrapper over `SpawnTaskHandle` that will notify a receiver whenever any
/// task spawned through it fails. The service should be on the receiver side
/// and will shut itself down whenever it receives any message, i.e. an
/// essential task has failed.
#[derive(Clone)]
pub struct SpawnEssentialTaskHandle {
    essential_failed_tx: TracingUnboundedSender<()>,
    inner: SpawnTaskHandle,
}

impl SpawnEssentialTaskHandle {
    /// Creates a new `SpawnEssentialTaskHandle`.
    pub fn new(
        essential_failed_tx: TracingUnboundedSender<()>,
        spawn_task_handle: SpawnTaskHandle,
    ) -> SpawnEssentialTaskHandle {
        SpawnEssentialTaskHandle {
            essential_failed_tx,
            inner: spawn_task_handle,
        }
    }

    /// Spawns the given task with the given name.
    ///
    /// See also [`SpawnTaskHandle::spawn`].
    pub fn spawn<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner(name.as_ref(), task_builder, TaskType::Async, false)
    }

    /// Spawns scoped task, will be terminated when `self` is dropped.
    pub fn scoped<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner(name.as_ref(), task_builder, TaskType::Async, true)
    }

    /// Spawns the blocking task with the given name.
    ///
    /// See also [`SpawnTaskHandle::spawn_blocking`].
    pub fn spawn_blocking<T, B, E>(&self, name: impl AsRef<str>, task_builder: B) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        self.spawn_inner(name.as_ref(), task_builder, TaskType::Blocking, false)
    }

    fn spawn_inner<T, B, E>(
        &self,
        name: &str,
        task_builder: B,
        task_type: TaskType,
        scoped: bool,
    ) -> TaskHandler
    where
        E: StdError,
        T: Future<Output = StdResult<(), E>> + Send + 'static,
        B: Fn() -> T + Send + 'static,
    {
        let essential_failed = self.essential_failed_tx.clone();
        let name1 = name.to_owned();
        self.inner.spawn_inner(
            name,
            move || {
                let essential_failed = essential_failed.clone();
                let name1 = name1.clone();
                std::panic::AssertUnwindSafe(task_builder())
                    .catch_unwind()
                    .map(move |_| {
                        tracing::error!(
                            "Essential task `{}` failed. Shutting down service.",
                            name1
                        );
                        let _ = essential_failed.close_channel();
                        Ok::<(), E>(())
                    })
            },
            task_type,
            scoped,
        )
    }
}

/// Provide `TaskManager` instance
pub struct TaskManagerFactory {
    pub instance: TaskManager,
    prometheus_conf: Option<Prometheus>,
    executor: TaskExecutor,
}

impl TaskManagerFactory {
    pub fn new(
        label: &'static str,
        prometheus_conf: Option<Prometheus>,
        executor: TaskExecutor,
    ) -> Result<Self> {
        let instance = TaskManager::new(
            label,
            executor.clone(),
            prometheus_conf.as_ref().map(|conf| &conf.registry),
        )?;

        Ok(Self {
            instance,
            prometheus_conf,
            executor,
        })
    }

    /// Prometheus configuration
    pub fn prometheus_conf(&self) -> Option<&Prometheus> {
        self.prometheus_conf.as_ref()
    }

    /// Create TaskManager with label
    pub fn create(&self, label: &'static str) -> Result<TaskManager> {
        let manager = TaskManager::new(
            label,
            self.executor.clone(),
            self.prometheus_conf.as_ref().map(|conf| &conf.registry),
        )?;

        Ok(manager)
    }

    pub fn collect(&mut self, child: TaskManager) {
        self.instance.add_child(child);
    }
}

impl Deref for TaskManagerFactory {
    type Target = TaskManager;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

/// Helper struct to manage background/async tasks in Service.
pub struct TaskManager {
    /// 结束任务接收器
    on_exit: signal::Exit,
    /// 结束任务发生器
    signal: Option<Signal>,
    /// How to spawn background tasks.
    executor: TaskExecutor,
    /// Prometheus metric where to report the polling times.
    metrics: Option<Metrics>,
    /// Send a signal when a spawned essential task has concluded. The next time
    /// the service future is polled it should complete with an error.
    essential_failed_tx: TracingUnboundedSender<()>,
    /// A receiver for spawned essential-tasks concluding.
    essential_failed_rx: TracingUnboundedReceiver<()>,
    /// Things to keep alive until the task manager is dropped.
    keep_alive: Box<dyn std::any::Any + Send + Sync>,
    /// A sender to a stream of background tasks. This is used for the completion future.
    task_notifier: TracingUnboundedSender<JoinFuture<()>>,
    /// This future will complete when all the tasks are joined and the stream is closed.
    completion_future: JoinFuture<()>,
    /// A list of other `TaskManager`'s to terminate and gracefully shutdown when the parent
    /// terminates and gracefully shutdown. Also ends the parent `future()` if a child's essential
    /// task fails.
    children: Vec<TaskManager>,
}

impl TaskManager {
    pub fn new(
        label: &'static str,
        executor: TaskExecutor,
        registry: Option<&Registry>,
    ) -> Result<Self> {
        let (signal, on_exit) = signal();
        let (essential_failed_tx, essential_failed_rx) = tracing_unbounded("mpsc_essential_tasks");
        let metrics = registry.map(Metrics::register(label)).transpose()?;

        let (task_notifier, background_tasks) = tracing_unbounded("mpsc_background_tasks");
        let completion_future = executor.spawn(
            Box::pin(background_tasks.for_each_concurrent(None, |x| x)),
            TaskType::Async,
        );

        Ok(Self {
            on_exit,
            signal: Some(signal),
            executor,
            metrics,
            essential_failed_tx,
            essential_failed_rx,
            keep_alive: Box::new(()),
            task_notifier,
            completion_future,
            children: Vec::new(),
        })
    }

    /// Terminate this task-manager
    pub fn terminate(&mut self) {
        if let Some(signal) = self.signal.take() {
            let _ = signal.fire();
            // NOTE: this will prevent new tasks to be spawned
            self.task_notifier.close_channel();
            for child in self.children.iter_mut() {
                child.terminate();
            }
        }
    }

    /// Set what the task manager should keep alive, can be called multiple times.
    pub fn keep_alive<T: 'static + Send + Sync>(&mut self, to_keep_alive: T) {
        // allows this fn to safely called multiple times.
        use std::mem;
        let old = mem::replace(&mut self.keep_alive, Box::new(()));
        self.keep_alive = Box::new((to_keep_alive, old));
    }

    /// Register another TaskManager to terminate and gracefully shutdown when the parent
    /// terminates and gracefully shutdown. Also ends the parent `future()` if a child's essential
    /// task fails. (But don't end the parent if a child's normal task fails.)
    pub fn add_child(&mut self, child: TaskManager) {
        self.children.push(child);
    }

    /// Send the signal for termination, prevent new tasks to be created, await for all the existing
    /// tasks to be finished and drop the object. You can consider this as an async drop.
    ///
    /// It's always better to call and await this function before exiting the process as background
    /// tasks may be running in the background. If the process exit and the background tasks are not
    /// cancelled, this will lead to objects not getting dropped properly.
    ///
    /// This is an issue in some cases as some of our dependencies do require that we drop all the
    /// objects properly otherwise it triggers a SIGABRT on exit.
    pub fn clean_shutdown(mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        self.terminate();
        let children_shutdowns = self.children.into_iter().map(|x| x.clean_shutdown());
        let keep_alive = self.keep_alive;
        let completion_future = self.completion_future;

        Box::pin(async move {
            join_all(children_shutdowns).await;
            completion_future.await;
            drop(keep_alive);
            Ok(())
        })
    }

    /// Get a handle for spawning tasks.
    pub fn spawn_handle(&self) -> SpawnTaskHandle {
        SpawnTaskHandle {
            on_exit: self.on_exit.clone(),
            executor: self.executor.clone(),
            metrics: self.metrics.clone(),
            task_notifier: self.task_notifier.clone(),
            subscribers: Arc::new(Subscribers::new()),
            condition: Arc::new(TaskCondition::new()),
            pool_size: None,
        }
    }

    /// Get a pooled handle for spawning tasks.
    pub fn spawn_pooled_handle(&self, pool_size: usize) -> SpawnTaskHandle {
        tracing::error!(pool_size, "spawn_pooled_handle size > 0");

        let pool_size = if pool_size > 0 { Some(pool_size) } else { None };

        SpawnTaskHandle {
            on_exit: self.on_exit.clone(),
            executor: self.executor.clone(),
            metrics: self.metrics.clone(),
            task_notifier: self.task_notifier.clone(),
            subscribers: Arc::new(Subscribers::new()),
            condition: Arc::new(TaskCondition::new()),
            pool_size,
        }
    }

    /// Get a handle for spawning essential tasks.
    pub fn spawn_essential_handle(&self) -> SpawnEssentialTaskHandle {
        SpawnEssentialTaskHandle::new(self.essential_failed_tx.clone(), self.spawn_handle())
    }

    pub fn future<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
        Box::pin(async move {
            let mut t1 = self.essential_failed_rx.next().fuse();
            let mut t2 = self.on_exit.clone().fuse();
            let mut t3 = try_join_all(
                self.children
                    .iter_mut()
                    .map(|x| x.future())
                    // Never end this future if there is no error because if there is no children,
                    // it must not stop
                    .chain(std::iter::once(pending().boxed())),
            )
            .fuse();

            futures::select! {
                _ = t1 => Err(Error::Error("Essential task failed.")),
                _ = t2 => Ok(()),
                res = t3 => Err(res.map(|_| ()).expect_err("this future never ends; qed")),
            }
        })
    }
}

#[derive(Clone)]
struct Metrics {
    poll_duration: HistogramVec,
    poll_start: CounterVec<U64>,
    tasks_spawned: CounterVec<U64>,
    tasks_ended: CounterVec<U64>,
}

impl Metrics {
    fn register(label: &'static str) -> impl FnOnce(&Registry) -> Result<Self> {
        move |registry: &Registry| -> Result<Self> {
            Ok(Self {
          poll_duration: register(HistogramVec::new(
            HistogramOpts {
              common_opts: Opts::new(
                format!("{}_tasks_polling_duration", label),
                "Duration in seconds of each invocation of Future::poll"
              ),
              buckets: exponential_buckets(0.001, 4.0, 9)
                .expect("function parameters are constant and always valid; qed"),
            },
            &["task_name"]
          )?, registry)?,
          poll_start: register(CounterVec::new(
            Opts::new(
              format!("{}_tasks_polling_started_total", label),
              "Total number of times we started invoking Future::poll"
            ),
            &["task_name"]
          )?, registry)?,
          tasks_spawned: register(CounterVec::new(
            Opts::new(
              format!("{}_tasks_spawned_total", label),
              "Total number of tasks that have been spawned on the Service"
            ),
            &["task_name"]
          )?, registry)?,
          tasks_ended: register(CounterVec::new(
            Opts::new(
              format!("{}_tasks_ended_total", label),
              "Total number of tasks for which Future::poll has returned Ready(()) or panicked"
            ),
            &["task_name", "reason"]
          )?, registry)?,
        })
        }
    }
}

fn register<T: Clone + Collector + 'static>(
    metric: T,
    registry: &Registry,
) -> StdResult<T, PrometheusError> {
    registry.register(Box::new(metric.clone()))?;
    Ok(metric)
}
