use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionType {
    Expense,
    Income,
    Transfer,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionMode {
    Online,
    Cash,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Paid,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Transaction {
    pub id: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<uuid::Uuid>,
    pub transaction_type: TransactionType,
    pub transaction_mode: TransactionMode,
    pub is_recurring: bool,
    pub from_account_id: Option<uuid::Uuid>,
    pub to_account_id: Option<uuid::Uuid>,
    pub amount: f64,
    pub transaction_date: NaiveDate,
    pub transaction_status: TransactionStatus,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
