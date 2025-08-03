mod models;

use crate::models::user_model::{UserModel, UserModelCreate};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use entity::user;
use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, Set};
use serde_json::json;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/test", get(test))
        .route("/users-create", post(create_user))
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

async fn create_user(Json(u): Json<UserModelCreate>) -> impl IntoResponse {
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection).await.unwrap();

    let new_user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(u.name),
        email: Set(u.email),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let created_user = new_user.insert(&db).await.unwrap();
    (
        StatusCode::CREATED,
        Json(json!({
            "id": created_user.id,
            "name": created_user.name,
            "email": created_user.email,
            "created_at": created_user.created_at,
            "updated_at": created_user.updated_at,
        })),
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
