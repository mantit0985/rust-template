pub mod core;
pub mod features;

use axum::{middleware, Router};
use std::sync::Arc;

pub struct AppState {
    pub db_pool: Arc<core::db::DbPool>,
    pub settings: Arc<core::config::Settings>,
    pub user_service: Arc<features::user::service::UserService>,
}

pub fn create_app(state: Arc<AppState>) -> Router {
    let (prometheus_layer, prometheus_routes) = core::system::prometheus_setup();

    let public_routes = Router::new()
        .merge(core::system::routes())
        .merge(prometheus_routes);

    let protected_routes = Router::new()
        .merge(features::user::routes())
        .layer(middleware::from_fn(core::middleware::auth::auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(prometheus_layer)
        .layer(middleware::from_fn(core::middleware::request_id_middleware))
        .with_state(state)
}
