use crate::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

pub fn prometheus_setup() -> (
    axum_prometheus::PrometheusMetricLayer<'static>,
    Router<Arc<AppState>>,
) {
    let (layer, handle) = axum_prometheus::PrometheusMetricLayer::pair();
    let routes = Router::new().route("/metrics", get(move || async move { handle.render() }));
    (layer, routes)
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/readyz", get(readiness_check))
        .route("/version", get(version_check))
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn readiness_check(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    if state.db_pool.health_check().await.is_err() {
        return StatusCode::SERVICE_UNAVAILABLE;
    }
    StatusCode::OK
}

async fn version_check() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");
    (StatusCode::OK, version)
}
