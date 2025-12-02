use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, HeaderMap},
};
use crate::{
    app_state::AppState,
    db::{models::User, repositories::{ApiKeyRepository, UserRepository}},
    error::AppError,
    utils::api_key::{hash_api_key, validate_api_key_format},
};

/// API Key authenticated user extractor
/// Use this in handlers that accept API key authentication
pub struct ApiKeyAuth(pub User);

#[async_trait]
impl FromRequestParts<AppState> for ApiKeyAuth {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract X-API-Key header
        let headers: &HeaderMap = &parts.headers;
        let api_key = headers
            .get("X-API-Key")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing API key".to_string()))?;

        // Validate API key format
        if !validate_api_key_format(api_key) {
            return Err(AppError::Unauthorized("Invalid API key format".to_string()));
        }

        // Hash the API key
        let key_hash = hash_api_key(api_key);

        // Lookup API key in database
        let api_key_record = ApiKeyRepository::find_by_hash(&state.db, &key_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Invalid API key".to_string()))?;

        // Check if API key is valid (not revoked, not expired)
        if !api_key_record.is_valid() {
            return Err(AppError::Unauthorized("API key is revoked or expired".to_string()));
        }

        // Fetch user from database
        let user = UserRepository::find_by_id(&state.db, api_key_record.user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        // Check if user is active
        if !user.is_active() {
            return Err(AppError::Forbidden);
        }

        // Update last_used_at timestamp (fire-and-forget)
        let pool = state.db.clone();
        let hash = key_hash.clone();
        tokio::spawn(async move {
            let _ = ApiKeyRepository::update_last_used(&pool, &hash).await;
        });

        Ok(ApiKeyAuth(user))
    }
}
