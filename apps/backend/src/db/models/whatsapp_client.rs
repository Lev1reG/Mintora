use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WhatsAppClient {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phone_number: String,
    pub country_code: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub last_interaction_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl WhatsAppClient {
    pub fn can_interact(&self) -> bool {
        self.is_active && self.is_verified
    }
}
