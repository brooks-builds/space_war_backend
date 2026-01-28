use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_game_by_code(code: i32, pool: &Pool<Postgres>) -> Result<Option<DBGame>> {
    sqlx::query_as!(DBGame, r#"SELECT id FROM games WHERE code = $1"#, code)
        .fetch_optional(pool)
        .await
        .context("getting game by code")
}

pub struct DBGame {
    pub id: Uuid,
}
