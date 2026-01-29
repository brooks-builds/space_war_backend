use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_colors(pool: &Pool<Postgres>) -> Result<Vec<DBColor>> {
    sqlx::query_as!(
        DBColor,
        r#"
            SELECT * FROM colors;
        "#
    )
    .fetch_all(pool)
    .await
    .context("Getting colors from database")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBColor {
    pub id: Uuid,
    pub name: String,
}
