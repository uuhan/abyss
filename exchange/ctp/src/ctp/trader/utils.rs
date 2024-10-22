use super::*;
use crossbeam::{
    channel::{bounded, unbounded},
    select,
};
use parking_lot::{Condvar, Mutex};
/// 同步方法, 如果 Req <-> `OnRsp` 是一一对应的, 可以做成同步
use rand::prelude::*;
use std::{sync::Arc, time::Duration};

impl CTPTrader {
    /// 从环境变量读取参数并初始化
    /// 不需要初始化回调
    pub fn env(mut self) -> CTPResult<Self> {
        self.init_by_env(empty!(_))?;
        Ok(self)
    }

    /// 交易前置需要完成认证和登陆
    pub fn init_by_env<F>(&self, mut cb: F) -> CTPResult<&Self>
    where
        F: FnMut(Api) -> CTPApiResult + Send,
    {
        let env = &self.env;
        let cdv: Wait<CTPResult<()>> = Wait::new();
        let mut cdv2 = cdv.clone();
        let mut cdv3 = cdv.clone();

        let auth_field = CThostFtdcReqAuthenticateField::new()
            .with_broker_id(&env.broker_id)
            .with_user_id(&env.user_id)
            .with_auth_code(&env.auth_token)
            .with_app_id(&env.app_id)
            .build();

        let login_field = CThostFtdcReqUserLoginField::new()
            .with_broker_id(&env.broker_id)
            .with_user_id(&env.user_id)
            .with_password(&env.user_password)
            .build();

        self.on_front_connected(move |api| {
            tracing::info!("交易前置服务器连接");
            let api = api.lock();
            api.req_authenticate(&auth_field, 0)
        })
        .on_rsp_authenticate(move |api, auth, error, idx, _| {
            tracing::info!("鉴权状态: {}", error);
            if error.ErrorID == 0 {
                tracing::info!("    AppID: {}", s(&auth.AppID));
                let api = api.lock();
                api.req_user_login(&login_field, 0)
            } else if error.ErrorID == 7 || error.ErrorID == 8 || error.ErrorID == 154 {
                // [7]   CTP:还没有初始化
                // [8]   CTP:前置不活跃
                // [154] CTP:查询核心忙 请稍后重试
                tracing::info!("鉴权重试: {}", idx);
                // NB: 30秒之后重试
                std::thread::sleep(Duration::from_secs(30));
                let api = api.lock();
                api.req_authenticate(&auth_field, idx + 1)
            } else {
                cdv2.wake(Err(CTPError::CTP(*error)));
                Err(CTPError::CTP(*error))
            }
        })
        .on_rsp_user_login(move |api, user, error, idx, _| {
            tracing::info!("登陆状态: {}", error);
            if error.ErrorID == 0 {
                tracing::info!("    UserID: {}", s(&user.UserID));
                tracing::info!("   FrontID: {}", user.FrontID);
                tracing::info!(" SessionID: {}", user.SessionID);
                tracing::info!("  登陆时间: {}-{}", s(&user.TradingDay), s(&user.LoginTime));
                // tracing::info!("  版本信息: {}", s(&user.SysVersion));
                tracing::info!("上期所时间: {}", s(&user.SHFETime));
                tracing::info!("大商所时间: {}", s(&user.DCETime));
                tracing::info!("郑商所时间: {}", s(&user.CZCETime));
                tracing::info!("中金所时间: {}", s(&user.FFEXTime));
                tracing::info!("能源所时间: {}", s(&user.INETime));
                // tracing::info!("广期所时间: {}", s(&user.GFEXTime));

                // NB: 这里在登陆的时候即设置对应条件变量
                // 如果需要在 .init_by_env 执行完第一次之后在进行接下来的逻辑
                // 那么应该在 .init_by_env 里面设置一些机制来阻塞第一次初始化
                tracing::info!("[{}] 交易初始化", user.FrontID);
                cdv3.wake(Ok(()));
                cb(api)
            } else if error.ErrorID == 60 {
                tracing::error!("[{}]会话数超限: {}", idx, error);
                cdv3.wake(Err(CTPError::CTP(*error)));
                Err(CTPError::CTP(*error))
            } else if error.ErrorID == 7 || error.ErrorID == 8 || error.ErrorID == 154 {
                // [7]   CTP:还没有初始化
                // [8]   CTP:前置不活跃
                // [154] CTP:查询核心忙 请稍后重试
                tracing::error!("登陆重试: {}", idx);
                // NB: 30秒之后重试
                std::thread::sleep(Duration::from_secs(30));
                let api = api.lock();
                api.req_user_login(&login_field, idx + 1)
            } else {
                tracing::error!("登陆失败: {}:{}", error.ErrorID, error);
                cdv3.wake(Err(CTPError::CTP(*error)));
                Err(CTPError::CTP(*error))
            }
        })
        .on_rsp_error(move |_, error, _, _| {
            tracing::error!("CTP响应: {}", error);
            Ok(())
        });

        self.register_front(&env.front).init();

        let duration = Duration::from_secs(5);

        // 如果 5s 之内未连接, 不再等待
        if let Some(waited) = cdv.timeout(duration) {
            waited?;
            // 这里的等待绝对不会超时
            self.ready.until(true);
            Ok(self)
        } else {
            Err(CTPError::TimeOut)
        }
    }

