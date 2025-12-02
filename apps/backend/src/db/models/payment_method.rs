use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PaymentMethod {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub r#type: PaymentMethodType,
    pub last_4_digits: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum PaymentMethodType {
    Cash,
    Card,
    BankTransfer,
    DigitalWallet,
    Other,
}
