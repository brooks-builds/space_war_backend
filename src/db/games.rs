use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, postgres::types::PgTimeTz};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::create_game::DBCreatedGameStatus;

pub async fn get_game_created_by_player(
    pool: &Pool<Postgres>,
    player_id: Uuid,
) -> Result<Option<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"SELECT id, status AS "status: _", created_at, code, host_id FROM games WHERE host_id = $1"#,
        player_id
    )
    .fetch_optional(pool)
    .await
    .context("Getting game owned by player")
}

pub async fn change_game_host(
    pool: &Pool<Postgres>,
    new_host_id: Uuid,
    game_id: Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE games
            SET host_id = $1
            WHERE id = $2
        "#,
        new_host_id,
        game_id
    )
    .execute(pool)
    .await
    .context("Changing game host")?;

    Ok(())
}

pub async fn delete_game(pool: &Pool<Postgres>, game_id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM games
            WHERE id = $1
        "#,
        game_id
    )
    .execute(pool)
    .await
    .context("Deleting game by id")?;

    Ok(())
}

pub async fn get_game_by_id(pool: &Pool<Postgres>, game_id: Uuid) -> Result<Option<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"SELECT id, status AS "status: _", created_at, code, host_id FROM games WHERE id = $1 "#,
        game_id
    )
    .fetch_optional(pool)
    .await
    .context("getting game by id")
}

#[derive(Debug, Deserialize)]
pub struct DBGame {
    pub id: Uuid,
    pub status: DBCreatedGameStatus,
    pub created_at: OffsetDateTime,
    pub code: i32,
    pub host_id: Uuid,
    pub width: i32,
    pub height: i32,
}

pub async fn get_all_games(pool: &Pool<Postgres>) -> Result<Vec<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"
        SELECT id, status AS "status: _", created_at, code, host_id FROM games
    "#
    )
    .fetch_all(pool)
    .await
    .context("Getting all games")
}

pub async fn set_game_status(
    pool: &Pool<Postgres>,
    game_id: Uuid,
    status: DBCreatedGameStatus,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE games SET status = $1 WHERE id = $2
        "#,
        status as _,
        game_id
    )
    .execute(pool)
    .await
    .context("Setting game status")?;

    Ok(())
}
