use super::*;

use std::{future::Future, sync::Arc};

/// Type for tasks spawned by the executor.
#[derive(PartialEq)]
pub enum TaskType {
    /// Regular non-blocking futures. Polling the task is expected to be a lightweight operation.
    Async,
    /// The task might perform a lot of expensive CPU operations and/or call `thread::sleep`.
    Blocking,
}

#[derive(Clone)]
pub struct TaskExecutor(Arc<dyn Fn(SomeFuture<()>, TaskType) -> JoinFuture<()> + Send + Sync>);

impl std::fmt::Debug for TaskExecutor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "TaskExecutor")
    }
}

impl<F, FUT> std::convert::From<F> for TaskExecutor
where
    F: Fn(SomeFuture<()>, TaskType) -> FUT + Send + Sync + 'static,
    FUT: Future<Output = ()> + Send + 'static,
{
    fn from(func: F) -> Self {
        Self(Arc::new(move |fut, tt| Box::pin(func(fut, tt))))
    }
}

impl TaskExecutor {
    /// Spawns a new asynchronous task.
    pub fn spawn(&self, future: SomeFuture<()>, task_type: TaskType) -> JoinFuture<()> {
        self.0(future, task_type)
    }
}
