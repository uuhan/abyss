#![allow(unused)]
use ctp::api::CThostFtdcDepthMarketDataField;
/// k线工具
use sled::{Db, Result, Tree};
use yata::core::Candle;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Kline {
    pub open_interest: f64,
    pub total_volume: i32,
    pub volume: i32,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub average: f64,
    pub diff: f64,
    pub diff2: f64,
}

impl Kline {
    #[inline(always)]
    pub fn new(data: &CThostFtdcDepthMarketDataField) -> Kline {
        let CThostFtdcDepthMarketDataField {
            LastPrice: price,
            OpenInterest: open_interest,
            Volume: volume,
            AveragePrice: average,
            ..
        } = *data;

        Kline {
            // 开盘价
            open: price,
            // 最高点
            high: price,
            // 最低点
            low: price,
            // 收盘价(k线价格)
            close: price,
            // 起始均价
            average,
            // 持仓量
            open_interest,
            // k线成交量
            volume: 0,
            // 成交量
            total_volume: volume,
            // 均价差值, 即: <起始均价> + self.diff = <结束均价>
            diff: 0.,
            // 备用字段, 再无用处
            diff2: 0.,
        }
    }

    #[inline(always)]
    pub fn update_by_tick(&mut self, data: &CThostFtdcDepthMarketDataField) -> &Self {
        let CThostFtdcDepthMarketDataField {
            LastPrice: price,
            OpenInterest: open_interest,
            Volume: volume,
            AveragePrice: average,
            ..
        } = *data;

        // NB: 郑商所集合竞价报单阶段会发一些昨天的tick, 此时需要拿真正的数据更新
        // 特征为tick的均值为 0
        if average == 0. {
            return self;
        }

        // NB: 郑商所日盘品种
        if self.average == 0. {
            // 更新为当日数据
            // 一般是在郑商所开盘前
            self.open = price;
            self.high = price;
            self.low = price;
            self.close = price;
            self.average = average;
            self.total_volume = volume;
        }

        let Kline { low, high, .. } = *self;

        // NB: 郑交所日盘夜盘为同一个时间
        // if volume < self.total_volume {
        //     log::warn!("[{}] [{}] 异常数据! {} < {}", data.timestamp(), ctp::s(&data.InstrumentID), volume, self.total_volume);
        // }

        if price > high {
            self.high = price;
        }
        if price < low {
            self.low = price;
        }
        self.close = price;
        self.volume = self.volume + (volume - self.total_volume).abs();
        self.total_volume = volume;
        self.open_interest = open_interest;
        self.diff = average - self.average;

        self
    }

    /// 多
    pub fn is_gain(&self) -> bool {
        self.close > self.open
    }

    /// 平
    pub fn is_equal(&self) -> bool {
        self.close == self.open
    }

    /// 空
    pub fn is_loss(&self) -> bool {
        self.close < self.open
    }

    /// 运行于均价之上
    pub fn above_average(&self) -> bool {
        self.close > self.average
    }

    /// 运行于均价之下
    pub fn below_average(&self) -> bool {
        self.close < self.average
    }
}

// 为 Kline 实现加法
// 同一个时间段内的kline加法才有意义, 所以持仓量, 收盘价都取后者
impl std::ops::Add for Kline {
    type Output = Kline;
    fn add(self, rhs: Kline) -> Self::Output {
        Kline {
            low: f64::min(self.low, rhs.low),
            high: f64::max(self.high, rhs.high),
            open: self.open,
            close: rhs.close,
            // 这个是总持仓
            open_interest: rhs.open_interest,
            // 这个是总成交量
            total_volume: rhs.total_volume,
            volume: self.volume + rhs.volume,
            average: rhs.average,
            diff: self.diff + rhs.diff,
            diff2: self.diff2 + rhs.diff2,
        }
    }
}

// 为 kline 实现 += 运算
impl std::ops::AddAssign for Kline {
    fn add_assign(&mut self, rhs: Kline) {
        self.low = f64::min(self.low, rhs.low);
        self.high = f64::max(self.high, rhs.high);
        self.close = rhs.close;
        self.open_interest = rhs.open_interest;
        self.total_volume = rhs.total_volume;
        self.average = rhs.average;
        self.volume += rhs.volume;
        self.diff += rhs.diff;
        self.diff += rhs.diff2;
    }
}

impl Into<Candle> for Kline {
    fn into(self) -> Candle {
        (
            self.open,
            self.high,
            self.low,
            self.close,
            self.volume as f64,
        )
            .into()
    }
}

// sled 的 merge 操作子, 将行情数据合并入k线数据
pub fn merge_tick_to_kline(multi: i32) -> impl Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {
    move |key: &[u8], old: Option<&[u8]>, data: &[u8]| -> Option<Vec<u8>> {
        // 深度行情
        let mut data = bincode::deserialize::<CThostFtdcDepthMarketDataField>(data).ok()?;
        data.AveragePrice /= (multi as f64);

        // 获取存储kline
        let mut kline = old.map_or_else(
            || Some(Kline::new(&data)),
            |ov| bincode::deserialize::<Kline>(ov).ok(),
        )?;

        // 更新k线数据
        kline.update_by_tick(&data);

        // 序列化kline
        bincode::serialize(&kline).ok()
    }
}
