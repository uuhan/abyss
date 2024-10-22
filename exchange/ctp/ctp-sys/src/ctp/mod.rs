#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::ext::*;
use chrono::Local;
use std::ffi::CString;

mod common;
mod market;
mod trader;

pub use common::*;
pub use market::{MarketApi, MarketSpi};
pub use trader::{TraderApi, TraderSpi};

/// raw api from ctp.h
pub mod api {
    use chrono::prelude::*;
    use smallvec::{smallvec, SmallVec};

    #[cfg(target_os = "macos")]
    macro_rules! link_name {
        ($name:literal) => {
            concat!("\u{1}__", $name)
        };
    }

    #[cfg(target_os = "ios")]
    macro_rules! link_name {
        ($name:literal) => {
            concat!("\u{1}__", $name)
        };
    }

    #[cfg(target_os = "linux")]
    macro_rules! link_name {
        ($name:literal) => {
            concat!("\u{1}_", $name)
        };
    }

    // ctp 接口
    include!("./ctp.rs");

    impl CThostFtdcDepthMarketDataField {
        /// 用于唯一标识一个 tick 数据
        /// 2021-04-01 之后 tick 的 key 使用当前日期, 而不是 TradingDay, 以避免各个交易所
        /// TradingDay 逻辑不同造成混乱
        /// CTP 每秒有两个 tick, 尾部的 0, 1 用于区分 0ms 和 500ms
        #[inline(always)]
        pub fn key(&self) -> SmallVec<[u8; 20]> {
            self.key1(&self.TradingDay[0..8])
        }

        /// 行情的唯一键值，格式为: 当前交易日(8*u8)+b' '(1*u8)+更新时间(8*u8)+更新毫秒(2*u8)+唯一标识
        /// NB: 最后一位唯一标识主要是兼容郑商所行情
        #[inline(always)]
        pub fn key1(&self, day: &[u8]) -> SmallVec<[u8; 20]> {
            // [50, 48, 50, 49, 48, 51, 49, 53, ' ', 50, 51, 58, 48, 50, 58, 52, 52, 0, 0, u8::MAX]
            // 2021031217:17:170(i32)u8::MAX
            let mut key = smallvec![];

            // 当前交易日
            key.extend_from_slice(day);
            key.push(b' ');

            // 行情更新时间
            // NB: UpdateTime [u8; 9], '00:00:00\0'
            key.extend_from_slice(&self.UpdateTime[0..8]);

            // NB: 好像不是所有交易所按照 0/500 切片的, 晕死! =.=
            // 这里以 500ms 作为分界, 500(不包含) 前的用 '0' 标识, 500(包含)后的用 '1' 标识
            // 这样比之前 UpdateMillisec == 0 好点
            // key.push(if self.UpdateMillisec < 500 {
            //     b'0'
            // } else {
            //     b'1'
            // });
            //
            // UPDATE: 20220224 这个交易日之后，行情的 key 扩展到 22 位长度, 最后 4 位是毫秒时间
            // 上期所: 只有 0, 500 分片
            // 大商所: 返回真实的毫秒值
            // 郑商所: 只有 0
            // 毫秒值范围 0~999, 这里使用 i16 存储毫秒值
            let m = self.UpdateMillisec as i16;
            key.extend_from_slice(&m.to_be_bytes());

            // NB: 这位用来兼容郑商所行情
            // 郑商所 UpdateMillisec 字段永远是 0i32, 所有没法区分行情的先后
            // 使用策略: 如果遇到同名的 key, 第 18 个 u8 减1
            key.push(u8::MAX);

            key
        }

        /// 该 tick 的时间戳
        #[inline]
        pub fn timestamp(&self) -> DateTime<Local> {
            let mut key: SmallVec<[u8; 18]> = smallvec![];
            key.extend_from_slice(&self.TradingDay);
            key[8] = b' ';
            key.extend_from_slice(&self.UpdateTime);
            key[17] = b' ';

            // "20210312 17:17:17 %3f"
            // NB: 偶尔会有 TradingDay 为 [\0;N] 的情况
            let ts = format!("{}{:0>3}", s(&key), self.UpdateMillisec);
            #[allow(deprecated)]
            match Local.datetime_from_str(&ts, "%Y%m%d %H:%M:%S %3f") {
                Ok(ts) => ts,
                Err(e) => {
                    tracing::error!(error=?e, ts, "tick.timestamp() parse failed.");
                    Local::now()
                }
            }
        }

        /// 该 tick 的时间戳, 可用交易日替换
        #[inline]
        pub fn timestamp1(&self, day: &[u8]) -> DateTime<Local> {
            let mut key: SmallVec<[u8; 18]> = smallvec![];

            // NB: 如果是推入的日期, 长度为8
            key.extend_from_slice(day);
            key.push(b' ');

            key.extend_from_slice(&self.UpdateTime);
            key[17] = b' ';

            // "20210312 17:17:17 %3f"
            // NB: 偶尔会有 TradingDay 为 [\0;N] 的情况
            let ts = format!("{}{:0>3}", s(&key), self.UpdateMillisec);
            #[allow(deprecated)]
            match Local.datetime_from_str(&ts, "%Y%m%d %H:%M:%S %3f") {
                Ok(ts) => ts,
                Err(e) => {
                    tracing::error!(error=?e, ts, "tick.timestamp() parse failed.");
                    Local::now()
                }
            }
        }

        /// 这个tick切片最后一笔是买单
        #[inline]
        pub fn is_buy(&self) -> bool {
            self.LastPrice == self.AskPrice1
        }

        /// 这个tick切片最后一笔是卖单
        #[inline]
        pub fn is_sell(&self) -> bool {
            self.LastPrice == self.BidPrice1
        }
    }

    impl CThostFtdcInvestorPositionField {
        /// 开仓均价
        #[inline]
        pub fn open_price(&self, info: &CThostFtdcInstrumentField) -> f64 {
            self.OpenCost / (self.Position * info.VolumeMultiple) as f64
        }

        /// 本持仓盈亏值
        #[inline]
        pub fn profit(&self, info: &CThostFtdcInstrumentField) -> f64 {
            // 当前价格
            let current_price = self.SettlementPrice;
            // 开仓价格
            let open_price = self.OpenCost / (self.Position * info.VolumeMultiple) as f64;

            // 持仓盈亏
            match self.PosiDirection {
                PositionDirection::Long => {
                    (current_price - open_price) * (info.VolumeMultiple * self.Position) as f64
                }
                PositionDirection::Short => {
                    (open_price - current_price) * (info.VolumeMultiple * self.Position) as f64
                }
                _ => self.PositionProfit,
            }
        }

        /// 本持仓盈亏率
        #[inline]
        pub fn profit_ratio(&self, info: &CThostFtdcInstrumentField) -> f64 {
            self.profit(info) / self.UseMargin
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ctp::api::*;
    use crate::ctp::*;
}
