#![allow(non_snake_case)]
/// 均值回归策略
/// 本策略期望价格对于均线会回归
/// 如果触及了过涨点则开空, 触及了过跌点则开多
/// 于均线处平仓
use ctp::*;
use ctp::api::*;
use ctp::api::opts::*;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use super::StrategyResult;
use super::Strategy;
use circular_queue::CircularQueue;
use statrs::statistics::Statistics;

use crate::kline::Kline;

pub struct MeanReversion {
    db: sled::Db,
    // 合约
    instrument: String,
    // 开仓量
    volume: i32,
    // 开空位置, 即上涨比例
    short: f64,
    // 开多位置, 即下跌比例
    long: f64,
    // 止损幅度
    alpha: f64,
    // 止盈幅度
    beta: f64,
    // 动态止损比例
    gamma: f64,
    // 最大止损次数
    count: i32,
    // 止损价格
    loss_price: Option<f64>,
    // 止损次数
    loss_count: i32,
    // 平仓价格
    close_price: Option<f64>,
    // 止损价格
    touch_price: Option<f64>,
    // 开多仓价格
    long_price: Option<f64>,
    // 开空仓价格
    short_price: Option<f64>,
    // 一段行情
    queue: CircularQueue::<Kline>,
    // 止损之后的一段行情
    queue_after_loss: CircularQueue::<Kline>,
}

impl MeanReversion {
    pub fn new(
        db: sled::Db,
        instrument: String,
        volume: i32,
        short: f64,
        long: f64,
        alpha: f64,
        beta: f64,
        gamma: f64,
        count: i32) -> Self
    {
        MeanReversion {
            db,
            instrument,
            volume,
            short,
            long,
            alpha,
            beta,
            gamma,
            count,
            loss_price: None,
            loss_count: 0,
            close_price: None,
            touch_price: None,
            long_price: None,
            short_price: None,
            queue: CircularQueue::with_capacity(1200),
            queue_after_loss: CircularQueue::with_capacity(720),
        }
    }
}

