use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub r#type: CategoryType,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_category_id: Option<Uuid>,
    pub is_system: bool,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum CategoryType {
    Income,
    Expense,
    Both,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CategoryAlias {
    pub id: Uuid,
    pub category_id: Uuid,
    pub alias: String,
    pub created_at: NaiveDateTime,
}
