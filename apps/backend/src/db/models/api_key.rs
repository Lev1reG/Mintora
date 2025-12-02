use chrono::{NaiveDateTime, Utc};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub key_hash: String,
    pub scopes: JsonValue,
    pub last_used_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub revoked_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl ApiKey {
    pub fn is_valid(&self) -> bool {
        let now = Utc::now().naive_utc();

        self.revoked_at.is_none()
            && self.expires_at.map(|exp| exp > now).unwrap_or(true)
    }

    pub fn has_scope(&self, scope: &str) -> bool {
        if let Some(scopes_array) = self.scopes.as_array() {
            scopes_array.iter().any(|s| s.as_str() == Some(scope))
        } else {
            false
        }
    }
}