impl Strategy for MeanReversion {
    fn run(&mut self, trader: &CTPTrader) -> StrategyResult<()> {
        // 多个 session 用于查询
        let Q = (&CTPTrader::default().env()?, &CTPTrader::default().env()?, trader);

        // 此策略止损一定是负的
        self.alpha = f64::min(-self.alpha, self.alpha);

        // true: 没有在途未成交的报单请求
        // false: 发起一笔报单请求
        let T = &Arc::new(AtomicBool::new(true));
        let tree = self.db.open_tree(&self.instrument)?;

        // tick数据
        let ticks = tree.watch_prefix("");

        trader.on_rsp_order_insert(move |api, _, error, _, _| {
            T.store(true, Ordering::Relaxed);
            log::warn!("{}", error);
            // 结算结果未确认
            if error.ErrorID == 42 {
                let api = api.lock().unwrap();
                api.req_settlement_info_confirm(&CThostFtdcSettlementInfoConfirmField::new()
                    .with_broker_id(env::var("CTP_BROKER_ID").ok()?)
                    .with_investor_id(env::var("CTP_USER_ID").ok()?), 0)?;
            }

            Ok(())
        }).on_rtn_order(move |_, order| {
            // 使用了 FAK 指令, 应该要么成交要么报错, 暂时不关注报单变化
            // 报单如果被拒绝, on_rsp_order_insert 不会有错误返回
            T.store(true, Ordering::Relaxed);
            log::info!("[{}] {} - {} - {}",
                s(&order.InstrumentID),
                order.OrderSubmitStatus,
                order.OrderStatus,
                gbk(&order.StatusMsg));
            Ok(())
        }).on_rtn_trade(move |_, trade| {
            T.store(true, Ordering::Relaxed);
            log::info!("[{}] [{}] [{} {}] [{}|{}] 成交价: {}, 成交手数: {}",
                s(&trade.TradeID),
                s(&trade.InstrumentID),
                s(&trade.TradeDate),
                s(&trade.TradeTime),
                trade.OffsetFlag,
                trade.Direction,
                trade.Price,
                trade.Volume,
            );
            Ok(())
        });

        let instrument = &self.instrument;
        let volume = self.volume;

        let tick = Q.req_qry_depth_market_data_sync(instrument)??;
        let lower = tick.LowerLimitPrice;
        let upper = tick.UpperLimitPrice;

        let alpha = self.alpha;
        let beta = self.beta;
        // 初始止损限度
        let loss = alpha.abs();

        let mut ratio = f64::NAN;

        for evt in ticks {
            match evt {
                sled::Event::Insert { value, .. } => {
                    let k: Kline = bincode::deserialize(&value).ok()?;
                    let delta = k.close - k.average;

                    if let Some(price) = self.touch_price {
                        log::info!("{{{} 止损: {}}}[{:.2} {:.2} {:.2}] [{:+.2}% {:+.2}% {:+.2}%] vol: {}, diff: {:+.8}, diff2: {:+.8}",
                            k.close,
                            price,
                            k.average*self.short,
                            delta,
                            -k.average*self.long,
                             100.*self.alpha, 100.*ratio, 100.*self.beta,
                            k.volume, k.diff, k.diff2);
                    } else {
                        log::info!("{{{}}}[{:.2} {:.2} {:.2}] [{:+.2}% {:+.2}% {:+.2}%] vol: {}, diff: {:+.8}, diff2: {:+.8}",
                            k.close,
                            k.average*self.short,
                            delta,
                            -k.average*self.long,
                            100.*self.alpha, 100.*ratio,  100.*self.beta,
                            k.volume, k.diff, k.diff2);
                    }

                    // 剔除无效数据
                    if k.average == 0. { continue; }

                    // 获取上一个 tick
                    let _tick = self.queue.iter().next();
                    // 行情处理从第二个tick开始
                    if _tick.is_none() {
                        self.queue.push(k);
                        continue;
                    }

                    let _delta = _tick?.close - _tick?.average;
                    self.queue.push(k);

                    // 当前持仓
                    // 无持仓但是可能有一个在途报单未成交
                    let position = Q.req_qry_investor_position_sync()?.into_iter().find(|p|
                        p.Position != 0 && s(&p.InstrumentID) == instrument.as_str()
                    );

                    // 止损状态
                    if let Some(price) = self.loss_price {
                        log::info!("[{}] 当前价格: {}, 止损价格: {}", self.loss_count, k.close, price);
                        // 存储止损发生那一刻的k线
                        self.queue_after_loss.push(k);

                        // 360 个 tick, 即 3m 内不作处理, 等待行情明确
                        // 如果 360 个 tick 之后, 远离止损线, 则等机会再开仓
                        // 如果 360 个 tick 之后, 回归止损线, 以止损价格接回来
                        // 这个时间选择5m, 避免短时间的震荡
                        // 如果之后的第一个 tick 价格合适, 则再次持仓, 目的是为了不被随便打掉止损单
                        if self.queue_after_loss.len() <= 360 {
                            continue;
                        }
                    }

                    // 已有持仓
                    if let Some(ref position) = position {
                        ratio = position.PositionProfit / position.UseMargin;
                        let instrument = s(&position.InstrumentID);

                        // 止损、保本
                        if ratio <= self.alpha {
                            log::warn!("[{}] 触发止损: [{}] 亏: {}", self.loss_count, instrument, -position.PositionProfit);
                            trader.close(position)?;

                            // 如果不是保本, 追踪此次止损
                            if ratio < 0. {
                                self.loss_price.replace(k.close);
                                self.loss_count += 1;
                            } else {
                                self.loss_price.take();
                            }

                            self.alpha = alpha;
                            self.beta = beta;
                            self.close_price.take();
                            self.touch_price.take();
                            self.queue_after_loss.clear();
                            continue;
                        }

                        // 触发平仓
                        if let Some(close_price) = self.close_price {
                            // 多头持仓
                            if position.PosiDirection == PositionDirection::Long {
                                // 如果下一个tick价格大于上一个tick价格, 那么等下一个tick
                                // 如果下一个tick价格小于上一个tick价格, 进行平仓
                                if k.close >= close_price {
                                    self.close_price.replace(k.close);
                                    continue;
                                }
                            }

                            // 空头持仓
                            if position.PosiDirection == PositionDirection::Short {
                                // 如果下一个tick价格小于上一个tick价格, 那么等下一个tick
                                // 如果下一个tick价格大于上一个tick价格, 进行平仓
                                if k.close <= close_price {
                                    self.close_price.replace(k.close);
                                    continue;
                                }
                            }

                            trader.close(position)?;
                            self.alpha = alpha;
                            self.beta = beta;

                            // 平仓, 重置开仓逻辑
                            self.loss_price.take();
                            self.queue_after_loss.clear();
                            self.close_price.take();
                            self.touch_price.take();
                            self.long_price.take();
                            self.short_price.take();
                        } else {
                            // 移动止损
                            let m = ratio - self.alpha;
                            if m > loss {
                                self.alpha += m - loss;
                            }

                            // 更新止损点
                            if ratio >= self.gamma*self.beta {
                                self.alpha = f64::max(self.alpha, 0.5*ratio);
                            }

                            // 盈利达 self.beta 止盈
                            if ratio >= self.beta {
                                log::warn!("触发止盈: [{} {}] 盈: {:+.2}", instrument, k.close, position.PositionProfit);
                                self.close_price.replace(k.close);
                            }

                            // 回归保本
                            // 如果均值回归比例 50% 以上，设置保本线
                            // 保证此次回归不会亏损
                            // 多单持仓
                            if let Some(price) = self.long_price {
                                if k.close >= price + f64::abs(delta)*0.7 {
                                    // 盈利部分的 50% 设置保本
                                    let delta = (k.close - price) / 2.0;
                                    // touch 值为优于开仓价 delta 的价格
                                    let touch = price + delta;
                                    if let Some(touch_) = self.touch_price {
                                        // 更新touch值
                                        self.touch_price.replace(f64::max(touch, touch_));
                                    } else {
                                        self.touch_price.replace(touch);
                                    }
                                }
                            }
                            // 空单持仓
                            if let Some(price) = self.short_price {
                                if k.close <= price - f64::abs(delta)*0.7 {
                                    // 盈利部分的 50% 设置保本
                                    let delta = (price - k.close) / 2.0;
                                    // touch 值为优于开仓价 delta 的价格
                                    let touch = price - delta;
                                    if let Some(touch_) = self.touch_price {
                                        // 更新touch值
                                        self.touch_price.replace(f64::min(touch, touch_));
                                    } else {
                                        self.touch_price.replace(touch);
                                    }
                                }
                            }

                            // 检查止损价格
                            if let Some(price) = self.touch_price {
                                // 多仓
                                if self.long_price.is_some() {
                                    // 触发止损
                                    if k.close < price {
                                        log::info!("触发止损: [{} {}] 盈亏: {:+.2}", instrument, k.close, position.PositionProfit);
                                        self.close_price.replace(k.close);
                                    }
                                }
                                // 空仓
                                if self.short_price.is_some() {
                                    // 触发止损
                                    if k.close > price {
                                        log::info!("触发止损: [{} {}] 盈亏: {:+.2}", instrument, k.close, position.PositionProfit);
                                        self.close_price.replace(k.close);
                                    }
                                }
                            }

                            // 进入平仓逻辑: 两次delta值符号变化, 即穿越了均值线视为平仓信号
                            if _delta*delta < 0. {
                                log::info!("触发平仓: [{} {}] 盈亏: {:+.2}", instrument, k.close, position.PositionProfit);
                                self.close_price.replace(k.close);
                            }
                        }
                    } else {
                        ratio = f64::NAN;
                        // 超过2次止损，放弃本次策略
                        // 此次交易损失大于 10% 停止策略
                        if self.loss_count >= self.count { continue; }

                        // 开仓判定: 跌幅超过 long 开多仓
                        if delta <= -k.average*self.long {
                            // 多单止损状态下, 为了避免震荡等待时机
                            if let Some(loss_price) = self.loss_price {
                                // 找到一个更低的、平稳的点位开多
                                // 如果当前价格高于止损价, 即表明价格触及止损之后回弹了
                                // 这个时候不用等待更低的价格了, 直接买入
                                // 否则, 确定一个买入时机
                                if k.close < loss_price {
                                    // 多单止损之后, 阶段性的最低价
                                    let min = self.queue_after_loss.iter().map(|k| k.close).collect::<Vec<_>>().min();
                                    let index = self.queue_after_loss.iter().position(|k| k.close == min).unwrap();
                                    let length = self.queue_after_loss.len();
                                    log::info!("抄底最低价: {}, p: {}, t: {}", min, index, length);

                                    // 如果不满足条件, 继续等待
                                    // 此条件为: 最低价格处于 queue_after_loss 1/3 位置
                                    // 如果不断在创新低, 则继续等待
                                    if index < length / 3 {
                                        self.long_price.take();
                                        continue;
                                    }
                                }
                            }

                            // 优化开多仓逻辑
                            if let Some(long_price) = self.long_price {
                                // 下一个tick价格更低
                                // 这里优化了开多仓逻辑, 可以获得更加低的点位
                                // 价格一旦回调就开多仓
                                if k.close < long_price {
                                    self.long_price.replace(k.close);
                                    continue;
                                }
                            } else {
                                // 保存可以开多仓点的价位
                                self.long_price.replace(k.close);
                                continue;
                            }

                            self.loss_price.take();
                            self.queue_after_loss.clear();

                            // 保证在报单未成交的情况下不再发起报单
                            // 这里只根据持仓不能判定未开仓
                            threshold!(T, {
                                log::info!("开多仓: {}, 手数: {}", k.close, volume);
                                trader.long_fak(&instrument, volume, upper)?;
                            });
                        } else if delta >= k.average*self.short {
                            // 空单止损
                            if let Some(loss_price) = self.loss_price {
                                // 找到一个更高的、平稳的点位开空
                                // 如果当前价格低于止损价, 即表明价格触及止损之后回弹了
                                // 这个之后不用等更高的价格了, 直接卖出
                                // 否则, 确定一个卖出时机
                                if k.close > loss_price {
                                    // 空单止损之后, 阶段性的最高价
                                    let max = self.queue_after_loss.iter().map(|k| k.close).collect::<Vec<_>>().max();
                                    let index = self.queue_after_loss.iter().position(|k| k.close == max).unwrap();
                                    let length = self.queue_after_loss.len();
                                    log::info!("摸顶最高价: {}, {}/{}", max, index, length);

                                    // 如果不满足条件, 继续等待
                                    // 此条件为: 最低价格处于 queue_after_loss 1/3 位置
                                    // 如果不断在创新高, 则继续等待
                                    if index < length / 3 {
                                        self.short_price.take();
                                        continue;
                                    }
                                }
                            }

                            // 优化开空仓逻辑
                            if let Some(short_price) = self.short_price {
                                // 下一个tick价格更高
                                // 这里优化了开空仓逻辑, 可以获得更加高的点位
                                // 价格一旦回调就开空仓
                                if k.close > short_price {
                                    self.short_price.replace(k.close);
                                    continue;
                                }
                            } else {
                                // 保存可以开多仓点的价位
                                self.short_price.replace(k.close);
                                continue;
                            }

                            self.loss_price.take();
                            self.queue_after_loss.clear();

                            // 保证在报单未成交的情况下不再发起报单
                            // 这里只根据持仓不能判定未开仓
                            threshold!(T, {
                                log::info!("开空仓: {}, 手数: {}", k.close, volume);
                                trader.short_fak(&instrument, volume, lower)?;
                            });
                        } else {
                            // 如果价格达到过开仓位置, 但是马上又下去了
                            // 那么清理之前的价格
                            // 这种情况应该比较少
                            self.short_price.take();
                            self.long_price.take();
                        }

                        // 如果手动平仓了，清理状态
                        self.loss_price.take();
                        self.close_price.take();
                        self.touch_price.take();
                        self.queue_after_loss.clear();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