    /// 订阅合约交易状态
    #[must_use]
    pub fn watch_instrument(&self, id: &str) -> Subscriber<CThostFtdcInstrumentStatusField> {
        self.subscribers.register(id.as_bytes())
    }

    /// 报单请求 (FAK限价报单)
    #[must_use]
    pub fn default_order_input_field(&self) -> CThostFtdcInputOrderField {
        CThostFtdcInputOrderField::default()
            .with_broker_id(&self.env.broker_id)
            .with_user_id(&self.env.user_id)
            .with_investor_id(&self.env.user_id)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_time_condition(OrderTimeCondition::Immediately)
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .build()
    }

    /// 请求查询合约手续费率响应
    pub fn req_qry_instrument_commission_rate_sync1(
        &self,
        field: &CThostFtdcQryInstrumentCommissionRateField,
    ) -> CTPResult<Option<CThostFtdcInstrumentCommissionRateField>> {
        let (tx, rx) = bounded(1);
        self.on_rsp_qry_instrument_commission_rate(move |_, commission, error, _, _| {
            signal!(tx, commission.copied())
        })
        .on_rsp_error(move |api, &error, id, _| {
            let api = api.lock();
            tracing::warn!("[{}] 手续费率查询触发限流.", id);
            api.req_qry_instrument_commission_rate(field, id + 1);
            Ok(())
        });

        self.req_qry_instrument_commission_rate(field, 0);

        on_signal!(rx)
    }

    /// 投资者结算结果确认
    pub fn req_settlement_info_confirm_sync(
        &self,
        field: &CThostFtdcSettlementInfoConfirmField,
    ) -> CTPResult<Option<CThostFtdcSettlementInfoConfirmField>> {
        let (tx, rx) = bounded(1);
        self.on_rsp_settlement_info_confirm(move |_, settlement, _, _, _| {
            signal!(tx, Some(*settlement))
        });

        self.req_settlement_info_confirm(field, 0);

        on_signal!(rx)
    }

    /// 请求查询投资者结算结果
    pub fn req_qry_settlement_info_sync(
        &self,
        field: &CThostFtdcQrySettlementInfoField,
    ) -> CTPResult<Option<CThostFtdcSettlementInfoField>> {
        let (tx, rx) = bounded(1);
        self.on_rsp_qry_settlement_info(move |_, settlement, _, _, _| {
            signal!(tx, settlement.copied())
        });

        self.req_qry_settlement_info(field, 0);

        on_signal!(rx)
    }

    /// 请求查询转帐银行响应
    pub fn req_qry_transfer_bank_sync(
        &self,
        field: &CThostFtdcQryTransferBankField,
    ) -> CTPResult<Option<CThostFtdcTransferBankField>> {
        let (tx, rx) = bounded(1);
        self.on_rsp_qry_transfer_bank(move |_, bank, _, _, _| signal!(tx, bank.copied()));

        self.req_qry_transfer_bank(field, 0);

        on_signal!(rx)
    }

