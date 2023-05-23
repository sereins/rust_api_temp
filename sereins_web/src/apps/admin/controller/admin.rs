use axum::{Router, routing};
use axum::response::IntoResponse;
use crate::apps::resp::RespVo;

/// 导出路由
pub fn export_routers() -> Router {
    Router::new()
        .route("/admin/login", routing::get(login))
}

/// 登录
pub async fn login() -> impl IntoResponse {
    RespVo::success("success".to_string()).resp_json()
}