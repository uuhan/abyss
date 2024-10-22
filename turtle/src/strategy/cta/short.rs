#![allow(unused,non_snake_case)]
/// 趋势策略, 看空
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
