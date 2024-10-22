use casbin::Error as CasbinError;
use displaydoc::Display as DocDisplay;
use prometheus::Error as PrometheusError;

#[derive(thiserror::Error, Debug, DocDisplay)]
pub enum Error {
    /// IO异常: {0}
    IO(#[from] std::io::Error),
    /// Prometheus错误
    PromError(#[from] PrometheusError),
    /// 权限配置错误: {0}
    CasbinError(#[from] CasbinError),
    /// 错误信息: {0}
    Error(&'static str),
}

pub type Result<T> = std::result::Result<T, Error>;