    /// 请求查询客户通知响应
    pub fn req_qry_notice_sync(
        &self,
        field: &CThostFtdcQryNoticeField,
    ) -> CTPResult<Option<CThostFtdcNoticeField>> {
        let (tx, rx) = bounded(1);
        self.on_rsp_qry_notice(move |_, notice, _, _, _| signal!(tx, notice.copied()));

        self.req_qry_notice(field, 0);

        on_signal!(rx)
    }

    /// 报单录入请求, 无成交回报
    pub fn req_order_insert_sync1(
        &self,
        input_order_field: &CThostFtdcInputOrderField,
    ) -> CTPResult<Option<i32>> {
        let request_id = thread_rng().gen::<i32>();
        let (order_tx, order_rx) = unbounded();
        self.on_rtn_order(move |_, &order| {
            order_tx
                .send(order)
                .map_err(|e| CTPError::Error(Arc::new(e)))?;
            Ok(())
        });

        let (error_tx, error_rx) = unbounded();
        self.on_rsp_order_insert(move |api, _, &error, id, _| {
            tracing::warn!("{}", error);
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
                            // println!("[{}] {} - {} - {}",
                            //     s(&order.InstrumentID),
                            //     order.OrderSubmitStatus,
                            //     order.OrderStatus,
                            //     gbk(&order.StatusMsg));
                            tracing::info!("[{}] {} - {} - {}",
                                s(&order.InstrumentID),
                                order.OrderSubmitStatus,
                                order.OrderStatus,
                                gbk(&order.StatusMsg));

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
                                s@OrderStatus::NoTradeQueueing => {
                                    tracing::info!("{} - {}/{}", s, order.VolumeTraded, order.VolumeTotal);
                                    break Ok(None)
                                }
                                // 报单撤销
                                s@OrderStatus::Canceled => {
                                    tracing::info!("{} - {}/{}", s, order.VolumeTraded, order.VolumeTotal);
                                    break Ok(None)
                                }

                                s@OrderStatus::PartTradedQueueing => {
                                    tracing::info!("{} - {}/{}", s, order.VolumeTraded, order.VolumeTotal);
                                    break Ok(Some(order.VolumeTraded))
                                }

                                // 报单成功
                                // NB: 这里有可能是多次成交的, 但是不关注
                                s@OrderStatus::AllTraded => {
                                    tracing::info!("{} - {}/{}", s, order.VolumeTraded, order.VolumeTotal);
                                    break Ok(Some(order.VolumeTraded))
                                }

                                // 其他情况
                                _ => {}
                            }
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
        self.on_rsp_order_insert(empty!(_, _, _, _, _));

        res
    }

    /// 开多仓
    pub fn long_fak(&self, instrument: impl AsRef<str>, volume: i32, price: f64) -> CTPResult<()> {
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

        self.req_order_insert(&field, 0)
    }

    /// 开多仓(FAK)
    pub fn long_fak_sync1(
        &self,
        instrument: impl AsRef<str>,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<i32>> {
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

        self.req_order_insert_sync1(&field)
    }

    /// 开多仓(FAK)
    pub fn long_fak_sync2(
        &self,
        instrument: impl AsRef<str>,
        volume: i32,
        price: f64,
    ) -> CTPResult<()> {
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

        self.req_order_insert(&field, 0)
    }

    /// 开空仓
    pub fn short_fak(&self, instrument: impl AsRef<str>, volume: i32, price: f64) -> CTPResult<()> {
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

        self.req_order_insert(&field, 0)
    }

    /// 开空仓(FAK)
    pub fn short_fak_sync1(
        &self,
        instrument: impl AsRef<str>,
        volume: i32,
        price: f64,
    ) -> CTPResult<Option<i32>> {
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

        self.req_order_insert_sync1(&field)
    }

    /// 开空仓(FAK)
    pub fn short_fak_sync2(
        &self,
        instrument: impl AsRef<str>,
        volume: i32,
        price: f64,
    ) -> CTPResult<()> {
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

        self.req_order_insert(&field, 0)
    }

    /// 平仓
    pub fn close(&self, position: &CThostFtdcInvestorPositionField) -> CTPResult<()> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(());
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        // 获取一个合约 tick
        let tick = self
            .req_qry_depth_market_data_sync(instrument_id)?
            .ok_or(ne!("DepthMarketData Not Exist"))?;
        let upper = tick.UpperLimitPrice;
        let lower = tick.LowerLimitPrice;

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(position.Position)
            .with_min_volume(1)
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
                field
                    .with_direction(OrderDirection::Sell)
                    .with_limit_price(lower);
            }
            // 空头, 买平, 涨停价买
            PositionDirection::Short => {
                field
                    .with_direction(OrderDirection::Buy)
                    .with_limit_price(upper);
            }
            // ?.ok_or(ne!("DepthMarketData Not Exist"))?
            PositionDirection::Net => {
                tracing::warn!("净持仓: {} {}", &instrument_id, position.Position);
                return Ok(());
            }
        }

