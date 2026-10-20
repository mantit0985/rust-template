use crate::features::user::handler::{create_user, delete_user, get_user, update_user};
use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/users", post(create_user)).route(
        "/users/:id",
        get(get_user).put(update_user).delete(delete_user),
    )
}
