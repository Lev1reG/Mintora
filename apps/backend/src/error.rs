use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    NotFound,
    BadRequest(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error, details) = match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                    None,
                )
            }
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                None,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "Bad request".to_string(),
                Some(msg),
            ),
        };

        (status, Json(ErrorResponse { error, details })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}
