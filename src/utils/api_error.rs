use axum::http::{header, StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code;
        (status, [(header::CONTENT_TYPE, "application/json")], Json(json!({
            "status": status.as_u16(),
            "message": self.message,
            "error": true,
        }))).into_response()
    }
}