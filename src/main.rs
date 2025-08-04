mod controllers;
mod models;
mod routes;
mod utils;
mod middlewares;

use axum::http::{Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use sea_orm::{Database, DatabaseConnection};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Get database URL from environment variable or use a default
    let db_connection = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/rust-vehicle-api".to_string()
    });
    let db: DatabaseConnection = Database::connect(db_connection)
        .await
        .expect("Database connection failed.");

    // configure CORS middleware
    let cors = tower_http::cors::CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/test", get(test))
        // include the user routes
        .merge(routes::user_route::user_router())
        // handle 404 not found
        .fallback(not_found)
        // add CORS middleware;
        .layer(cors)
        // add database connection as a shared state
        .layer(Extension(db));

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    let app_res = axum::serve(listener, app).await;
    match app_res {
        Ok(_) => println!("Server running on http://localhost:5000"),
        Err(e) => eprintln!("Server error: {}", e),
    }
}

async fn home() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "message": "Welcome to the Rust Vehicle API",
            "status": "success"
        })),
    )
}

async fn test() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "message": "Test route is working!",
            "status": "success"
        })),
    )
}

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "Resource not found"
        })),
    )
}
