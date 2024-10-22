use crossbeam::channel::{
    unbounded,
};
use neon::prelude::*;
use ctp::api::*;
use super::*;

mod functions;

#[derive(Default)]
struct Auth {
    broker_id: Option<String>,
    user_id: Option<String>,
    password: Option<String>,
    app_id: Option<String>,
    auth_token: Option<String>,
}

pub struct Trader {
    trader: CTPTrader<'static>,
    auth: Auth,
    connect_signal: SignalSlot<()>,
    disconnect_signal: SignalSlot<i32>,
    rsp_error_signal: SignalSlot<CThostFtdcRspInfoField>,
    rtn_instrument_status_signal: SignalSlot<CThostFtdcInstrumentStatusField>,
    rtn_order_signal: SignalSlot<CThostFtdcOrderField>,
    rtn_trade_signal: SignalSlot<CThostFtdcTradeField>,
}

declare_types! {
    pub class JsCTPTrader for Trader {
        init(mut ctx) {
            let path = ctx.argument::<JsString>(0)?;
            let trader = CTPTrader::new(path.value());

            // 信号槽
            let connect_signal = unbounded();
            let disconnect_signal = unbounded();
            let rsp_error_signal = unbounded();
            let rtn_instrument_status_signal = unbounded();
            let rtn_order_signal = unbounded();
            let rtn_trade_signal = unbounded();

            // 交易前置连接
            let tx = connect_signal.0.clone();
            trader.on_front_connected(move |_| {
                signal!(tx, Some(()))
            });

            // 交易前置断连
            let tx = disconnect_signal.0.clone();
            trader.on_front_disconnected(move |_, reason| {
                signal!(tx, Some(reason))
            });

            // 错误应答
            let tx = rsp_error_signal.0.clone();
            trader.on_rsp_error(move |_, error, _, _| {
                signal!(tx, Some(*error))
            });

            // 合约状态变化
            let tx = rtn_instrument_status_signal.0.clone();
            trader.on_rtn_instrument_status(move |_, status| {
                signal!(tx, Some(*status))
            });

            // 报单通知
            let tx = rtn_order_signal.0.clone();
            trader.on_rtn_order(move |_, order| {
                signal!(tx, Some(*order))
            });

            // 成交通知
            let tx = rtn_trade_signal.0.clone();
            trader.on_rtn_trade(move |_, trade| {
                signal!(tx, Some(*trade))
            });

            Ok(Trader {
                trader,
                auth: Auth::default(),
                connect_signal,
                disconnect_signal,
                rsp_error_signal,
                rtn_instrument_status_signal,
                rtn_order_signal,
                rtn_trade_signal,
            })
        }

        method register_front(ctx) {
            functions::register_front(ctx)
        }

        method init(ctx) {
            functions::init(ctx)
        }

        method subscribe_private_topic(ctx) {
            functions::subscribe_private_topic(ctx)
        }

        method subscribe_public_topic(ctx) {
            functions::subscribe_public_topic(ctx)
        }

        method req_user_login(ctx) {
            functions::req_user_login(ctx)
        }

        method req_qry_investor_position(ctx) {
            functions::req_qry_investor_position(ctx)
        }

        method req_qry_instrument(ctx) {
            functions::req_qry_instrument(ctx)
        }

        method req_settlement_info_confirm(ctx) {
            functions::req_settlement_info_confirm(ctx)
        }

        method req_qry_trading_account(ctx) {
            functions::req_qry_trading_account(ctx)
        }

        method req_order_insert(ctx) {
            functions::req_order_insert(ctx)
        }

        method req_qry_depth_market_data(ctx) {
            functions::req_qry_depth_market_data(ctx)
        }

        // 信号
        method on_front_connected(mut ctx) {
            on_signal!(ctx, connect_signal)
        }

        method on_front_disconnected(mut ctx) {
            on_signal!(ctx, disconnect_signal)
        }

        method on_rsp_error(mut ctx) {
            on_signal!(ctx, rsp_error_signal)
        }

        method on_rtn_instrument_status(mut ctx) {
            on_signal!(ctx, rtn_instrument_status_signal)
        }

        method on_rtn_order(mut ctx) {
            on_signal!(ctx, rtn_order_signal)
        }

        method on_rtn_trade(mut ctx) {
            on_signal!(ctx, rtn_trade_signal)
        }
    }
}

