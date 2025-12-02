use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

/// User account model
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: String,
    pub full_name: String,
    pub password_hash: Option<String>,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Suspended,
    Deleted,
}

impl User {
    pub fn is_active(&self) -> bool {
        self.deleted_at.is_none() && self.status == UserStatus::Active
    }

    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    pub fn has_dashboard_access(&self) -> bool {
        self.email.is_some() && self.password_hash.is_some()
    }

    /// Safe user representation without password hash
    pub fn to_safe_user(&self) -> SafeUser {
        SafeUser {
            id: self.id,
            email: self.email.clone(),
            username: self.username.clone(),
            full_name: self.full_name.clone(),
            role: self.role.clone(),
            status: self.status.clone(),
            created_at: self.created_at,
        }
    }
}

/// Safe user representation without sensitive data
#[derive(Debug, Clone, Serialize)]
pub struct SafeUser {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: String,
    pub full_name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
}
