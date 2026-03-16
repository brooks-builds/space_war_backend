use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_all_ships(pool: &Pool<Postgres>) -> Result<Vec<DBShip>> {
    sqlx::query_as!(
        DBShip,
        r#"
            SELECT
                id,
                name,
                character,
                max_speed,
                max_torpedo_count,
                max_hitpoints
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
    pub max_speed: i32,
    pub max_torpedo_count: i32,
    pub max_hitpoints: i32,
}
