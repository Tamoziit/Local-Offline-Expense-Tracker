use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::models::transaction_model::{TransactionMode, TransactionStatus, TransactionType};

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct CreateTransactionDto {
    #[validate(length(min = 1, message = "Title is required"))]
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub transaction_mode: TransactionMode,
    pub is_recurring: bool,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    #[validate(range(min = 0.0, message = "Amount must be positive"))]
    pub amount: f64,
    pub transaction_date: NaiveDate,
    pub transaction_status: TransactionStatus,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct NewTransactionDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub transaction_mode: TransactionMode,
    pub is_recurring: bool,
    pub from_account_id: Option<Uuid>,
    pub to_account_id: Option<Uuid>,
    pub amount: f64,
    pub transaction_date: NaiveDate,
    pub transaction_status: TransactionStatus,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct TransactionDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub transaction_type: TransactionType,
    pub transaction_mode: TransactionMode,
    pub is_recurring: bool,
    pub from_account: Option<String>,
    pub to_account: Option<String>,
    pub amount: f64,
    pub transaction_date: NaiveDate,
    pub transaction_status: TransactionStatus,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
