/// 策略模块
#[macro_use]
mod mac;

pub mod indicator;
pub mod kline;
pub mod stats;
pub mod strategy;

pub use strategy::Strategy;

#[derive(Debug, Clone)]
pub struct Delta {
    /// 保存当前价格与目标价格的差值
    delta: Option<f64>,
}

impl Delta {
    pub fn new() -> Self {
        Delta { delta: None }
    }

    pub fn take(&mut self) -> Option<f64> {
        self.delta.take()
    }

    /// 测试价格是否发生交叉, 即触发目标价格
    pub fn crossup(&mut self, current: f64, target: f64) -> bool {
        let delta_ = current - target;
        if let Some(_delta) = self.delta {
            self.delta.replace(delta_);
            _delta * delta_ < 0.
        } else {
            self.delta.replace(delta_);
            false
        }
    }
}