        // FAK 指令
        field
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_time_condition(OrderTimeCondition::Immediately);

        // FOK 指令
        // field.with_volume_condition(OrderVolumeCondition::All)
        //     .with_time_condition(OrderTimeCondition::Immediately);

        self.req_order_insert(&field, 0)?;

        Ok(())
    }

    /// 平仓(同步)
    pub fn close_sync1(
        &self,
        position: &CThostFtdcInvestorPositionField,
        price: Option<f64>,
    ) -> CTPResult<Option<i32>> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(None);
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(position.Position)
            .with_min_volume(1)
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
                        .ok_or(ne!("DepthMarketData Not Exist"))?;
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
                        .ok_or(ne!("DepthMarketData Not Exist"))?;
                    let upper = tick.UpperLimitPrice;
                    field
                        .with_direction(OrderDirection::Buy)
                        .with_limit_price(upper);
                }
            }
            // ?.ok_or(ne!("DepthMarketData Not Exist"))?
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

        self.req_order_insert_sync1(&field)
    }

    /// 平仓(同步)
    pub fn close_sync2(
        &self,
        position: &CThostFtdcInvestorPositionField,
        price: Option<f64>,
    ) -> CTPResult<Option<()>> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(None);
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(position.Position)
            .with_min_volume(1)
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
                        .ok_or(ne!("DepthMarketData Not Exist"))?;
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
                        .ok_or(ne!("DepthMarketData Not Exist"))?;
                    let upper = tick.UpperLimitPrice;
                    field
                        .with_direction(OrderDirection::Buy)
                        .with_limit_price(upper);
                }
            }
            // ?.ok_or(ne!("DepthMarketData Not Exist"))?
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

        self.req_order_insert(&field, 0).map(|r| Some(()))
    }

    /// 锁仓
    pub fn lock(&self, position: &CThostFtdcInvestorPositionField) -> CTPResult<()> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(());
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        // 获取一个合约 tick
        let tick = self
            .req_qry_depth_market_data_sync(instrument_id)?
            .ok_or(ne!("DepthMarketData Not Exist"))?;
        let upper = tick.UpperLimitPrice;
        let lower = tick.LowerLimitPrice;

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(position.Position)
            .with_min_volume(1)
            .with_order_price_type(OrderPriceType::LimitPrice)
            .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
            .with_force_close_reason(OrderForceCloseReason::NotForceClose)
            .with_contingent_condition(OrderContingentCondition::Immediately)
            .build();

        match position.PosiDirection {
            // 多头, 卖开, 跌停价卖
            PositionDirection::Long => {
                field
                    .with_comb_offset_flag(OrderOffsetFlag::Open)
                    .with_direction(OrderDirection::Sell)
                    .with_limit_price(lower);
            }
            // 空头, 买开, 涨停价买
            PositionDirection::Short => {
                field
                    .with_comb_offset_flag(OrderOffsetFlag::Open)
                    .with_direction(OrderDirection::Buy)
                    .with_limit_price(upper);
            }
            // ?.ok_or(ne!("DepthMarketData Not Exist"))?
            PositionDirection::Net => {
                tracing::warn!("净持仓: {} {}", &instrument_id, position.Position);
                return Ok(());
            }
        }

        // FAK 指令
        field
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_time_condition(OrderTimeCondition::Immediately);

        // FOK 指令
        // field.with_volume_condition(OrderVolumeCondition::All)
        //     .with_time_condition(OrderTimeCondition::Immediately);

        self.req_order_insert(&field, 0)?;

        Ok(())
    }

    /// 反手
    pub fn reverse(&self, position: &CThostFtdcInvestorPositionField) -> CTPResult<()> {
        // 未持仓的历史单, 不处理
        if position.Position == 0 {
            return Ok(());
        }

        let instrument_id = s(&position.InstrumentID);
        let investor_id = s(&position.InvestorID);
        let broker_id = s(&position.BrokerID);

        // 获取一个合约 tick
        let tick = self
            .req_qry_depth_market_data_sync(instrument_id)?
            .ok_or(ne!("DepthMarketData Not Exist"))?;
        let upper = tick.UpperLimitPrice;
        let lower = tick.LowerLimitPrice;

        let mut field = CThostFtdcInputOrderField::new()
            .with_broker_id(&broker_id)
            .with_user_id(&investor_id)
            .with_investor_id(&investor_id)
            .with_instrument_id(&instrument_id)
            .with_total_volume(position.Position)
            .with_min_volume(1)
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
                field
                    .with_direction(OrderDirection::Sell)
                    .with_limit_price(lower);
            }
            // 空头, 买开, 涨停价买
            PositionDirection::Short => {
                field
                    .with_direction(OrderDirection::Buy)
                    .with_limit_price(upper);
            }
            // ??
            PositionDirection::Net => {
                tracing::warn!("净持仓: {} {}", &instrument_id, position.Position);
                return Ok(());
            }
        }

        // FAK 指令
        field
            .with_volume_condition(OrderVolumeCondition::Any)
            .with_time_condition(OrderTimeCondition::Immediately);

        // FOK 指令
        // field.with_volume_condition(OrderVolumeCondition::All)
        //     .with_time_condition(OrderTimeCondition::Immediately);

        // 平仓
        self.req_order_insert(&field, 0)?;
        field.with_comb_offset_flag(OrderOffsetFlag::Open);
        // 开仓
        self.req_order_insert(&field, 0)?;

        Ok(())
    }

    /// 合约运行
    #[allow(clippy::missing_panics_doc)]
    pub fn instrument_exec(
        &self,
        id: impl AsRef<str>,
        mut cmd: impl FnMut(Box<dyn Fn() -> InstrumentStatus>) -> CTPResult<()>,
    ) -> CTPResult<()> {
        let paused = Arc::new((Mutex::new(InstrumentStatus::Continous), Condvar::new()));
        let paused_2 = paused.clone();
        let id = id.as_ref();

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

                {
                    let (lock, cvar) = &*paused;
                    let mut paused = lock.lock();
                    *paused = status.InstrumentStatus;
                    cvar
                }
                .notify_one();
            }
            Ok(())
        });

        cmd(Box::new(move || {
            let (lock, cvar) = &*paused_2;
            let mut paused = lock.lock();
            while !matches!(
                *paused,
                InstrumentStatus::Continous
                    | InstrumentStatus::AuctionOrdering
                    | InstrumentStatus::AuctionMatch
            ) {
                cvar.wait(&mut paused);
            }

            *paused
        }))
    }

    /// 指定状态运行
    #[allow(clippy::missing_panics_doc)]
    pub fn instrument_exec_when(
        &self,
        id: impl AsRef<str>,
        status: Vec<InstrumentStatus>,
        mut cmd: impl FnMut(Box<dyn Fn()>) -> CTPResult<()>,
    ) -> CTPResult<()> {
        let paused = Arc::new((Mutex::new(InstrumentStatus::Continous), Condvar::new()));
        let paused_2 = paused.clone();
        let id = id.as_ref();

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

                {
                    let (lock, cvar) = &*paused;
                    let mut paused = lock.lock();
                    *paused = status.InstrumentStatus;
                    cvar
                }
                .notify_one();
            }
            Ok(())
        });

        cmd(Box::new(move || {
            let (lock, cvar) = &*paused_2;
            let mut paused = lock.lock();
            while !matches!(*paused, ref s if status.contains(s)) {
                tracing::info!("等待: {:?}", status);
                cvar.wait(&mut paused);
            }
        }))
    }
}
