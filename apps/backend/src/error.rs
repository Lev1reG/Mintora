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
    // Authentication errors
    Unauthorized(String),
    Forbidden,
    InvalidCredentials,
    TokenExpired,
    TokenInvalid,
    // Validation errors
    ValidationError(String),
    // Hashing errors
    HashError,
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
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
                Some(msg),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "Forbidden".to_string(),
                None,
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials".to_string(),
                Some("Email or password is incorrect".to_string()),
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Token expired".to_string(),
                Some("Please refresh your token or login again".to_string()),
            ),
            AppError::TokenInvalid => (
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
                None,
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation error".to_string(),
                Some(msg),
            ),
            AppError::HashError => {
                tracing::error!("Password hashing error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                    None,
                )
            }
        };

        (status, Json(ErrorResponse { error, details })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_: argon2::password_hash::Error) -> Self {
        AppError::HashError
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::TokenInvalid,
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        AppError::ValidationError(err.to_string())
    }
}
