use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::models::UserRole;
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // user_id
    pub email: String,
    pub role: String,
    pub exp: i64,           // expiration timestamp
    pub iat: i64,           // issued at timestamp
    pub token_type: TokenType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

/// Generate access token (short-lived)
pub fn generate_access_token(
    user_id: Uuid,
    email: &str,
    role: &UserRole,
    secret: &str,
    expiry_seconds: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::seconds(expiry_seconds);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: format!("{:?}", role).to_lowercase(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
        token_type: TokenType::Access,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Generate refresh token (long-lived)
pub fn generate_refresh_token(
    user_id: Uuid,
    email: &str,
    role: &UserRole,
    secret: &str,
    expiry_seconds: i64,
) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::seconds(expiry_seconds);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: format!("{:?}", role).to_lowercase(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
        token_type: TokenType::Refresh,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Validate and decode token
pub fn validate_token(
    token: &str,
    secret: &str,
    expected_type: TokenType,
) -> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    // Verify token type
    if token_data.claims.token_type != expected_type {
        return Err(AppError::TokenInvalid);
    }

    Ok(token_data.claims)
}

/// Extract user_id from token
pub fn extract_user_id(claims: &Claims) -> Result<Uuid, AppError> {
    Uuid::parse_str(&claims.sub).map_err(|_| AppError::TokenInvalid)
}
