mod metrics;

pub mod config;
pub mod endpoints;
mod error;
pub mod task;

pub mod prelude {
    use super::*;
    pub use error::{Error as CommonError, Result as CommonResult};
    pub use task::{SpawnEssentialTaskHandle, SpawnTaskHandle};

    pub use endpoints::http::{self, HttpStateT};
}
