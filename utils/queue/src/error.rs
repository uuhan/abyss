use lapin::Error as LapinError;
use redis::RedisError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Redis Error: {0}")]
    RedisError(#[from] RedisError),
    #[error("RabbitMQ Error: {0}")]
    LapinError(#[from] LapinError),
    #[error("Custom Error: {0}")]
    CustomError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
