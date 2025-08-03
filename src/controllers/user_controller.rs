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
    let users = entity::user::Entity::find().all(&db).await.unwrap();
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

    Json(json!({"users": users}))
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
        .await
        .unwrap();
    if user_exists.is_some() {
        return (
            StatusCode::CONFLICT,
            Json(json!({"error": "User already exists"})),
        );
    }

    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(u.name),
        email: Set(u.email),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let created_user = new_user.insert(&db).await.unwrap();
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

pub async fn update_user(
    Extension(db): Extension<DatabaseConnection>,
    Json(u): Json<UserModelUpdate>,
) -> impl IntoResponse {
    // Check if the user ID is provided
    let user_exists = entity::user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(u.id.clone())))
        .one(&db)
        .await
        .unwrap();

    if user_exists.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User does not exist"})),
        );
    }

    let mut user: user::ActiveModel = user_exists.unwrap().into();
    if let Some(name) = u.name {
        user.name = Set(name);
    }
    if let Some(email) = u.email {
        user.email = Set(email);
    }

    let updated_user = user.update(&db).await.unwrap();
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

pub async fn delete_user(
    Extension(db): Extension<DatabaseConnection>,
    Path(uuid): Path<Uuid>,
) -> impl IntoResponse {
    // Check if the user exists
    let user_exists = entity::user::Entity::find()
        .filter(Condition::all().add(user::Column::Id.eq(uuid)))
        .one(&db)
        .await
        .unwrap();

    if user_exists.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User does not exist"})),
        );
    }
    user::Entity::delete_by_id(uuid).exec(&db).await.unwrap();
    // This function is not implemented yet
    (
        StatusCode::OK,
        Json(json!({"message": "User deleted successfully"})),
    )
}
