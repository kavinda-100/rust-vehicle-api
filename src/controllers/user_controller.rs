use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use chrono::Utc;
use migration::Condition;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait};
use sea_orm::{DatabaseConnection, Set};
use serde_json::json;
use uuid::Uuid;

use crate::models::user_model::{UserModel, UserModelCreate, UserModelUpdate};
use entity::user;

pub async fn get_all_users(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    // Fetch all users from the database
    let users = entity::user::Entity::find().all(&db).await;
    // Handle the result of the query
    return match users {
        Ok(users) => {
            let users: Vec<UserModel> = users
                .into_iter()
                .map(|user| UserModel {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                })
                .collect();

            (
                StatusCode::OK,
                Json(json!({"message": "Users fetched successfully", "users": users})),
            )
        }
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
        }
    };
}

pub async fn create_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(u): Json<UserModelCreate>,
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
    match user_exists {
        Ok(Some(_)) => {
            return (
                StatusCode::CONFLICT,
                Json(json!({"error": "User already exists"})),
            );
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Error checking user existence: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            );
        }
    }
    // Create a new user
    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(u.name),
        email: Set(u.email),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };
    // Insert the new user into the database
    let created_user = new_user.insert(&db).await;
    // Handle the result of the insert operation
    return match created_user {
        Ok(created_user) => {
            // Convert the created user to UserModel for response
            let created_user = UserModel {
                id: created_user.id,
                name: created_user.name,
                email: created_user.email,
                created_at: created_user.created_at,
                updated_at: created_user.updated_at,
            };
            (
                StatusCode::CREATED,
                Json(json!({"message": "User created successfully", "user": created_user})),
            )
        }
        Err(e) => {
            eprintln!("Error creating user: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
        }
    };
}

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(u): Json<UserModelUpdate>,
) -> impl IntoResponse {
    // Check if the user ID is provided
    let user_exists = entity::user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(u.id.clone())))
        .one(&db)
        .await;

    // Handle the result of the query
    return match user_exists {
        Ok(Some(o_user)) => {
            // If the user exists, update the user details
            // Convert the incoming UserModelUpdate to ActiveModel
            let mut user: user::ActiveModel = o_user.into();
            // Update the fields if they are provided
            if let Some(name) = u.name {
                user.name = Set(name);
            }
            if let Some(email) = u.email {
                user.email = Set(email);
            }
            user.updated_at = Set(Utc::now().naive_utc());
            // Update the user
            let updated_user = user.update(&db).await;
            // Handle the result of the update operation
            match updated_user {
                Ok(updated_user) => {
                    // Convert the updated user to UserModel for response
                    let updated_user = UserModel {
                        id: updated_user.id,
                        name: updated_user.name,
                        email: updated_user.email,
                        created_at: updated_user.created_at,
                        updated_at: updated_user.updated_at,
                    };
                    (
                        StatusCode::OK,
                        Json(json!({"message": "User updated successfully", "user": updated_user})),
                    )
                }
                Err(e) => {
                    eprintln!("Error updating user: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Internal server error"})),
                    )
                }
            }
        }
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User does not exist"})),
        ),
        Err(e) => {
            eprintln!("Error checking user existence: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
        }
    };
}

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    // Check if the user exists
    let user_exists = entity::user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(uuid)))
        .one(&db)
        .await;

    // Handle the result of the query
    if let Err(e) = user_exists {
        eprintln!("Error checking user existence: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        );
    }

    let result = user::Entity::delete_by_id(uuid).exec(&db).await;
    // Handle the result of the delete operation
    if let Err(e) = result {
        eprintln!("Error deleting user: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Internal server error"})),
        );
    }
    // This function is not implemented yet
    (
        StatusCode::OK,
        Json(json!({"message": "User deleted successfully"})),
    )
}
