use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    db::DBClient,
    dtos::transaction_dtos::NewTransactionDto,
    models::transaction_model::{TransactionMode, TransactionStatus, TransactionType},
    services::account_service::AccountExt,
    utils::get_id::get_id,
};

pub trait TransactionExt {
    async fn create_transaction(
        &self,
        title: &str,
        description: Option<&str>,
        category_id: Option<Uuid>,
        transaction_type: &TransactionType,
        transaction_mode: &TransactionMode,
        is_recurring: bool,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        amount: f64,
        transaction_date: NaiveDate,
        transaction_status: TransactionStatus,
    ) -> Result<Option<NewTransactionDto>, sqlx::Error>;
}

impl TransactionExt for DBClient {
    async fn create_transaction(
        &self,
        title: &str,
        description: Option<&str>,
        category_id: Option<Uuid>,
        transaction_type: &TransactionType,
        transaction_mode: &TransactionMode,
        is_recurring: bool,
        from_account_id: Option<Uuid>,
        to_account_id: Option<Uuid>,
        amount: f64,
        transaction_date: NaiveDate,
        transaction_status: TransactionStatus,
    ) -> Result<Option<NewTransactionDto>, sqlx::Error> {
        // Fetching personal account before opening the transaction so it doesn't compete for the same pool connection and cause a timeout.
        let personal = self.get_my_personal_acc().await?;

        // for transaction aware queries
        let mut tx = self.pool.begin().await?;

        let transaction = sqlx::query_as::<_, NewTransactionDto>(
            r#"
            INSERT INTO transactions (
                id,
                title,
                description,
                category_id,
                transaction_type,
                transaction_mode,
                is_recurring,
                from_account_id,
                to_account_id,
                amount,
                transaction_date,
                transaction_status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING
                id,
                title,
                description,
                category_id,
                transaction_type,
                transaction_mode,
                is_recurring,
                from_account_id,
                to_account_id,
                amount,
                transaction_date,
                transaction_status,
                created_at,
                updated_at
            "#,
        )
        .bind(get_id())
        .bind(title)
        .bind(description)
        .bind(category_id)
        .bind(transaction_type)
        .bind(transaction_mode)
        .bind(is_recurring)
        .bind(from_account_id)
        .bind(to_account_id)
        .bind(amount)
        .bind(transaction_date)
        .bind(transaction_status)
        .fetch_one(&mut *tx)
        .await?;

        if let Some(personal_acc) = personal {
            let personal_id = personal_acc.id;

            Self::update_personal_balance(
                &mut tx,
                personal_id,
                from_account_id,
                to_account_id,
                amount,
            )
            .await?;

            Self::update_external_balance(
                &mut tx,
                transaction_status,
                from_account_id,
                to_account_id,
                personal_id,
                amount,
            )
            .await?;
        }

        tx.commit().await?;

        Ok(Some(transaction))
    }
}
