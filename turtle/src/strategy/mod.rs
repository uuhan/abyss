/// 策略模块
use ctp::*;

// mod cta;
// mod lua;
// mod mean_reversion;
// pub use mean_reversion::MeanReversion;

pub trait Strategy {
    fn run(&mut self, trader: &CTPTrader) -> Result<(), Box<dyn std::error::Error>>;
}
