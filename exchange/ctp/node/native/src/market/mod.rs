use crossbeam::channel::{
    unbounded,
};
use neon::prelude::*;
use ctp::api::*;
use super::*;

mod functions;

pub struct Market {
    market: CTPMarket<'static>,
    connect_signal: SignalSlot<()>,
    disconnect_signal: SignalSlot<i32>,
    rsp_error_signal: SignalSlot<CThostFtdcRspInfoField>,
    rtn_depth_market_data_signal: SignalSlot<CThostFtdcDepthMarketDataField>,
    rtn_for_quote_rsp_signal: SignalSlot<CThostFtdcForQuoteRspField>,
}

declare_types! {
    pub class JsCTPMarket for Market {
        init(ctx) {
            let market = CTPMarket::default();

            // 信号槽
            let connect_signal = unbounded();
            let disconnect_signal = unbounded();
            let rsp_error_signal = unbounded();
            let rtn_depth_market_data_signal = unbounded();
            let rtn_for_quote_rsp_signal = unbounded();

            // 前置地址连接
            let tx = connect_signal.0.clone();
            market.on_front_connected(move |_| {
                signal!(tx, Some(()))
            });

            // 前置地址断开
            let tx = disconnect_signal.0.clone();
            market.on_front_disconnected(move |_, reason| {
                signal!(tx, Some(reason))
            });

            // 错误应答
            let tx = rsp_error_signal.0.clone();
            market.on_rsp_error(move |_, error, _, _| {
                signal!(tx, Some(*error))
            });

            // 深度行情接收
            let tx = rtn_depth_market_data_signal.0.clone();
            market.on_rtn_depth_market_data(move |_, data| {
                signal!(tx, Some(*data))
            });

            Ok(Market {
                market,
                connect_signal,
                disconnect_signal,
                rsp_error_signal,
                rtn_depth_market_data_signal,
                rtn_for_quote_rsp_signal,
            })
        }

        method get_trading_day(mut ctx) {
            let this = ctx.this();
            let day = ctx.borrow(&this, |market| {
                market.market.get_trading_day()
            });

            Ok(ctx.string(day).upcast())
        }

        method register_front(ctx) {
            functions::register_front(ctx)
        }

        method init(ctx) {
            functions::init(ctx)
        }

        method release(ctx) {
            functions::release(ctx)
        }

        method req_user_login(ctx) {
            functions::req_user_login(ctx)
        }

        method req_user_logout(ctx) {
            functions::req_user_logout(ctx)
        }

        method subscribe_market_data(ctx) {
            functions::subscribe_market_data(ctx)
        }

        method unsubscribe_market_data(ctx) {
            functions::unsubscribe_market_data(ctx)
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

        method on_rtn_depth_market_data(mut ctx) {
            on_signal!(ctx, rtn_depth_market_data_signal)
        }

        method on_rtn_for_quote_rsp(mut ctx) {
            on_signal!(ctx, rtn_for_quote_rsp_signal)
        }
    }
}
