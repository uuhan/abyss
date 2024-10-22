/// Lua 策略模块
use std::sync::Arc;
use rlua::prelude::*;
use ctp::*;
use ctp::api::*;
use ctp::api::opts::*;

use super::StrategyResult;
use super::Strategy;

#[derive(Clone)]
struct InputOrderField(CThostFtdcInputOrderField);
impl LuaUserData for InputOrderField {}

pub struct LuaStrategy<'a> {
    script: Option<&'a [u8]>,
    lua: Lua,
}

impl<'a> LuaStrategy<'a> {
    pub fn new() -> Self {
        let lua = Lua::new();
        LuaStrategy {
            lua,
            script: None,
        }
    }

    pub fn script<S: AsRef<[u8]>>(&mut self, script: &'a S) -> &Self {
        self.script = Some(script.as_ref());
        self
    }
}

impl<'a> Strategy for LuaStrategy<'a> {
    fn run(&mut self, trader: &CTPTrader) -> StrategyResult<()> {
        let script = self.script?;

        self.lua.context(|ctx| {
            let globals = ctx.globals();
            ctx.load(include_str!("./class.lua")).exec()?;
            ctx.load(include_str!("./inspect.lua")).exec()?;

            // 买单
            globals.set("buy", ctx.create_function(|_, (instrument, volume): (String, i32)| {
                Ok(InputOrderField(
                    CThostFtdcInputOrderField::new()
                        .with_instrument_id(&instrument)
                        .with_total_volume(volume)
                        .with_min_volume(1)
                        .with_order_price_type(OrderPriceType::LimitPrice)
                        .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
                        .with_force_close_reason(OrderForceCloseReason::NotForceClose)
                        .with_contingent_condition(OrderContingentCondition::Immediately)
                        .with_comb_offset_flag(OrderOffsetFlag::Open)
                        .build()
                ))
            })?)?;

            // 卖单
            globals.set("sell", ctx.create_function(|_, (instrument, volume): (String, i32)| {
                Ok(InputOrderField(
                    CThostFtdcInputOrderField::new()
                        .with_instrument_id(&instrument)
                        .with_total_volume(volume)
                        .with_min_volume(1)
                        .with_order_price_type(OrderPriceType::LimitPrice)
                        .with_comb_hedge_flag(OrderHedgeFlag::Speculation)
                        .with_force_close_reason(OrderForceCloseReason::NotForceClose)
                        .with_contingent_condition(OrderContingentCondition::Immediately)
                        .with_comb_offset_flag(OrderOffsetFlag::Close)
                        .build()
                ))
            })?)?;

            ctx.scope(|scope| {
                let ctp = ctx.create_table()?;

                // 开仓
                ctp.set("open", scope.create_function(|_, mut field: InputOrderField| {
                    trader.req_order_insert(&mut field.0, 0)
                        .map_err(|e| LuaError::ExternalError(Arc::new(e)))
                })?)?;

                // 平仓
                ctp.set("close", scope.create_function(|_, mut field: InputOrderField| {
                    trader.req_order_insert(&mut field.0, 0)
                        .map_err(|e| LuaError::ExternalError(Arc::new(e)))
                })?)?;

                globals.set("Ctp", ctp)?;

                ctx.load(script).exec()
            })
        })?;

        Ok(())
    }
}
