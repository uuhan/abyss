use crate::ctp::api::CThostFtdcRspInfoField;
use crate::ext::*;
use bincode::Error as BinError;
use displaydoc::Display as DocDisplay;
use std::env::VarError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IOError;
use std::num::ParseFloatError;
use std::num::ParseIntError;
/// CTP 通用错误
use std::sync::Arc;

#[derive(Debug, DocDisplay)]
pub enum CTPError {
    /// 前端限流
    ClientLimiting,
    /// 服务限流
    ServerLimiting,
    /// 后端返回为空, 请求号: {0}
    RspFieldEmpty(i32),
    /// 错误的请求代码: {0}
    RequestError(i32),
    /// 错误: 解构 Option::None
    NoneError(&'static str),
    /// 错误: 环境变量 {0}
    VarError(VarError),
    /// 数据类型异常
    FieldTypeError,
    /// IO异常: {0}
    IO(std::io::Error),
    #[cfg(feature = "bincode")]
    /// 序列化错误
    BinError(BinError),
    /// CTP响应错误: {0}
    CTP(CThostFtdcRspInfoField),

    /// CTP 连接超时
    TimeOut,
    /// CTP 未连接
    Disconnected,
    /// CTP 未就绪
    NotReady,

    /// 其他错误: {0}
    Error(Arc<dyn StdError + Send + Sync>),
}

impl std::cmp::PartialEq for CTPError {
    fn eq(&self, other: &Self) -> bool {
        false
    }

    fn ne(&self, other: &Self) -> bool {
        true
    }
}

impl std::cmp::Eq for CTPError {}

impl StdError for CTPError {}

impl From<IOError> for CTPError {
    fn from(e: IOError) -> Self {
        Self::IO(e)
    }
}

impl From<ParseFloatError> for CTPError {
    fn from(e: ParseFloatError) -> Self {
        Self::FieldTypeError
    }
}

impl From<ParseIntError> for CTPError {
    fn from(e: ParseIntError) -> Self {
        Self::FieldTypeError
    }
}

impl From<BinError> for CTPError {
    fn from(e: BinError) -> Self {
        Self::BinError(e)
    }
}

impl From<VarError> for CTPError {
    fn from(e: VarError) -> Self {
        Self::VarError(e)
    }
}

/// 返回类型
pub type CTPResult<T> = Result<T, CTPError>;

/// req_xxx 请求的返回
pub type CTPApiResult = Result<(), CTPError>;
