use crate::{
    db::DBClient, dtos::account_dtos::{AccountSummary, NewAccountDto}, models::account_model::{Account, AccountType}
};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub trait AccountExt {
    async fn create_account(
        &self,
        name: &String,
        account_type: &AccountType,
    ) -> Result<Option<NewAccountDto>, sqlx::Error>;

    async fn get_account_by_type(
        &self,
        account_type: &AccountType,
    ) -> Result<Option<AccountSummary>, sqlx::Error>;
}

impl AccountExt for DBClient {
    async fn create_account(
        &self,
        name: &String,
        account_type: &AccountType,
    ) -> Result<Option<NewAccountDto>, sqlx::Error> {
        let new_account = sqlx::query_as::<_, NewAccountDto>(
            r#"
                INSERT INTO accounts (id, name, account_type)
                VALUES ($1, $2, $3)
                RETURNING
                    id,
                    name,
                    account_type,
                    created_at,
                    updated_at
                "#,
        )
        .bind(uuid::Uuid::new_v4())
        .bind(name)
        .bind(account_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(new_account))
    }

    async fn get_account_by_type(
        &self,
        account_type: &AccountType,
    ) -> Result<Option<AccountSummary>, sqlx::Error> {
        let account = sqlx::query_as::<_, AccountSummary>(
            r#"
                SELECT id, name, account_type, balance, created_at, updated_at
                FROM accounts
                WHERE account_type = $1
                LIMIT 1
                "#,
        )
        .bind(account_type)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }
}
