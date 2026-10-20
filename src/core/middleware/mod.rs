pub mod auth;

use axum::{extract::Request, middleware::Next, response::Response};
use tracing::{info_span, Instrument};
use uuid::Uuid;

pub async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    // Add request ID to extensions so it's available in handlers
    let mut request = request;
    request
        .extensions_mut()
        .insert(RequestId(request_id.clone()));

    let span = info_span!("request", request_id = %request_id);

    next.run(request).instrument(span).await
}

#[derive(Clone, Debug)]
pub struct RequestId(pub String);
