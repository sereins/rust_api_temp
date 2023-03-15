use axum::{Router, routing};
use axum::response::IntoResponse;

/// 返回配置列表
pub async fn list()->impl IntoResponse{
    "SYS_CONFIG_LIST"
}

pub fn export_router() -> Router {
    let router = Router::new()
        .route("/list", routing::get(list));
    router
}





