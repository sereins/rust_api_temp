use axum::{Router, routing};
use axum::response::IntoResponse;
use sqlx::mysql::MySqlPoolOptions;
use crate::apps::admin::domain::{adds, admin};
use crate::config::ApplicationConfig;
use crate::initialize::APPLICATION_CONTEXT;

/// 用户登录
pub async fn login() -> impl IntoResponse {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let db_config = &config.database;
    let url = db_config.get_url();

    let pool = MySqlPoolOptions::new()
        .connect(&*url)
        .await
        .unwrap();

    let res = admin(&pool).await;

    serde_json::to_string(&res).unwrap()
}

// 添加测试
pub async fn add() -> impl IntoResponse {
    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let db_config = &config.database;
    let url = db_config.get_url();

    let pool = MySqlPoolOptions::new()
        .connect(&*url)
        .await
        .unwrap();

    let res = adds(&pool).await;

    serde_json::to_string(&res).unwrap()
}

/// 到处用户相关的模块
pub fn export_router() -> Router {
    let router = Router::new()
        .route("/user", routing::get(login))
        .route("/add", routing::get(add));
    router
}

