use state::Container;
use tokio::fs::read_to_string;
use crate::config::ApplicationConfig;

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

/// 初始化应用配置
pub async fn init_application_config() {
    let content = read_to_string("application.toml").await.unwrap();

    let config = ApplicationConfig::new(content.as_str());

    // 将配置信息放入到容器中
    APPLICATION_CONTEXT.set::<ApplicationConfig>(config);
}

