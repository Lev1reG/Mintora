use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use validator::Validate;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    db::repositories::{ApiKeyRepository, RefreshTokenRepository, UserRepository},
    dto::auth::*,
    error::AppError,
    middleware::AuthUser,
    utils::{
        api_key::generate_api_key,
        jwt::{generate_access_token, generate_refresh_token, validate_token, extract_user_id, TokenType},
        password::{hash_password, verify_password},
    },
};

// ============================================================================
// POST /auth/register - Register new user with email/password
// ============================================================================
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    // Validate input
    payload.validate()?;

    // Check if email already exists
    if UserRepository::email_exists(&state.db, &payload.email).await? {
        return Err(AppError::BadRequest("Email already registered".to_string()));
    }

    // Check if username already exists
    if UserRepository::username_exists(&state.db, &payload.username).await? {
        return Err(AppError::BadRequest("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Create user
    let user = UserRepository::create_with_password(
        &state.db,
        &payload.email,
        &payload.username,
        &payload.full_name,
        &password_hash,
    )
    .await?;

    // Generate tokens
    let access_token = generate_access_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.access_secret,
        state.config.jwt.access_expiry_seconds,
    )?;

    let refresh_token_str = generate_refresh_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.refresh_secret,
        state.config.jwt.refresh_expiry_seconds,
    )?;

    // Store refresh token in database
    let token_hash = crate::utils::api_key::hash_api_key(&refresh_token_str);
    RefreshTokenRepository::create(
        &state.db,
        user.id,
        &token_hash,
        state.config.jwt.refresh_expiry_seconds,
        None, // device_info
        None, // ip_address
        None, // user_agent
    )
    .await?;

    let safe_user = user.to_safe_user();
    let user_response = UserResponse {
        id: safe_user.id,
        email: safe_user.email,
        username: safe_user.username,
        full_name: safe_user.full_name,
        role: format!("{:?}", safe_user.role).to_lowercase(),
        status: format!("{:?}", safe_user.status).to_lowercase(),
        created_at: safe_user.created_at,
    };

    Ok(Json(RegisterResponse {
        user: user_response,
        access_token,
        refresh_token: refresh_token_str,
    }))
}

// ============================================================================
// POST /auth/login - Login with email/password
// ============================================================================
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate input
    payload.validate()?;

    // Find user by email
    let user = UserRepository::find_by_email(&state.db, &payload.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // Check if user has dashboard access (email + password)
    if !user.has_dashboard_access() {
        return Err(AppError::InvalidCredentials);
    }

    // Verify password
    let password_hash = user.password_hash.as_ref().unwrap();
    if !verify_password(&payload.password, password_hash)? {
        return Err(AppError::InvalidCredentials);
    }

    // Check if user is active
    if !user.is_active() {
        return Err(AppError::Forbidden);
    }

    // Generate tokens
    let access_token = generate_access_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.access_secret,
        state.config.jwt.access_expiry_seconds,
    )?;

    let refresh_token_str = generate_refresh_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.refresh_secret,
        state.config.jwt.refresh_expiry_seconds,
    )?;

    // Store refresh token in database
    let token_hash = crate::utils::api_key::hash_api_key(&refresh_token_str);
    RefreshTokenRepository::create(
        &state.db,
        user.id,
        &token_hash,
        state.config.jwt.refresh_expiry_seconds,
        None, // device_info
        None, // ip_address
        None, // user_agent
    )
    .await?;

    let safe_user = user.to_safe_user();
    let user_response = UserResponse {
        id: safe_user.id,
        email: safe_user.email,
        username: safe_user.username,
        full_name: safe_user.full_name,
        role: format!("{:?}", safe_user.role).to_lowercase(),
        status: format!("{:?}", safe_user.status).to_lowercase(),
        created_at: safe_user.created_at,
    };

    Ok(Json(LoginResponse {
        user: user_response,
        access_token,
        refresh_token: refresh_token_str,
    }))
}

