use abyss_common::config::PrometheusConfig;
use serde::Deserialize;
use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::LazyLock as Lazy;

static CONFIG_NAME: &'static str = "sail.toml";

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let path = if let Ok(mut path) = std::env::current_dir() {
        path.push(CONFIG_NAME);
        if path.exists() {
            path
        } else {
            dirs::home_dir().unwrap().join(CONFIG_NAME)
        }
    } else {
        dirs::home_dir().unwrap().join(CONFIG_NAME)
    };

    assert!(path.exists(), "{} file not found!", CONFIG_NAME);

    let mut file = std::fs::File::open(&path).expect("open config file failed!");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read config file failed!");

    match toml::from_str(&buf) {
        Ok(config) => config,
        Err(e) => {
            panic!("config parse failed: {}", e);
        }
    }
});

#[derive(Debug, Deserialize)]
pub struct Config {
    pub endpoints: EndpointConfig,
}

#[derive(Debug, Deserialize)]
pub struct EndpointConfig {
    pub grpc: Option<GrpcConfig>,
    pub http: Option<HttpConfig>,
    pub prometheus: Option<PrometheusConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GrpcConfig {
    pub address: String,
    pub ticks_address: Option<String>,
    pub enable: Option<bool>,
    tls: Option<bool>,
}

impl GrpcConfig {
    pub fn enabled(&self) -> bool {
        self.enable.unwrap_or(true)
    }
}

impl GrpcConfig {
    pub fn tls_enabled(&self) -> bool {
        self.tls.unwrap_or(true)
    }
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub address: String,
    pub secret: String,
    pub assets: Option<PathBuf>,
    pub enable: Option<bool>,
    pub admin: Option<UserConfig>,
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub name: String,
    pub password: String,
}
