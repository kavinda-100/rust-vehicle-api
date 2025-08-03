use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
mod models;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use chrono::Utc;
use chrono::format::Pad;
use migration::Condition;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;

use crate::models::user_model::{UserModel, UserModelCreate, UserModelUpdate};
use entity::user;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/test", get(test))
        .route("/users", get(get_all_users))
        .route("/users-create", post(create_user))
        .route("/users-update", patch(update_user))
        .route("/users-delete/{uuid}", delete(delete_user))
        // handle 404 not found
        .fallback(not_found);

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    let app_res = axum::serve(listener, app).await;
    match app_res {
        Ok(_) => println!("Server running on http://localhost:5000"),
        Err(e) => eprintln!("Server error: {}", e),
    }
}

async fn get_all_users() -> impl IntoResponse {
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection).await.unwrap();

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

async fn create_user(Json(u): Json<UserModelCreate>) -> impl IntoResponse {
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection).await.unwrap();

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

async fn update_user(Json(u): Json<UserModelUpdate>) -> impl IntoResponse {
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection).await.unwrap();

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

async fn delete_user(Path(uuid): Path<Uuid>) -> impl IntoResponse {
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection).await.unwrap();

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

async fn home() -> &'static str {
    "Hello, World from Axum With Rust! 🦀"
}

async fn test() -> impl IntoResponse {
    let response = "Hello, Axum!";
    // Respond with a JSON object
    (StatusCode::OK, Json(response))
}

async fn not_found() -> &'static str {
    "404 Not Found"
}
