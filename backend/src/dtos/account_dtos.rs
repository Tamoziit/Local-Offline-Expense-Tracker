use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::models::account_model::AccountType;

#[derive(Debug, Validate, Clone, Serialize, Deserialize)]
pub struct AccountCreationDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    pub account_type: AccountType,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct NewAccountDto {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountSummary {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
}
