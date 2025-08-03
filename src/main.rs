use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use axum::routing::get;
use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let db: DatabaseConnection = Database::connect("protocol://username:password@host/database").await.expect("Database connection failed");

    // build our application with a single route
    let app = Router::new()
        .route("/", get(home))
        .route("/test", get(test))
        // handle 404 not found
        .fallback(not_found);

    // run our app with hyper, listening globally on port 5000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    let app_res= axum::serve(listener, app).await;
    match app_res {
        Ok(_) => println!("Server running on http://localhost:5000"),
        Err(e) => eprintln!("Server error: {}", e),
    }
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
