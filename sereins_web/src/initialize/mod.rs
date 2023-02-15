use tokio::fs::read_to_string;
use crate::APPLICATION_CONTEXT;
use crate::initialize::config::ApplicationConfig;

pub mod config;

/// 初始化应用配置
pub async fn init_application_config() {
    let content = read_to_string("application.toml").await.unwrap();

    let config = ApplicationConfig::new(content.as_str());

    // 将配置信息放入到容器中
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}
