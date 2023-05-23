use axum::Router;

pub mod controller;
pub mod entities;
pub mod domain;

/// 导出admin应用下的所有路由
pub fn routers() -> Router {
    Router::new()
        /* -----------------管理员-----------------*/
        .merge(controller::admin::export_routers())
}
