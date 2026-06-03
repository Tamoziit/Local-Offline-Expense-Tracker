use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, Eq, PartialEq)]
#[sqlx(rename_all = "lowercase")]
pub enum AccountType {
    Personal,
    External,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct Account {
    pub id: uuid::Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub balance: f64,
    pub to_receive: f64,
    pub to_give: f64,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
