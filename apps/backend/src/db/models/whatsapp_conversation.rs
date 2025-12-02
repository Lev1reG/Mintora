use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WhatsAppConversation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub whatsapp_client_id: Uuid,
    pub message_id: Option<String>,
    pub direction: MessageDirection,
    pub message_text: String,
    pub intent: Option<String>,
    pub extracted_data: Option<sqlx::types::JsonValue>,
    pub confidence_score: Option<Decimal>,
    pub transaction_id: Option<Uuid>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum MessageDirection {
    Inbound,
    Outbound,
}
