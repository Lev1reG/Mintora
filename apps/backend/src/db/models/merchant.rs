use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Merchant {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub default_category_id: Option<Uuid>,
    pub location: Option<String>,
    pub tags: Option<sqlx::types::JsonValue>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
