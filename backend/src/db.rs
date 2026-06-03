use sqlx::{Pool, Sqlite};

#[derive(Debug, Clone)]
pub struct DBClient {
    pub pool: Pool<Sqlite>,
}

impl DBClient {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}