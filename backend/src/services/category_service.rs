use uuid::Uuid;

use crate::{
    db::DBClient,
    errors::{ErrorMessage, ServiceError},
    models::category_model::Category,
    utils::get_id::get_id,
};

pub trait CategoryExt {
    async fn create_category(&self, name: &String) -> Result<Category, ServiceError>;

    async fn get_all_categories(&self) -> Result<Vec<Category>, ServiceError>;

    async fn category_exists_by_id(&self, id: Uuid) -> Result<bool, ServiceError>;
}

impl CategoryExt for DBClient {
    async fn create_category(&self, name: &String) -> Result<Category, ServiceError> {
        let new_category = sqlx::query_as::<_, Category>(
            r#"
                INSERT INTO categories (id, name)
                VALUES ($1, $2)
                RETURNING
                    id,
                    name,
                    created_at,
                    updated_at
                "#,
        )
        .bind(get_id())
        .bind(name)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(ref db_err) if db_err.is_unique_violation() => {
                ServiceError::Conflict(ErrorMessage::CategoryAlreadyExists.to_string())
            }
            other => ServiceError::Db(other),
        })?;

        Ok(new_category)
    }

    async fn get_all_categories(&self) -> Result<Vec<Category>, ServiceError> {
        let categories = sqlx::query_as::<_, Category>(
            r#"
                SELECT *
                FROM categories
                "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(categories)
    }

    async fn category_exists_by_id(&self, id: Uuid) -> Result<bool, ServiceError> {
        let exists: bool =
            sqlx::query_scalar(r#"SELECT EXISTS(SELECT 1 FROM categories WHERE id = $1)"#)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        Ok(exists)
    }
}
