use axum::Router;
use axum::routing::post;
use crate::controllers::auth_controller::auth_user;

pub fn auth_router() -> Router {
    // Define the user routes
    Router::new()
        .route("/auth", post(auth_user))
}