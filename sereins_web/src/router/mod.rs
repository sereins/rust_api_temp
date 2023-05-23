use axum::{Router};
use axum::http::header::{ACCEPT, AUTHORIZATION};
use axum::http::{HeaderName, Method};
use axum::response::{IntoResponse};
use axum::routing::get;
use crate::apps::admin::controllers;
use crate::common::RespVo;
use crate::ws::handler;
use tower_http::cors::{Any, CorsLayer};


/// 项目中的所有路由
pub fn app_router() -> Router {
    let token = "token".parse::<HeaderName>().unwrap();
    let app = Router::new()
        .nest("/api", controllers::routers())
        .route("/ws/:token", get(handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, ACCEPT, token])
                .allow_methods([Method::GET,Method::POST,Method::PUT,Method::OPTIONS,Method::DELETE])
        )
        .fallback(not_found_handle);
    app
}

/// 未定义路由
async fn not_found_handle() -> impl IntoResponse {
    RespVo::fail(&"接口未定义".to_string()).resp_json()
}