use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::db::models::RefreshToken;
use crate::error::AppError;

pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    /// Create a new refresh token
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        token_hash: &str,
        expires_in_seconds: i64,
        device_info: Option<String>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<RefreshToken, AppError> {
        let expires_at = Utc::now().naive_utc() + Duration::seconds(expires_in_seconds);

        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_info, ip_address, user_agent, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, token_hash, device_info, ip_address, user_agent,
                      expires_at, revoked_at, created_at, last_used_at
            "#,
        )
        .bind(user_id)
        .bind(token_hash)
        .bind(device_info)
        .bind(ip_address)
        .bind(user_agent)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(token)
    }

    /// Find a valid refresh token by its hash
    pub async fn find_by_hash(pool: &PgPool, token_hash: &str) -> Result<Option<RefreshToken>, AppError> {
        let token = sqlx::query_as::<_, RefreshToken>(
            r#"
            SELECT id, user_id, token_hash, device_info, ip_address, user_agent,
                   expires_at, revoked_at, created_at, last_used_at
            FROM refresh_tokens
            WHERE token_hash = $1
              AND revoked_at IS NULL
              AND expires_at > now()
            "#,
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await?;

        Ok(token)
    }

    /// Revoke a specific refresh token
    pub async fn revoke(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = $1
            WHERE token_hash = $2
              AND revoked_at IS NULL
            "#,
        )
        .bind(now)
        .bind(token_hash)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Revoke all refresh tokens for a user (logout from all devices)
    pub async fn revoke_all_for_user(pool: &PgPool, user_id: Uuid) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = $1
            WHERE user_id = $2
              AND revoked_at IS NULL
            "#,
        )
        .bind(now)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Update last_used_at timestamp for a token
    pub async fn update_last_used(pool: &PgPool, token_hash: &str) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET last_used_at = $1
            WHERE token_hash = $2
            "#,
        )
        .bind(now)
        .bind(token_hash)
        .execute(pool)
        .await?;

        Ok(())
    }
}
