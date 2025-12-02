use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use crate::{
    app_state::AppState,
    db::{models::User, repositories::UserRepository},
    error::AppError,
    utils::jwt::{extract_user_id, validate_token, TokenType},
};

/// Authenticated user extractor (required authentication)
/// Use this in handlers that require a valid JWT token
pub struct AuthUser(pub User);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized("Missing authorization token".to_string()))?;

        // Validate JWT token
        let claims = validate_token(
            bearer.token(),
            &state.config.jwt.access_secret,
            TokenType::Access,
        )?;

        // Extract user_id from claims
        let user_id = extract_user_id(&claims)?;

        // Fetch user from database
        let user = UserRepository::find_by_id(&state.db, user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized("User not found".to_string()))?;

        // Check if user is active
        if !user.is_active() {
            return Err(AppError::Forbidden);
        }

        Ok(AuthUser(user))
    }
}

/// Optional authenticated user extractor
/// Use this in handlers where authentication is optional
pub struct OptionalAuthUser(pub Option<User>);

#[async_trait]
impl FromRequestParts<AppState> for OptionalAuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Try to extract Authorization header
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok();

        // If no token, return None
        let Some(TypedHeader(Authorization(bearer))) = bearer else {
            return Ok(OptionalAuthUser(None));
        };

        // Try to validate token
        let claims = validate_token(
            bearer.token(),
            &state.config.jwt.access_secret,
            TokenType::Access,
        )
        .ok();

        // If token invalid, return None
        let Some(claims) = claims else {
            return Ok(OptionalAuthUser(None));
        };

        // Try to extract user_id
        let user_id = extract_user_id(&claims).ok();

        // If user_id invalid, return None
        let Some(user_id) = user_id else {
            return Ok(OptionalAuthUser(None));
        };

        // Try to fetch user from database
        let user = UserRepository::find_by_id(&state.db, user_id)
            .await
            .ok()
            .flatten();

        // If user not found or inactive, return None
        let Some(user) = user else {
            return Ok(OptionalAuthUser(None));
        };

        if !user.is_active() {
            return Ok(OptionalAuthUser(None));
        }

        Ok(OptionalAuthUser(Some(user)))
    }
}
