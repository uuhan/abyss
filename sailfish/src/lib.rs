pub mod endpoints;
pub mod service;

mod config;
mod error;
mod metrics;
mod runner;

pub use error::{AppError, AppResult};

pub mod prelude {
    pub use crate::config::CONFIG;
    pub use crate::error::ok;
    pub use crate::runner::Runner;
    pub use crate::*;
}

pub fn init_logger() {
    use tracing_subscriber::filter::EnvFilter;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
