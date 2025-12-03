use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    pub user_id: Uuid,
    pub phone_number: String,
    pub country_code: String,
    pub channel: Channel,
    pub is_verified: bool,
    pub is_active: bool,
    pub last_interaction_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Whatsapp,
    Telegram,
    Line,
    Discord,
    Slack,
}

impl Client {
    pub fn can_interact(&self) -> bool {
        self.is_active && self.is_verified
    }

    pub fn is_whatsapp(&self) -> bool {
        self.channel == Channel::Whatsapp
    }

    pub fn is_telegram(&self) -> bool {
        self.channel == Channel::Telegram
    }
}
