use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::db::models::ApiKey;
use crate::error::AppError;

pub struct ApiKeyRepository;

impl ApiKeyRepository {
    /// Create a new API key
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        name: &str,
        key_prefix: &str,
        key_hash: &str,
        expires_in_days: Option<i64>,
    ) -> Result<ApiKey, AppError> {
        let expires_at = expires_in_days.map(|days| {
            Utc::now().naive_utc() + Duration::days(days)
        });

        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            INSERT INTO api_keys (user_id, name, key_prefix, key_hash, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_id, name, key_prefix, key_hash, scopes,
                      last_used_at, expires_at, revoked_at, created_at, updated_at
            "#,
        )
        .bind(user_id)
        .bind(name)
        .bind(key_prefix)
        .bind(key_hash)
        .bind(expires_at)
        .fetch_one(pool)
        .await?;

        Ok(api_key)
    }

    /// Find an API key by its hash (only active keys)
    pub async fn find_by_hash(pool: &PgPool, key_hash: &str) -> Result<Option<ApiKey>, AppError> {
        let api_key = sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT id, user_id, name, key_prefix, key_hash, scopes,
                   last_used_at, expires_at, revoked_at, created_at, updated_at
            FROM api_keys
            WHERE key_hash = $1
              AND revoked_at IS NULL
              AND (expires_at IS NULL OR expires_at > now())
            "#,
        )
        .bind(key_hash)
        .fetch_optional(pool)
        .await?;

        Ok(api_key)
    }

    /// List all API keys for a user (only active keys)
    pub async fn list_for_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<ApiKey>, AppError> {
        let api_keys = sqlx::query_as::<_, ApiKey>(
            r#"
            SELECT id, user_id, name, key_prefix, key_hash, scopes,
                   last_used_at, expires_at, revoked_at, created_at, updated_at
            FROM api_keys
            WHERE user_id = $1
              AND revoked_at IS NULL
              AND (expires_at IS NULL OR expires_at > now())
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(api_keys)
    }

    /// Revoke an API key (user must own it)
    pub async fn revoke(pool: &PgPool, key_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        let result = sqlx::query(
            r#"
            UPDATE api_keys
            SET revoked_at = $1, updated_at = $1
            WHERE id = $2
              AND user_id = $3
              AND revoked_at IS NULL
            "#,
        )
        .bind(now)
        .bind(key_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    /// Update last_used_at timestamp for an API key
    pub async fn update_last_used(pool: &PgPool, key_hash: &str) -> Result<(), AppError> {
        let now = Utc::now().naive_utc();

        sqlx::query(
            r#"
            UPDATE api_keys
            SET last_used_at = $1
            WHERE key_hash = $2
            "#,
        )
        .bind(now)
        .bind(key_hash)
        .execute(pool)
        .await?;

        Ok(())
    }
}
