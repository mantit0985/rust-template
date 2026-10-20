use crate::core::error::AppError;
use crate::features::user::model::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = state.user_service.create_user(payload).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.get_user(id).await?;
    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = state.user_service.update_user(id, payload).await?;
    Ok(Json(user))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    state.user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
