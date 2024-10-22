use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct PrometheusConfig {
    pub address: String,
    pub enable: Option<bool>,
}

impl PrometheusConfig {
    pub fn enabled(&self) -> bool {
        self.enable.unwrap_or(true)
    }
}

#[derive(Debug, Deserialize)]
pub struct PERMConfig {
    pub model: String,
    pub policy: Option<String>,
    pub policy_path: Option<PathBuf>,
}
