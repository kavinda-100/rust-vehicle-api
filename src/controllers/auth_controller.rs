use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait};
use sea_orm::QueryFilter;
use serde_json::json;
use entity::user;
use migration::Condition;
use crate::models::auth_model::{UserAuthModel, UserAuthentication};
use crate::utils::jwt::encode_jwt;

pub async fn auth_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(u): Json<UserAuthModel>,
) -> impl IntoResponse {
    // Check if the user already exists
    let user_exists = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(user::Column::Email.eq(u.email.clone()))
                .add(user::Column::Name.eq(u.name.clone())),
        )
        .one(&db)
        .await;
    // Handle the result of the query
    return match user_exists {
        Ok(Some(user)) => {
            // get the JWT
            let jwt_res = encode_jwt(user.id);
            // Handle JWT generation result
            match jwt_res {
                Ok(jwt) => {
                    // If user exists, generate JWT and return authentication details
                    let auth_user = UserAuthentication {
                        jwt,
                        name: u.name.clone(),
                        email: u.email.clone(),
                    };
                    (
                        StatusCode::OK,
                        Json(json!({
                    "message": "User authenticated successfully",
                    "data": auth_user
                })),
                    )
                }
                Err(e) => {
                    eprintln!("Error generating JWT: {}", e);
                    (
                        e,
                        Json(json!({"error": "Internal server error"})),
                    )
                }
            }
        }
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "User not found"})),
            )
        }
        Err(e) => {
            eprintln!("Error checking user existence: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
        }
    }
}