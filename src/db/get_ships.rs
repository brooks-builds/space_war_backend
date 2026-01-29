use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_all_ships(pool: &Pool<Postgres>) -> Result<Vec<DBShip>> {
    sqlx::query_as!(
        DBShip,
        r#"
            SELECT id, name, character
            FROM ships;
        "#
    )
    .fetch_all(pool)
    .await
    .context("Getting all ships from the database")
}

pub struct DBShip {
    pub id: Uuid,
    pub name: String,
    pub character: String,
}
