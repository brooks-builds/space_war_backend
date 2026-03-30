use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::create_game::DBCreatedGameStatus;

pub async fn get_game_by_code(code: i32, pool: &Pool<Postgres>) -> Result<Option<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"SELECT id, status AS "status: _" FROM games WHERE code = $1"#,
        code
    )
    .fetch_optional(pool)
    .await
    .context("getting game by code")
}

pub struct DBGame {
    pub id: Uuid,
    pub status: DBCreatedGameStatus,
}
