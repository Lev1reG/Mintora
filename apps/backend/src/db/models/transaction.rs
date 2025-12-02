use chrono::{NaiveDateTime, NaiveDate};
use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub r#type: TransactionType,
    pub amount: Decimal,
    pub currency: String,
    pub category_id: Option<Uuid>,
    pub payment_method_id: Option<Uuid>,
    pub merchant_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub transaction_date: NaiveDate,
    pub source: TransactionSource,
    pub source_message_id: Option<String>,
    pub attachment_urls: Option<sqlx::types::JsonValue>,
    pub metadata: Option<sqlx::types::JsonValue>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum TransactionSource {
    Whatsapp,
    Web,
}
