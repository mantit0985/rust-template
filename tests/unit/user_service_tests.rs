use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::Utc;
use async_trait::async_trait;
use crate::features::user::model::{User, UserResponse, CreateUserRequest, UpdateUserRequest};
use crate::features::user::repository::UserRepository;
use crate::features::user::error::UserError;
use crate::features::user::service::UserService;

// Manual Mock for UserRepository
struct MockUserRepository {
    users: Mutex<Vec<User>>,
    should_fail: Mutex<bool>,
}

impl MockUserRepository {
    fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
            should_fail: Mutex::new(false),
        }
    }

    fn set_fail(&self, fail: bool) {
        *self.should_fail.lock().unwrap() = fail;
    }
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, user: User) -> Result<User, UserError> {
        if *self.should_fail.lock().unwrap() {
            return Err(UserError::DatabaseError("Mock DB Error".into()));
        }
        let mut users = self.users.lock().unwrap();
        users.push(user.clone());
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, UserError> {
        if *self.should_fail.lock().unwrap() {
            return Err(UserError::DatabaseError("Mock DB Error".into()));
        }
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.id == id).cloned())
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, UserError> {
        if *self.should_fail.lock().unwrap() {
            return Err(UserError::DatabaseError("Mock DB Error".into()));
        }
        let users = self.users.lock().unwrap();
        Ok(users.iter().find(|u| u.username == username).cloned())
    }

    async fn update(&self, user: User) -> Result<User, UserError> {
        if *self.should_fail.lock().unwrap() {
            return Err(UserError::DatabaseError("Mock DB Error".into()));
        }
        let mut users = self.users.lock().unwrap();
        if let Some(pos) = users.iter().position(|u| u.id == user.id) {
            users[pos] = user.clone();
            Ok(user)
        } else {
            Err(UserError::NotFound(user.id.to_string()))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        if *self.should_fail.lock().unwrap() {
            return Err(UserError::DatabaseError("Mock DB Error".into()));
        }
        let mut users = self.users.lock().unwrap();
        if let Some(pos) = users.iter().position(|u| u.id == id) {
            users.remove(pos);
            Ok(())
        } else {
            Err(UserError::NotFound(id.to_string()))
        }
    }
}

#[tokio::test]
async fn test_create_user_success() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo);

    let req = CreateUserRequest {
        username: "testuser".into(),
        email: "test@example.com".into(),
    };

    let result = service.create_user(req).await;
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_create_user_already_exists() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo.clone());

    let req = CreateUserRequest {
        username: "duplicate".into(),
        email: "test1@example.com".into(),
    };
    service.create_user(req).await.unwrap();

    let req2 = CreateUserRequest {
        username: "duplicate".into(),
        email: "test2@example.com".into(),
    };
    let result = service.create_user(req2).await;
    assert!(matches!(result, Err(UserError::AlreadyExists(_))));
}

#[tokio::test]
async fn test_get_user_success() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo.clone());

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        username: "found".into(),
        email: "found@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.users.lock().unwrap().push(user);

    let result = service.get_user(user_id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, user_id);
}

#[tokio::test]
async fn test_get_user_not_found() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo);

    let result = service.get_user(Uuid::new_v4()).await;
    assert!(matches!(result, Err(UserError::NotFound(_))));
}

#[tokio::test]
async fn test_update_user_success() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo.clone());

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        username: "old_name".into(),
        email: "old@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.users.lock().unwrap().push(user);

    let req = UpdateUserRequest {
        username: Some("new_name".into()),
        email: Some("new@example.com".into()),
    };

    let result = service.update_user(user_id, req).await;
    assert!(result.is_ok());
    let updated = result.unwrap();
    assert_eq!(updated.username, "new_name");
    assert_eq!(updated.email, "new@example.com");
}

#[tokio::test]
async fn test_update_user_username_already_exists() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo.clone());

    let user1_id = Uuid::new_v4();
    let user1 = User {
        id: user1_id,
        username: "user1".into(),
        email: "u1@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let user2_id = Uuid::new_v4();
    let user2 = User {
        id: user2_id,
        username: "user2".into(),
        email: "u2@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.users.lock().unwrap().extend(vec![user1, user2]);

    let req = UpdateUserRequest {
        username: Some("user2".into()),
        email: None,
    };

    let result = service.update_user(user1_id, req).await;
    assert!(matches!(result, Err(UserError::AlreadyExists(_))));
}

#[tokio::test]
async fn test_delete_user_success() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo.clone());

    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id,
        username: "todelete".into(),
        email: "del@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.users.lock().unwrap().push(user);

    let result = service.delete_user(user_id).await;
    assert!(result.is_ok());
    assert!(repo.users.lock().unwrap().is_empty());
}

#[tokio::test]
async fn test_delete_user_not_found() {
    let repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(repo);

    let result = service.delete_user(Uuid::new_v4()).await;
    assert!(matches!(result, Err(UserError::NotFound(_))));
}
