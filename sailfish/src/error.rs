#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum AppError {
    /// IO异常: {0}
    IO(#[from] std::io::Error),

    /// Binance Error: {0}
    BinanceError(#[from] binance::errors::Error),

    /// Common Error: {0}
    CommonError(#[from] abyss_common::prelude::CommonError),
}

pub type AppResult<T> = std::result::Result<T, AppError>;

pub const fn ok() -> AppResult<()> {
    Ok(())
}
