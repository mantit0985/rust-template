use crate::core::error::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found with ID: {0}")]
    NotFound(String),

    #[error("Username already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<UserError> for AppError {
    fn from(error: UserError) -> Self {
        match error {
            UserError::NotFound(_) => AppError::NotFound,
            UserError::AlreadyExists(msg) => AppError::BadRequest(msg),
            UserError::ValidationError(msg) => AppError::BadRequest(msg),
            UserError::DatabaseError(e) => AppError::Internal(anyhow::anyhow!(e)),
        }
    }
}
