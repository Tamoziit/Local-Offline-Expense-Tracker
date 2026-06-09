use crate::{
    db::DBClient,
    dtos::account_dtos::{AccountSummary, ExternalAccount, NewAccountDto, PersonalAccount},
    models::account_model::AccountType,
    utils::get_id::get_id,
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

    async fn get_my_personal_acc(&self) -> Result<Option<PersonalAccount>, sqlx::Error>;

    async fn get_external_acc(&self) -> Result<Vec<ExternalAccount>, sqlx::Error>;
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
        .bind(get_id())
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
                SELECT id, name, account_type, balance
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

    async fn get_my_personal_acc(&self) -> Result<Option<PersonalAccount>, sqlx::Error> {
        let account = sqlx::query_as::<_, PersonalAccount>(
            r#"
            SELECT id, name, account_type, balance, created_at, updated_at
            FROM accounts
            WHERE account_type = $1
            LIMIT 1
            "#,
        )
        .bind(AccountType::Personal)
        .fetch_optional(&self.pool)
        .await?;

        Ok(account)
    }

    async fn get_external_acc(&self) -> Result<Vec<ExternalAccount>, sqlx::Error> {
        let accounts = sqlx::query_as::<_, ExternalAccount>(
            r#"
            SELECT id, name, account_type, to_receive, to_give, created_at, updated_at
            FROM accounts
            WHERE account_type = $1
            "#,
        )
        .bind(AccountType::External)
        .fetch_all(&self.pool)
        .await?;

        Ok(accounts)
    }
}
