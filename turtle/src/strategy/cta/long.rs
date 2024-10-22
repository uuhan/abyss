#![allow(unused,non_snake_case)]
/// 趋势策略, 看多
use ctp::*;
use ctp::api::*;
use ctp::api::opts::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use circular_queue::CircularQueue;
use statrs::statistics::Statistics;

use crate::strategy::{StrategyResult, Strategy};
use crate::kline::Kline;

pub struct Long {
    db: sled::Db,
    // 合约
    instrument: String,
    // 开仓量
    volume: i32,
}

impl Long {
    pub fn new(db: sled::Db, instrument: String, volume: i32) -> Self {
        Long { db, instrument, volume }
    }
}

impl Strategy for Long {
    fn run(&mut self, trader: &CTPTrader) -> StrategyResult<()> {
        todo!()
    }
}
