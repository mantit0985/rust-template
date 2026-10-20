use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::OnceCell;
use tower::util::ServiceExt;
use uuid::Uuid;

use rust_template::core::{DbPool, Settings};
use rust_template::features::user::{repository::SqlUserRepository, service::UserService};
use rust_template::{create_app, AppState};

static APP: OnceCell<Router> = OnceCell::const_new();

async fn get_app() -> Router {
    APP.get_or_init(|| async {
        let settings = Arc::new(Settings {
            database_url: "postgres://user:pass@localhost/test".to_string(),
            server_port: 8080,
            env: "test".to_string(),
        });

        let db_pool = Arc::new(DbPool::new(&settings.database_url).await.unwrap());
        let user_repo = Arc::new(SqlUserRepository::new(db_pool.clone()));
        let user_service = Arc::new(UserService::new(user_repo));

        let state = Arc::new(AppState {
            db_pool,
            settings,
            user_service,
        });

        create_app(state)
    })
    .await
    .clone()
}

#[tokio::test]
async fn test_health_check_endpoint() {
    let app = get_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(body, "OK");
}

#[tokio::test]
async fn test_create_user_api() {
    let app = get_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .header("authorization", "Bearer secret-token")
                .body(Body::from(
                    json!({
                        "username": "e2e_user",
                        "email": "e2e@example.com"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_user_api() {
    let app = get_app().await;
    let user_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/users/{}", user_id))
                .header("authorization", "Bearer secret-token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // In a real test with a mock DB, this would return 200 or 404
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND);
}
