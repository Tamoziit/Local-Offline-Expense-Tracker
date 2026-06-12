use crate::{
    db::DBClient,
    dtos::account_dtos::{AccountSummary, ExternalAccount, NewAccountDto, PersonalAccount},
    errors::{ErrorMessage, ServiceError},
    models::{account_model::AccountType, transaction_model::TransactionStatus},
    utils::get_id::get_id,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub trait AccountExt {
    async fn create_account(
        &self,
        name: &String,
        account_type: &AccountType,
    ) -> Result<NewAccountDto, ServiceError>;

    async fn get_account_by_type(
        &self,
        account_type: &AccountType,
    ) -> Result<Option<AccountSummary>, ServiceError>;

    async fn get_my_personal_acc(&self) -> Result<Option<PersonalAccount>, ServiceError>;

    async fn get_external_acc(&self) -> Result<Vec<ExternalAccount>, ServiceError>;

    async fn account_exists_by_id(&self, id: Uuid) -> Result<bool, ServiceError>;

    async fn update_personal_balance(
        tx: &mut Transaction<'_, Sqlite>,
        transaction_status: TransactionStatus,
        personal_id: Uuid,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        amount: f64,
    ) -> Result<(), ServiceError>;

    async fn update_external_balance(
        tx: &mut Transaction<'_, Sqlite>,
        transaction_status: TransactionStatus,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        personal_id: Uuid,
        amount: f64,
    ) -> Result<(), ServiceError>;
}

impl AccountExt for DBClient {
    async fn create_account(
        &self,
        name: &String,
        account_type: &AccountType,
    ) -> Result<NewAccountDto, ServiceError> {
        if *account_type == AccountType::Personal {
            let existing = self.get_account_by_type(account_type).await?;
            if existing.is_some() {
                return Err(ServiceError::Conflict(
                    ErrorMessage::PersonalAccountAlreadyExists.to_string(),
                ));
            }
        }

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
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                ServiceError::Conflict(ErrorMessage::AccountAlreadyExists.to_string())
            }
            other => ServiceError::Db(other),
        })?;

        Ok(new_account)
    }

    async fn get_account_by_type(
        &self,
        account_type: &AccountType,
    ) -> Result<Option<AccountSummary>, ServiceError> {
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

    async fn get_my_personal_acc(&self) -> Result<Option<PersonalAccount>, ServiceError> {
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

    async fn get_external_acc(&self) -> Result<Vec<ExternalAccount>, ServiceError> {
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

    async fn account_exists_by_id(&self, id: Uuid) -> Result<bool, ServiceError> {
        let exists: bool =
            sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM accounts WHERE id = $1)"#)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(exists)
    }

    async fn update_personal_balance(
        tx: &mut Transaction<'_, Sqlite>,
        transaction_status: TransactionStatus,
        personal_id: Uuid,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        amount: f64,
    ) -> Result<(), ServiceError> {
        if transaction_status == TransactionStatus::Pending {
            return Ok(());
        }

        if from_account_id == Some(personal_id) {
            sqlx::query(
                r#"
                UPDATE accounts
                SET
                    balance = balance - ?,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE id = ?
                "#,
            )
            .bind(amount)
            .bind(personal_id)
            .execute(&mut **tx)
            .await?;
        } else if to_account_id == Some(personal_id) {
            sqlx::query(
                r#"
                UPDATE accounts
                SET
                    balance = balance + ?,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE id = ?
                "#,
            )
            .bind(amount)
            .bind(personal_id)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    async fn update_external_balance(
        tx: &mut Transaction<'_, Sqlite>,
        transaction_status: TransactionStatus,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        personal_id: Uuid,
        amount: f64,
    ) -> Result<(), ServiceError> {
        if transaction_status != TransactionStatus::Pending {
            return Ok(());
        }

        if let Some(ext_id) = from_account_id.filter(|&id| id != personal_id) {
            sqlx::query(
                r#"
                UPDATE accounts
                SET
                    to_receive = to_receive + ?,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE
                    id = ?
                    AND account_type = 'external'
                "#,
            )
            .bind(amount)
            .bind(ext_id)
            .execute(&mut **tx)
            .await?;
        } else if let Some(ext_id) = to_account_id.filter(|&id| id != personal_id) {
            sqlx::query(
                r#"
                UPDATE accounts
                SET
                    to_give = to_give + ?,
                    updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                WHERE
                    id = ?
                    AND account_type = 'external'
                "#,
            )
            .bind(amount)
            .bind(ext_id)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }
}
