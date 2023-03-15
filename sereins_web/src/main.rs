use axum::{Router, Server};
use log::info;
use sereis_web::apps::admin::controller::routers;
use sereis_web::{APPLICATION_CONTEXT, init_context};
use sereis_web::initialize::config::{ApplicationConfig};

#[tokio::main]
async fn main() {
    // 初始化项目
    init_context().await;

    // 项目路由
    let app = Router::new()
        .nest("/admin", routers())
        .fallback(|| async { "not found" });

    let config = APPLICATION_CONTEXT.get::<ApplicationConfig>();
    let addr = config.server_config.service_format();

    // 启动服务
    info!("http服务启动:{}",config.server_config.http_addr());
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}