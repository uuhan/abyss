use crate::{api::opts::*, api::*, *};
use crossbeam::{
    channel::{bounded, unbounded, Receiver, Sender},
    select,
};
use parking_lot::{Condvar, Mutex, RwLock};
use rand::prelude::*;
use std::{
    any::Any,
    env,
    future::Future,
    marker::PhantomData,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    time::Duration,
};

mod market;
mod trader;
pub use market::*;
pub use trader::*;

type SignalSlot<T> = (Sender<CTPResult<Option<T>>>, Receiver<CTPResult<Option<T>>>);

pub(self) trait FtdcField: Unpin {}

pub(self) type Response = Arc<Mutex<Option<Box<dyn Any + Send>>>>;
pub(self) type FutWaker = (Waker, Response);

/// 对 CTP 请求-返回 的抽象
/// 每个请求都应该有一个相应, 所有的响应都发生在同一个线程中
pub struct CTPResponse<T> {
    /// 需要在 Spi 回调中唤起
    sender: Sender<FutWaker>,
    /// CTP 请求
    request: Option<Box<dyn FnOnce() -> CTPApiResult + Send>>,
    /// CTP 响应
    response: Response,
    /// Type Info
    phantom: PhantomData<T>,
}

impl<T: Unpin + 'static> Future for CTPResponse<T> {
    type Output = CTPResult<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        let Some(request) = this.request.take() else {
            let mut response = this.response.lock();
            let Some(response) = this.response.lock().take() else {
                tracing::error!("future unreachable!");
                return Poll::Pending;
            };

            let Ok(response) = response.downcast::<Self::Output>() else {
                tracing::error!("response wrong downcast!");
                return Poll::Pending;
            };

            // 请求已经发送
            return Poll::Ready(*response);
        };

        // 只有成功的请求才能有所期待
        // ... 就像爱情一样
        if let Err(e) = request() {
            return Poll::Ready(Err(e));
        }

        // 往队列塞入一个 Waker, 用以唤起 Future
        this.sender
            .send((cx.waker().clone(), this.response.clone()));

        // 成功的请求等待未来的响应
        Poll::Pending
    }
}

impl Trader for CTPTrader {
    type Error = CTPError;

