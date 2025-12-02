use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub color: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TransactionTag {
    pub transaction_id: Uuid,
    pub tag_id: Uuid,
    pub created_at: NaiveDateTime,
}
