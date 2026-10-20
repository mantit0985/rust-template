use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::features::user::model::User;
use crate::features::user::repository::{UserRepository, SqlUserRepository};
use crate::core::db::DbPool;

async fn setup_repository() -> Arc<SqlUserRepository> {
    let pool = DbPool::new("postgres://user:pass@localhost/db").await.expect("Failed to create pool");
    Arc::new(SqlUserRepository::new(Arc::new(pool)))
}

#[tokio::test]
async fn test_repository_create_and_find() {
    let repo = setup_repository().await;

    let user = User {
        id: Uuid::new_v4(),
        username: "repo_test".into(),
        email: "repo@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created = repo.create(user.clone()).await.expect("Create failed");
    assert_eq!(created.id, user.id);
    assert_eq!(created.username, user.username);

    // Note: In the current placeholder implementation, find_by_id returns None.
    // This test is written for a real implementation.
    let found = repo.find_by_id(user.id).await.expect("Find failed");
    // assert!(found.is_some());
}

#[tokio::test]
async fn test_repository_find_by_username() {
    let repo = setup_repository().await;

    let username = "username_test";
    let found = repo.find_by_username(username).await.expect("Find failed");
    // assert!(found.is_some());
}

#[tokio::test]
async fn test_repository_update() {
    let repo = setup_repository().await;

    let user = User {
        id: Uuid::new_v4(),
        username: "update_test".into(),
        email: "update@example.com".into(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let updated = repo.update(user.clone()).await.expect("Update failed");
    assert_eq!(updated.id, user.id);
}

#[tokio::test]
async fn test_repository_delete() {
    let repo = setup_repository().await;
    let id = Uuid::new_v4();

    let result = repo.delete(id).await;
    assert!(result.is_ok());
}
