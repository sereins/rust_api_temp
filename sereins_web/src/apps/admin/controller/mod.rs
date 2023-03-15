use axum::Router;
use crate::apps::admin::controller;

pub mod user;
pub mod system_config;

pub fn routers() -> Router{

    let routers = Router::new()
        .merge(user::export_router())
        .merge(system_config::export_router());
    routers
}