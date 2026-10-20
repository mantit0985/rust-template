use crate::core::error::AppError;
use axum::{extract::Request, http::header, middleware::Next, response::Response};

pub async fn auth_middleware(request: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    match auth_header {
        Some(auth_value) if auth_value.starts_with("Bearer ") => {
            // Placeholder: In a real app, validate the token here
            let token = &auth_value[7..];
            if token == "secret-token" {
                Ok(next.run(request).await)
            } else {
                Err(AppError::Unauthorized)
            }
        }
        _ => Err(AppError::Unauthorized),
    }
}
