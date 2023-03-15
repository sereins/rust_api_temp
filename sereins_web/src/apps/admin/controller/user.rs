use axum::{Router, routing};
use axum::response::IntoResponse;

/// 用户登录
pub async fn user_login() -> impl IntoResponse {
    "hello"
}


/// 到处用户相关的模块
pub fn export_router() -> Router {
    let router = Router::new()
        .route("/user", routing::get(user_login));
    router
}

