use axum::Router;

pub mod user;

pub fn routers() -> Router {
    Router::new()
        .merge(user::export_router())
}