    /// 请求查询投资者持仓
    fn req_qry_investor_position_sync(&self) -> CTPResult<Vec<CThostFtdcInvestorPositionField>> {
        metric!("req_qry_investor_position_sync");
        let _guard = self.lock.lock();

        let field = CThostFtdcQryInvestorPositionField::new();
        let (position_tx, position_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_investor_position(move |_, position, error, id, last| {
            rsp!(position_tx, position.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询投资者持仓触发限流.", id);
                let mut api = api.lock();
                api.req_qry_investor_position(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_investor_position(&field, 0)?;

        let res = on_rsp!(position_rx, error_rx);

        // 清理回调
        self.on_rsp_qry_investor_position(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询投资者持仓明细响应
    fn req_qry_investor_position_detail_sync(
        &self,
    ) -> CTPResult<Vec<CThostFtdcInvestorPositionDetailField>> {
        metric!("req_qry_investor_position_detail_sync");
        let _lock = self.lock.lock();

        let field = CThostFtdcQryInvestorPositionDetailField::new();
        let (detail_tx, detail_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_investor_position_detail(move |_, detail, _, _, last| {
            rsp!(detail_tx, detail.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询投资者持仓明细响应触发限流.", id);
                let mut api = api.lock();
                api.req_qry_investor_position_detail(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_investor_position_detail(&field, 0);

        let res = on_rsp!(detail_rx, error_rx);

        // 清理回调
        self.on_rsp_qry_investor_position_detail(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 使用产品ID查询合约
    fn req_qry_product_sync(&self, id: &str) -> CTPResult<Vec<CThostFtdcInstrumentField>> {
        metric!("req_qry_product_sync");
        let _lock = self.lock.lock();

        let &mut field = CThostFtdcQryInstrumentField::new().with_product_id(id);
        let (instrument_tx, instrument_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_instrument(move |_, instrument, error, _, last| {
            rsp!(instrument_tx, instrument.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询合约触发限流.", id);
                let mut api = api.lock();
                api.req_qry_instrument(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_instrument(&field, 0)?;
        let res = on_rsp!(instrument_rx, error_rx);

        // 清理回调
        self.on_rsp_qry_instrument(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询合约
    fn req_qry_instruments_sync(&self, id: &str) -> CTPResult<Vec<CThostFtdcInstrumentField>> {
        metric!("req_qry_instruments_sync");
        let _lock = self.lock.lock();

        let &mut field = CThostFtdcQryInstrumentField::new().with_instrument_id(id);
        let (instrument_tx, instrument_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_instrument(move |_, instrument, error, _, last| {
            rsp!(instrument_tx, instrument.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询合约触发限流.", id);
                let mut api = api.lock();
                api.req_qry_instrument(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_instrument(&field, 0)?;
        let res = on_rsp!(instrument_rx, error_rx);

        // 清理回调
        self.on_rsp_qry_instrument(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询(单一)合约
    fn req_qry_instrument_sync(&self, id: &str) -> CTPResult<Option<CThostFtdcInstrumentField>> {
        metric!("req_qry_instrument_sync");

        let _lock = self.lock.lock();
        let &mut field = CThostFtdcQryInstrumentField::new()
            .with_instrument_id(id)
            .with_exchange_inst_id(id);
        let (instrument_tx, instrument_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_instrument(move |_, instrument, error, _, last| {
            rsp!(instrument_tx, instrument.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询(单一)合约触发限流.", id);
                let mut api = api.lock();
                api.req_qry_instrument(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        // 这里如果不加上 .with_exchange_inst_id() 会返回期权、多腿合约
        self.req_qry_instrument(&field, 0)?;
        let res = on_rsp!(instrument_rx, error_rx).map(|instruments| {
            instruments
                .iter()
                .find(|instrument| s(&instrument.InstrumentID) == id)
                .copied()
        });

        // 清理回调
        self.on_rsp_qry_instrument(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询合约保证金率
    fn req_qry_instrument_margin_rate_sync(
        &self,
        id: &str,
    ) -> CTPResult<Option<CThostFtdcInstrumentMarginRateField>> {
        metric!("req_qry_instrument_margin_rate_sync");
        let _lock = self.lock.lock();

        let &mut field = CThostFtdcQryInstrumentMarginRateField::new()
            .with_instrument_id(id)
            .with_broker_id(&self.env.broker_id)
            .with_investor_id(&self.env.user_id)
            .with_hedge_flag(OrderHedgeFlag::Speculation);
        let (margin_tx, margin_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_instrument_margin_rate(move |_, margin, error, _, last| {
            rsp!(margin_tx, margin.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询合约保证金率触发限流.", id);
                let mut api = api.lock();
                api.req_qry_instrument_margin_rate(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        // 这里如果不加上 .with_exchange_inst_id() 会返回期权、多腿合约
        self.req_qry_instrument_margin_rate(&field, 0)?;
        let res = on_rsp!(margin_rx, error_rx)
            .map(|margins| margins.iter().find(|m| s(&m.InstrumentID) == id).copied());

        // 清理回调
        self.on_rsp_qry_instrument_margin_rate(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询行情响应
    fn req_qry_depth_market_data_sync(
        &self,
        id: &str,
    ) -> CTPResult<Option<CThostFtdcDepthMarketDataField>> {
        metric!("req_qry_depth_market_data_sync");

        let _lock = self.lock.lock();
        let &mut field = CThostFtdcQryDepthMarketDataField::new().with_instrument_id(id);
        let (depth_tx, depth_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_depth_market_data(move |_, depth, _, r, last| {
            rsp!(depth_tx, depth.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询行情响应触发限流.", id);
                let mut api = api.lock();
                api.req_qry_depth_market_data(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_depth_market_data(&field, 0)?;
        // 这里api有点坑爹, .with_instrument_id() 会把很多期权、多腿合约带过来
        // 用 .collect() 消耗完整个 .on_rsp_qry_depth_market_data() 响应
        // 避免接收器过早析构
        let res = on_rsp!(depth_rx, error_rx)
            .map(|depths| depths.iter().find(|t| s(&t.InstrumentID) == id).copied());

        // 清理回调
        self.on_rsp_qry_depth_market_data(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询资金账户
    fn req_qry_trading_account_sync(&self) -> CTPResult<Option<CThostFtdcTradingAccountField>> {
        metric!("req_qry_trading_account_sync");
        // CTP 每次只允许一个在途请求
        let _lock = self.lock.lock();

        let field = CThostFtdcQryTradingAccountField::new();
        let (account_tx, account_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_trading_account(move |_, account, _, _, last| {
            rsp!(account_tx, account.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询资金账户触发限流.", id);
                let mut api = api.lock();
                api.req_qry_trading_account(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_trading_account(&field, 0);
        let res = on_rsp!(account_rx, error_rx).map(|accounts| accounts.first().copied());

        // 清理回调
        self.on_rsp_qry_trading_account(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 请求查询合约手续费率响应
    fn req_qry_instrument_commission_rate_sync(
        &self,
        id: &str,
    ) -> CTPResult<Option<CThostFtdcInstrumentCommissionRateField>> {
        metric!("req_qry_instrument_commission_rate_sync");
        let _lock = self.lock.lock();

        let field = CThostFtdcQryInstrumentCommissionRateField::new()
            .with_instrument_id(id)
            .with_broker_id(&self.env.broker_id)
            .with_investor_id(&self.env.user_id)
            .build();

        let (commission_tx, commission_rx) = bounded(1);
        let (error_tx, error_rx) = bounded(1);
        self.on_rsp_qry_instrument_commission_rate(move |_, commission, error, _, last| {
            rsp!(commission_tx, commission.copied(), last)
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                let mut api = api.lock();
                tracing::warn!("[{}] 手续费率查询触发限流.", id);
                api.req_qry_instrument_commission_rate(&field, id + 1)
            } else {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_instrument_commission_rate(&field, 0);
        let res = on_rsp!(commission_rx, error_rx).map(|commissions| commissions.first().copied());

        // 清理回调
        self.on_rsp_qry_instrument_commission_rate(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        res
    }

    /// 报单录入请求(同步)
    /// TODO: 部分成交
    #[allow(clippy::too_many_lines)]
    fn req_order_insert_sync(
        &self,
        input_order_field: &CThostFtdcInputOrderField,
    ) -> CTPResult<Option<(f64, i32)>> {
        metric!("req_order_insert_sync");

        // TODO: 报单逻辑需要重新设计, 避免出现报单互相影响
        // 目前暂时用锁来解决, 即一个 session 一次只允许一个报单
        let _lock = self.lock.lock();
        let request_id = thread_rng().gen::<i32>();
        let (order_tx, order_rx) = unbounded();
        self.on_rtn_order(move |_, &order| {
            order_tx
                .send(order)
                .map_err(|e| CTPError::Error(Arc::new(e)))?;
            Ok(())
        });

        let (trade_tx, trade_rx) = unbounded();
        self.on_rtn_trade(move |_, &trade| {
            tracing::info!(
                "[{}] [{}] [{} {}] [{}|{}] 成交价: {}, 成交手数: {}",
                s(&trade.TradeID),
                s(&trade.InstrumentID),
                s(&trade.TradeDate),
                s(&trade.TradeTime),
                trade.OffsetFlag,
                trade.Direction,
                trade.Price,
                trade.Volume,
            );
            trade_tx
                .send(trade)
                .map_err(|e| CTPError::Error(Arc::new(e)))?;
            Ok(())
        });

        let (error_tx, error_rx) = unbounded();
        self.on_rsp_order_insert(move |api, _, &error, id, _| {
            tracing::debug!("{}", error);
            if id == request_id {
                error_tx
                    .send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))?;
            }
            Ok(())
        });

        self.req_order_insert(input_order_field, request_id)?;

        let res = loop {
            select! {
                recv(order_rx) -> order => {
                    match order {
                        Ok(order) => {
                            // FIXME: 会导致奔溃
                            // // std/src/io/stdio.rs#940
                            // f let Err(e) = global_s().write_fmt(args) {
                            //     panic!("failed printing to {}: {}", label, e);
                            // }
                            tracing::debug!("[{}] {} - {} - {}",
                                s(&order.InstrumentID),
                                order.OrderSubmitStatus,
                                order.OrderStatus, gbk(&order.StatusMsg));
                            let id = s(&order.InstrumentID);

                            // 报单提交情况
                            match order.OrderSubmitStatus {
                                OrderSubmitStatus::InsertRejected|
                                OrderSubmitStatus::CancelRejected|
                                OrderSubmitStatus::ModifyRejected => {
                                    break Ok(None)
                                }
                                _ => {}
                            }

                            // 报单状态
                            match order.OrderStatus {
                                // 未成交还在队列中
                                s@OrderStatus::NoTradeQueueing => {
                                    tracing::info!("[{}] {} - {}/{}", id, s, order.VolumeTraded, order.VolumeTotal);
                                }
                                // 部分成交还在队列中
                                s@OrderStatus::PartTradedQueueing => {
                                    tracing::info!("[{}] {} - {}/{}", id, s, order.VolumeTraded, order.VolumeTotal);
                                }
                                // 报单撤销
                                s@OrderStatus::Canceled => {
                                    tracing::info!("[{}] {} - {}/{}", id, s, order.VolumeTraded, order.VolumeTotal);
                                    break Ok(None)
                                }
                                // 其他情况
                                s => {
                                    tracing::info!("[{}] {} - {}/{}", id, s, order.VolumeTraded, order.VolumeTotal);
                                }
                            }
                        }
                        Err(e) => {
                            break Err(CTPError::Error(Arc::new(e)))
                        }
                    }
                }
                recv(trade_rx) -> trade => {
                    match trade {
                        Ok(trade) => {
                            // FIXME: 会导致奔溃
                            // // std/src/io/stdio.rs#940
                            // f let Err(e) = global_s().write_fmt(args) {
                            //     panic!("failed printing to {}: {}", label, e);
                            // }
                            tracing::debug!("[{}] [{} {}] [{}|{}] 成交价: {}, 成交手数: {}",
                                s(&trade.InstrumentID),
                                s(&trade.TradeDate),
                                s(&trade.TradeTime),
                                trade.OffsetFlag,
                                trade.Direction,
                                trade.Price,
                                trade.Volume,
                            );
                            break Ok(Some((trade.Price, trade.Volume)))
                        }
                        Err(e) => {
                            break Err(CTPError::Error(Arc::new(e)))
                        }
                    }
                }
                recv(error_rx) -> error => {
                    match error {
                        Ok(error) => {
                            break Err(CTPError::CTP(error))
                        }
                        Err(e) => {
                            break Err(CTPError::Error(Arc::new(e)))
                        }
                    }
                }
            }
        };

        // 清理回调
        self.on_rtn_order(empty!(_, _));
        self.on_rtn_trade(empty!(_, _));
        self.on_rsp_order_insert(empty!(_, _, _, _, _));

        res
    }

    /// 开多仓(FAK)
    fn long_fak_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<(f64, i32)>> {
        let &mut field = CThostFtdcInputOrderField::default()
            .with_broker_id(&self.env.broker_id)
            .with_user_id(&self.env.user_id)
            .with_investor_id(&self.env.user_id)
            .with_instrument_id(&instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_limit_price(price)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose);

        self.req_order_insert_sync(&field)
    }

    /// 开多仓(FOK)
    fn long_fok_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<(f64, i32)>> {
        let &mut field = CThostFtdcInputOrderField::default()
            .with_broker_id(&self.env.broker_id)
            .with_user_id(&self.env.user_id)
            .with_investor_id(&self.env.user_id)
            .with_instrument_id(&instrument)
            .with_min_volume(volume)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Buy)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_limit_price(price)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose);

        self.req_order_insert_sync(&field)
    }

    /// 开空仓(FAK)
    fn short_fak_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<(f64, i32)>> {
        let &mut field = CThostFtdcInputOrderField::default()
            .with_broker_id(&self.env.broker_id)
            .with_user_id(&self.env.user_id)
            .with_investor_id(&self.env.user_id)
            .with_instrument_id(&instrument)
            .with_min_volume(1)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_limit_price(price)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose);

        self.req_order_insert_sync(&field)
    }

    /// 开空仓(FOK)
    fn short_fok_sync(
        &self,
        instrument: &str,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<(f64, i32)>> {
        let &mut field = CThostFtdcInputOrderField::default()
            .with_broker_id(&self.env.broker_id)
            .with_user_id(&self.env.user_id)
            .with_investor_id(&self.env.user_id)
            .with_instrument_id(&instrument)
            .with_min_volume(volume)
            .with_total_volume(volume)
            .with_comb_offset_flag(OrderOffsetFlag::Open)
            .with_direction(OrderDirection::Sell)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_limit_price(price)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::All)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose);

        self.req_order_insert_sync(&field)
    }

    /// 平仓(同步)
    fn close_sync(
        &self,
        position: &CThostFtdcInvestorPositionField,
        price: Option<f64>,
        volume: Option<i32>,
    ) -> CTPResult<Option<(f64, i32)>> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(None);
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        let total_volume = volume.unwrap_or(position.Position);
        let min_volume = 1; // volume.unwrap_or(position.Position);

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(total_volume)
            .with_min_volume(min_volume)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_comb_offset_flag(OrderOffsetFlag::Close)
            .build();

        // 今日仓平今仓
        if position.PositionDate == PositionDate::Today {
            field.with_comb_offset_flag(OrderOffsetFlag::CloseToday);
        }

        match position.PosiDirection {
            // 多头, 卖平, 跌停价卖
            PositionDirection::Long => {
                if let Some(price) = price {
                    field
                        .with_direction(OrderDirection::Sell)
                        .with_limit_price(price);
                } else {
                    // 获取一个合约 tick
                    let tick = self
                        .req_qry_depth_market_data_sync(instrument_id)?
                        .ok_or(CTPError::NoneError("DepthMarketData Not Found."))?;
                    let lower = tick.LowerLimitPrice;
                    field
                        .with_direction(OrderDirection::Sell)
                        .with_limit_price(lower);
                }
            }
            // 空头, 买平, 涨停价买
            PositionDirection::Short => {
                if let Some(price) = price {
                    field
                        .with_direction(OrderDirection::Buy)
                        .with_limit_price(price);
                } else {
                    // 获取一个合约 tick
                    let tick = self
                        .req_qry_depth_market_data_sync(instrument_id)?
                        .ok_or(ne!("DepthMarketData Not Found."))?;
                    let upper = tick.UpperLimitPrice;
                    field
                        .with_direction(OrderDirection::Buy)
                        .with_limit_price(upper);
                }
            }
            // ??
            PositionDirection::Net => {
                tracing::warn!("净持仓: {} {}", &instrument_id, position.Position);
                return Ok(None);
            }
        }

        // FAK 指令
        field
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_time_condition(OrderTimeCondition::Immediately);

        // FOK 指令
        // field.with_volume_condition(OrderVolumeCondition::All)
        //     .with_time_condition(OrderTimeCondition::Immediately);

        self.req_order_insert_sync(&field)
    }

    fn instrument_wait_status(
        &self,
        product_id: &str,
    ) -> Arc<dyn Fn() -> Option<InstrumentStatus> + Send + Sync> {
        let subscribers = self.subscribers.clone();
        let id = product_id.to_string();
        // 可选状态列表, 如果提供此参数, 等待列表中的状态出现
        Arc::new(move || {
            subscribers
                .register(id.as_bytes())
                .next()
                .map(|s| s.InstrumentStatus)
        })
    }

    fn instrument_get_status(
        &self,
        product_id: &str,
        status: InstrumentStatus,
    ) -> Arc<dyn Fn() -> InstrumentStatus + Send + Sync> {
        let paused = Arc::new(RwLock::new(status));
        let paused_2 = paused.clone();
        let id = product_id;

        self.add_on_rtn_instrument_status(move |_, status| {
            if s(&status.InstrumentID) == id {
                tracing::info!(
                    "[{}] [{}:{}] {} {}",
                    s(&status.EnterTime),
                    s(&status.ExchangeID),
                    s(&status.InstrumentID),
                    status.InstrumentStatus,
                    status.EnterReason
                );

                let mut w_status = paused.write();
                *w_status = status.InstrumentStatus;
            }
            Ok(())
        });

        let id: String = product_id.to_string();

        Arc::new(move || *paused_2.read())
    }

    fn instrument_watch_status(&self, id: &str) -> Subscriber<CThostFtdcInstrumentStatusField> {
        self.watch_instrument(id)
    }

    fn get_trading_day(&self) -> CTPResult<String> {
        let mut trader = self.api.lock();
        let day = trader.get_trading_day().to_owned();
        if day.is_empty() {
            tracing::error!("[EMPTY] trading day.");
        }
        Ok(day)
    }

    fn ready(&self) -> CTPResult<()> {
        self.ready.until(true);
        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl AsyncTrader for CTPTrader {
    type Error = CTPError;

    async fn req_qry_trading_account_async(
        &self,
    ) -> Result<Option<CThostFtdcTradingAccountField>, Self::Error> {
        use async_channel::{bounded, unbounded};
        use futures::FutureExt;

        let field = CThostFtdcQryTradingAccountField::new();
        let (account_tx, account_rx) = unbounded();
        let (error_tx, error_rx) = bounded(1);

        self.on_rsp_qry_trading_account(move |_, account, _, _, last| {
            tracing::info!("[AsyncTrader] {:?}", account);

            // FIXME: core dump here
            let r = futures::executor::block_on(account_tx.send((account.copied(), last)))
                .map_err(|e| CTPError::Error(Arc::new(e)));

            tracing::info!("sended {:?}", r);

            r
        })
        .on_rsp_error(move |api, &error, id, _| {
            // 触发服务端限流，重新发起请求。
            if error.ErrorID == 90 {
                tracing::warn!("[{}] 请求查询资金账户触发限流.", id);
                let mut api = api.lock();
                api.req_qry_trading_account(&field, id + 1)
            } else {
                error_tx
                    .try_send(error)
                    .map_err(|e| CTPError::Error(Arc::new(e)))
            }
        });

        self.req_qry_trading_account(&field, 0);

        let mut response = vec![];
        loop {
            futures::select! {
                value = account_rx.recv().fuse() => {
                    match value {
                        Ok((Some(position), last)) => {
                            response.push(position);
                            if last { break; }
                        }
                        Ok((None, _)) => {
                            break;
                        }
                        Err(e) => {
                            return Err(CTPError::Error(Arc::new(e)));
                        }
                    }
                }

                error = error_rx.recv().fuse() => {
                    match error {
                        Ok(error) => {
                            return Err(CTPError::CTP(error));
                        }
                        Err(e) => {
                            return Err(CTPError::Error(Arc::new(e)));
                        }
                    }
                }
            };
        }

        // 清理回调
        self.on_rsp_qry_trading_account(empty!(_, _, _, _, _));
        self.on_rsp_error(empty!(_, _, _, _));

        Ok(response.pop())
    }
}

macro_rules! repeat_type {
    ($t:ty, ($literal:literal)) => {
        ($t,)
    };
    ($t:ty, ($($literals:literal),+)) => {
        repeat_type!(@step $t, (), ($($literals),*))
    };
    (@step $t:ty, (), ($first:literal, $($rest:literal),*)) => {
        repeat_type!(@step $t, ($t), ($($rest),*))
    };
    (@step $t:ty, ($($partial:tt),*), ($first:literal, $($rest:literal),*)) => {
        repeat_type!(@step $t, ($t, $($partial),*), ($($rest),*))
    };
    (@step $t:ty, ($($partial:tt),*), ($last:literal)) => {
        ($($partial),*, $t)
    };
}

macro_rules! repeat_expr {
    ($t:expr, ($literal:literal)) => {
        ($t,)
    };
    ($t:expr, ($($literals:literal),+)) => {
        repeat_expr!(@step $t, (), ($($literals),*))
    };
    (@step $t:expr, (), ($first:literal, $($rest:literal),*)) => {
        repeat_expr!(@step $t, ($t), ($($rest),*))
    };
    (@step $t:expr, ($($partial:tt),*), ($first:literal, $($rest:literal),*)) => {
        repeat_expr!(@step $t, ($t, $($partial),*), ($($rest),*))
    };
    (@step $t:expr, ($($partial:tt),*), ($last:literal)) => {
        ($($partial),*, $t)
    };
}

macro_rules! count {
    () => (0_u8);
    ($x:tt $($xs:tt)*) => (1_u8 + count!($($xs)*));
}

macro_rules! impl_trader_tuple {
    ($T:ty; $($indices:tt),+) => {
        impl Trader for repeat_type!($T, ($($indices),+)) {
            type Error = CTPError;

            #[inline(always)]
            #[allow(clippy::modulo_one)]
            fn turn(&self) -> u8 {
                use std::sync::atomic::{AtomicU8, Ordering};
                static TURN_TABLE: AtomicU8 = AtomicU8::new(0);
                TURN_TABLE.fetch_add(1, Relaxed) % count!($($indices)+)
            }

            fn req_qry_investor_position_sync(&self) -> CTPResult<Vec<CThostFtdcInvestorPositionField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_investor_position_sync()
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_investor_position_detail_sync(&self)
                -> CTPResult<Vec<CThostFtdcInvestorPositionDetailField>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_investor_position_detail_sync()
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_product_sync(&self, id: &str) -> CTPResult<Vec<CThostFtdcInstrumentField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_product_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_instruments_sync(&self, id: &str) -> CTPResult<Vec<CThostFtdcInstrumentField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_instruments_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_instrument_sync(&self, id: &str) -> CTPResult<Option<CThostFtdcInstrumentField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_instrument_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_instrument_margin_rate_sync(&self, id: &str) -> CTPResult<Option<CThostFtdcInstrumentMarginRateField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_instrument_margin_rate_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_depth_market_data_sync(&self, id: &str) -> CTPResult<Option<CThostFtdcDepthMarketDataField>> {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_depth_market_data_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_trading_account_sync(&self)
                -> CTPResult<Option<CThostFtdcTradingAccountField>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_trading_account_sync()
                    })+
                    _ => unreachable!()
                }
            }

            fn req_qry_instrument_commission_rate_sync(
                &self, id: &str
            )
                -> CTPResult<Option<CThostFtdcInstrumentCommissionRateField>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_qry_instrument_commission_rate_sync(id)
                    })+
                    _ => unreachable!()
                }
            }

            fn req_order_insert_sync(&self, input_order_field: &CThostFtdcInputOrderField)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.req_order_insert_sync(input_order_field)
                    })+
                    _ => unreachable!()
                }
            }

            fn long_fak_sync(&self, instrument: &str, volume: i32, price: f64)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.long_fak_sync(instrument, volume, price)
                    })+
                    _ => unreachable!()
                }
            }

            fn long_fok_sync(&self, instrument: &str, volume: i32, price: f64)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.long_fok_sync(instrument, volume, price)
                    })+
                    _ => unreachable!()
                }
            }

            fn short_fak_sync(&self, instrument: &str, volume: i32, price: f64)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.short_fak_sync(instrument, volume, price)
                    })+
                    _ => unreachable!()
                }
            }

            fn short_fok_sync(&self, instrument: &str, volume: i32, price: f64)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.short_fok_sync(instrument, volume, price)
                    })+
                    _ => unreachable!()
                }
            }

            fn close_sync(&self, position: &CThostFtdcInvestorPositionField, price: Option<f64>, volume: Option<i32>)
                -> CTPResult<Option<(f64, i32)>>
            {
                match self.turn() {
                    $($indices => {
                        tracing::trace!("[#{}] SESSION TURN.", $indices);
                        self.$indices.close_sync(position, price, volume)
                    })+
                    _ => unreachable!()
                }
            }

            fn instrument_wait_status(&self, product_id: &str)
                -> Arc<dyn Fn() -> Option<InstrumentStatus> + Send + Sync>
            {
                self.0.instrument_wait_status(product_id)
            }

            fn instrument_get_status(&self, product_id: &str, status: InstrumentStatus)
                -> Arc<dyn Fn() -> InstrumentStatus + Send + Sync>
            {
                self.0.instrument_get_status(product_id, status)
            }

            fn instrument_watch_status(&self, product_id: &str)
                -> Subscriber<CThostFtdcInstrumentStatusField>
            {
                self.0.watch_instrument(product_id)
            }

            fn get_trading_day(&self) -> CTPResult<String> {
                // 获取交易日不会触发限流
                self.0.get_trading_day()
            }

            fn ready(&self) -> CTPResult<()> {
                $(self.$indices.ready()?;)+
                Ok(())
            }
        }
    }
}

/// # Panics
///
/// Will panic if `count` > 16
pub fn prepare_ctp_trader<'a, Factory>(
    count: usize,
    factory: Factory,
) -> CTPResult<Arc<dyn Trader<Error = CTPError> + Send + Sync + 'a>>
where
    Factory: Fn() -> CTPResult<CTPTrader>,
{
    match count {
        1 => Ok(Arc::new(repeat_expr!(factory()?, (0)))),
        2 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1)))),
        3 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2)))),
        4 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2, 3)))),
        5 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2, 3, 4)))),
        6 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2, 3, 4, 5)))),
        7 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2, 3, 4, 5, 6)))),
        8 => Ok(Arc::new(repeat_expr!(factory()?, (0, 1, 2, 3, 4, 5, 6, 7)))),
        9 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8)
        ))),
        10 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
        ))),
        11 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
        ))),
        12 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11)
        ))),
        13 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)
        ))),
        14 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13)
        ))),
        15 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14)
        ))),
        16 => Ok(Arc::new(repeat_expr!(
            factory()?,
            (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)
        ))),
        _ => unimplemented!(),
    }
}

impl_trader_tuple!(CTPTrader; 0);
impl_trader_tuple!(CTPTrader; 0, 1);
impl_trader_tuple!(CTPTrader; 0, 1, 2);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_trader_tuple!(CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);

impl_trader_tuple!(&CTPTrader; 0);
impl_trader_tuple!(&CTPTrader; 0, 1);
impl_trader_tuple!(&CTPTrader; 0, 1, 2);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
impl_trader_tuple!(&CTPTrader; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
