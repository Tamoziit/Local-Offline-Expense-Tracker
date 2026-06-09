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

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct PersonalAccount {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Clone, Serialize, Deserialize, FromRow)]
pub struct ExternalAccount {
    pub id: Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub to_receive: f64,
    pub to_give: f64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
