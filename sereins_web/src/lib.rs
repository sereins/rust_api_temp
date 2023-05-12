extern crate core;

use state::Container;
use crate::initialize::config::ApplicationConfig;
use crate::initialize::init_application_config;

pub mod apps;
pub mod initialize;

/// 整个项目上下文ApplicationContext
pub static APPLICATION_CONTEXT: Container![Send + Sync] = <Container![Send + Sync]>::new();

/// 初始化应用的上下文配置
pub async fn init_context() {
    // 初始化应用配置
    init_application_config().await;
    // 日志
    init_log();
}

/// 日志初始化
pub fn init_log() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}