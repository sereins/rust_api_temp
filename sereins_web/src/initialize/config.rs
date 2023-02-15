use serde::{Deserialize, Serialize};

// 服务的配置
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    ip: Vec<u8>,
    port: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationConfig {
    // 调试模式
    pub debug: bool,
    // 日志级别
    pub log_level: String,
    // 服务配置
    pub server_config: ServerConfig,
}

impl ApplicationConfig {
    // 实例化配置
    pub fn new(config: &str) -> Self {
        let config = match toml::from_str(config) {
            Ok(e) => e,
            Err(e) => panic!("{}", e)
        };
        config
    }
}