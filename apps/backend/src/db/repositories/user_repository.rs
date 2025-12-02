use sqlx::PgPool;
use uuid::Uuid;
use crate::db::models::{User, UserRole, UserStatus};
use crate::error::AppError;

pub struct UserRepository;

impl UserRepository {
    /// Find user by email
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, username, full_name, password_hash,
                   role::text as role, status::text as status,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE email = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(pool: &PgPool, user_id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, username, full_name, password_hash,
                   role::text as role, status::text as status,
                   created_at, updated_at, deleted_at
            FROM users
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Create new user with email/password
    pub async fn create_with_password(
        pool: &PgPool,
        email: &str,
        username: &str,
        full_name: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, username, full_name, password_hash)
            VALUES ($1, $2, $3, $4)
            RETURNING id, email, username, full_name, password_hash,
                      role::text as role, status::text as status,
                      created_at, updated_at, deleted_at
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(full_name)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Check if email exists
    pub async fn email_exists(pool: &PgPool, email: &str) -> Result<bool, AppError> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND deleted_at IS NULL)",
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }

    /// Check if username exists
    pub async fn username_exists(pool: &PgPool, username: &str) -> Result<bool, AppError> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 AND deleted_at IS NULL)",
        )
        .bind(username)
        .fetch_one(pool)
        .await?;

        Ok(exists)
    }
}
