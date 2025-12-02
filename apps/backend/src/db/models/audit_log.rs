use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub old_values: Option<sqlx::types::JsonValue>,
    pub new_values: Option<sqlx::types::JsonValue>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
}
