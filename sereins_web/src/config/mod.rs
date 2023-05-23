use serde::{Serialize,Deserialize};
use crate::config::database::DatabaseConfig;
use crate::config::redis::RedisConfig;
use crate::config::server::ServerConfig;

pub mod server;
pub mod database;
pub mod redis;

#[derive(Debug, Serialize, Deserialize)]
/// 应用配置
pub struct ApplicationConfig {
    // 调试模式
    pub debug: bool,
    // 日志级别
    pub log_level: String,
    // 服务配置
    pub server_config: ServerConfig,
    // 数据库配置
    pub database: DatabaseConfig,
    // redis
    pub redis:RedisConfig,
}
impl ApplicationConfig {
    // 实例化配置
    pub fn new(config: &str) -> Self {
        match toml::from_str(config) {
            Ok(e) => e,
            Err(e) => panic!("{}", e)
        }
    }
}
