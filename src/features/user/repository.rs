use crate::core::db::DbPool;
use crate::features::user::error::UserError;
use crate::features::user::model::User;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<User, UserError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError>;
    async fn update(&self, user: User) -> Result<User, UserError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;
}

pub struct SqlUserRepository {
    #[allow(dead_code)]
    pool: Arc<DbPool>,
}

impl SqlUserRepository {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for SqlUserRepository {
    async fn create(&self, user: User) -> Result<User, UserError> {
        tracing::info!("SQL: Creating user {} using pool", user.username);
        // In real implementation:
        // sqlx::query_as!(User, "INSERT INTO users ...").fetch_one(&self.pool).await...
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        tracing::info!("SQL: Finding user by ID {} using pool", id);
        Ok(None)
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
        tracing::info!("SQL: Finding user by username {} using pool", username);
        Ok(None)
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        tracing::info!("SQL: Updating user {} using pool", user.id);
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        tracing::info!("SQL: Deleting user {} using pool", id);
        Ok(())
    }
}
