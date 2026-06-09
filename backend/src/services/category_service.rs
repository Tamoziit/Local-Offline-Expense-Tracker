use crate::{db::DBClient, models::category_model::Category, utils::get_id::get_id};

pub trait CategoryExt {
    async fn create_category(&self, name: &String) -> Result<Option<Category>, sqlx::Error>;

    async fn get_all_categories(&self) -> Result<Vec<Category>, sqlx::Error>;
}

impl CategoryExt for DBClient {
    async fn create_category(&self, name: &String) -> Result<Option<Category>, sqlx::Error> {
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
        .await?;

        Ok(Some(new_category))
    }

    async fn get_all_categories(&self) -> Result<Vec<Category>, sqlx::Error> {
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
}
