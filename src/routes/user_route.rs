use crate::controllers::user_controller::{create_user, delete_user, get_all_users, update_user};
use axum::Router;
use axum::routing::{delete, get, patch, post};

pub fn user_router() -> Router {
    // Define the user routes
    Router::new()
        .route("/users", get(get_all_users))
        .route("/users-create", post(create_user))
        .route("/users-update", patch(update_user))
        .route("/users-delete/{uuid}", delete(delete_user))
}
