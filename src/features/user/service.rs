use crate::features::user::error::UserError;
use crate::features::user::model::{CreateUserRequest, UpdateUserRequest, User, UserResponse};
use crate::features::user::repository::UserRepository;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<UserResponse, UserError> {
        // Business Logic: Validate uniqueness
        if self.repo.find_by_username(&req.username).await?.is_some() {
            return Err(UserError::AlreadyExists(req.username));
        }

        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            username: req.username,
            email: req.email,
            created_at: now,
            updated_at: now,
        };

        let created_user = self.repo.create(user).await?;
        Ok(UserResponse::from(created_user))
    }

    pub async fn get_user(&self, id: Uuid) -> Result<UserResponse, UserError> {
        let user = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| UserError::NotFound(id.to_string()))?;

        Ok(UserResponse::from(user))
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, UserError> {
        let mut user = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| UserError::NotFound(id.to_string()))?;

        if let Some(username) = req.username {
            if username != user.username {
                if self.repo.find_by_username(&username).await?.is_some() {
                    return Err(UserError::AlreadyExists(username));
                }
                user.username = username;
            }
        }

        if let Some(email) = req.email {
            user.email = email;
        }

        user.updated_at = Utc::now();

        let updated_user = self.repo.update(user).await?;
        Ok(UserResponse::from(updated_user))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), UserError> {
        // Verify existence first
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| UserError::NotFound(id.to_string()))?;

        self.repo.delete(id).await
    }
}
