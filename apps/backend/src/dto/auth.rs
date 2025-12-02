use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use chrono::NaiveDateTime;

// ============================================================================
// Registration
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 3, max = 50, message = "Username must be between 3 and 50 characters"))]
    pub username: String,

    #[validate(length(min = 1, max = 100, message = "Full name must be between 1 and 100 characters"))]
    pub full_name: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

// ============================================================================
// Login
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
}

// ============================================================================
// Refresh Token
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

// ============================================================================
// Logout
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct LogoutRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
}

// ============================================================================
// Me (Current User)
// ============================================================================

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user: UserResponse,
}

// ============================================================================
// API Keys
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct CreateApiKeyRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    pub is_live: bool,

    #[serde(default)]
    pub expires_in_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,  // Full key shown once
    pub key_prefix: String,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyListItem {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ListApiKeysResponse {
    pub api_keys: Vec<ApiKeyListItem>,
}

#[derive(Debug, Deserialize)]
pub struct RevokeApiKeyRequest {
    // Path parameter, not in body
}

#[derive(Debug, Serialize)]
pub struct RevokeApiKeyResponse {
    pub message: String,
}

// ============================================================================
// Shared User Response
// ============================================================================

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: String,
    pub full_name: String,
    pub role: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}
