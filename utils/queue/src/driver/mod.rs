// Queue Driver
use crate::*;

mod amqp;
mod redis;

pub trait DodoQueueDriver {
    /// 任务提交
    fn push(&self, task: Task) -> Result<()>;
    /// 结果拉取
    fn pull(&self, task: Task) -> Result<String>;
}