// ============================================================================
// POST /auth/refresh - Refresh access token using refresh token
// ============================================================================
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResponse>, AppError> {
    // Validate input
    payload.validate()?;

    // Validate refresh token
    let claims = validate_token(
        &payload.refresh_token,
        &state.config.jwt.refresh_secret,
        TokenType::Refresh,
    )?;

    // Extract user_id
    let user_id = extract_user_id(&claims)?;

    // Check if refresh token exists in database
    let token_hash = crate::utils::api_key::hash_api_key(&payload.refresh_token);
    let refresh_token = RefreshTokenRepository::find_by_hash(&state.db, &token_hash)
        .await?
        .ok_or(AppError::TokenInvalid)?;

    // Verify refresh token is valid
    if !refresh_token.is_valid() {
        return Err(AppError::TokenExpired);
    }

    // Fetch user
    let user = UserRepository::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized("User not found".to_string()))?;

    // Check if user is active
    if !user.is_active() {
        return Err(AppError::Forbidden);
    }

    // Generate new tokens
    let new_access_token = generate_access_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.access_secret,
        state.config.jwt.access_expiry_seconds,
    )?;

    let new_refresh_token_str = generate_refresh_token(
        user.id,
        user.email.as_ref().unwrap(),
        &user.role,
        &state.config.jwt.refresh_secret,
        state.config.jwt.refresh_expiry_seconds,
    )?;

    // Revoke old refresh token (token rotation)
    RefreshTokenRepository::revoke(&state.db, &token_hash).await?;

    // Store new refresh token
    let new_token_hash = crate::utils::api_key::hash_api_key(&new_refresh_token_str);
    RefreshTokenRepository::create(
        &state.db,
        user.id,
        &new_token_hash,
        state.config.jwt.refresh_expiry_seconds,
        refresh_token.device_info,
        refresh_token.ip_address,
        refresh_token.user_agent,
    )
    .await?;

    Ok(Json(RefreshTokenResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token_str,
    }))
}

// ============================================================================
// POST /auth/logout - Revoke refresh token
// ============================================================================
pub async fn logout(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, AppError> {
    // Validate input
    payload.validate()?;

    // Hash the refresh token
    let token_hash = crate::utils::api_key::hash_api_key(&payload.refresh_token);

    // Revoke the refresh token
    RefreshTokenRepository::revoke(&state.db, &token_hash).await?;

    Ok(Json(LogoutResponse {
        message: "Logged out successfully".to_string(),
    }))
}

// ============================================================================
// GET /auth/me - Get current user info (requires JWT auth)
// ============================================================================
pub async fn me(AuthUser(user): AuthUser) -> Result<Json<MeResponse>, AppError> {
    let safe_user = user.to_safe_user();
    let user_response = UserResponse {
        id: safe_user.id,
        email: safe_user.email,
        username: safe_user.username,
        full_name: safe_user.full_name,
        role: format!("{:?}", safe_user.role).to_lowercase(),
        status: format!("{:?}", safe_user.status).to_lowercase(),
        created_at: safe_user.created_at,
    };

    Ok(Json(MeResponse {
        user: user_response,
    }))
}

// ============================================================================
// POST /auth/api-keys - Create new API key (requires JWT auth)
// ============================================================================
pub async fn create_api_key(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<CreateApiKeyResponse>), AppError> {
    // Validate input
    payload.validate()?;

    // Generate API key
    let generated = generate_api_key(payload.is_live)?;

    // Store API key in database
    let api_key = ApiKeyRepository::create(
        &state.db,
        user.id,
        &payload.name,
        &generated.key_prefix,
        &generated.key_hash,
        payload.expires_in_days,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(CreateApiKeyResponse {
            id: api_key.id,
            name: api_key.name,
            key: generated.key, // Full key shown once
            key_prefix: api_key.key_prefix,
            created_at: api_key.created_at,
            expires_at: api_key.expires_at,
        }),
    ))
}

// ============================================================================
// GET /auth/api-keys - List user's API keys (requires JWT auth)
// ============================================================================
pub async fn list_api_keys(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
) -> Result<Json<ListApiKeysResponse>, AppError> {
    let api_keys = ApiKeyRepository::list_for_user(&state.db, user.id).await?;

    let api_key_items: Vec<ApiKeyListItem> = api_keys
        .into_iter()
        .map(|key| ApiKeyListItem {
            id: key.id,
            name: key.name,
            key_prefix: key.key_prefix,
            last_used_at: key.last_used_at,
            created_at: key.created_at,
            expires_at: key.expires_at,
        })
        .collect();

    Ok(Json(ListApiKeysResponse {
        api_keys: api_key_items,
    }))
}

// ============================================================================
// POST /auth/api-keys/:key_id/revoke - Revoke API key (requires JWT auth)
// ============================================================================
pub async fn revoke_api_key(
    State(state): State<AppState>,
    AuthUser(user): AuthUser,
    Path(key_id): Path<Uuid>,
) -> Result<Json<RevokeApiKeyResponse>, AppError> {
    // Revoke the API key (verifies ownership)
    ApiKeyRepository::revoke(&state.db, key_id, user.id).await?;

    Ok(Json(RevokeApiKeyResponse {
        message: "API key revoked successfully".to_string(),
    }))
}

// ============================================================================
// Auth Router
// ============================================================================
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route("/api-keys", post(create_api_key))
        .route("/api-keys", get(list_api_keys))
        .route("/api-keys/:key_id/revoke", post(revoke_api_key))
}
