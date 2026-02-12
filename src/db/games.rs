use eyre::{Context, Result};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::db::create_game::DBCreatedGameStatus;

pub async fn get_game_created_by_player(
    pool: &Pool<Postgres>,
    player_id: Uuid,
) -> Result<Option<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"
        SELECT
            games.id,
            status AS "status: _",
            created_at,
            host_id,
            width,
            height,
            COALESCE(MAX(turn_number), 0) AS turn_number
        FROM games
        LEFT JOIN game_turns ON game_turns.game_id = games.id
        WHERE host_id = $1
        GROUP BY games.id
        "#,
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
        r#"
        SELECT
            games.id,
            status AS "status: _",
            created_at,
            host_id,
            width,
            height,
            COALESCE(MAX(turn_number), 0) AS turn_number
        FROM games
        LEFT JOIN game_turns on game_turns.game_id = games.id
        WHERE games.id = $1
        GROUP BY games.id
        "#,
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
    pub host_id: Uuid,
    pub width: i32,
    pub height: i32,
    pub turn_number: Option<i32>,
}

pub async fn get_all_games(pool: &Pool<Postgres>) -> Result<Vec<DBGame>> {
    sqlx::query_as!(
        DBGame,
        r#"
        SELECT
            games.id,
            status AS "status: _",
            created_at,
            host_id,
            width,
            height,
            COALESCE(MAX(turn_number), 0) AS turn_number
        FROM games
        LEFT JOIN game_turns ON game_turns.game_id = games.id
        GROUP BY games.id
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
