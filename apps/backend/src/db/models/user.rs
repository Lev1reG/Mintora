use chrono::NaiveDateTime;
use uuid::Uuid;

/// User account model
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub username: String,
    pub full_name: String,
    pub role: UserRole,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
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
}
