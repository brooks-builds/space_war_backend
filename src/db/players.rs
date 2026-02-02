use eyre::{Context, Result};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn delete_player(pool: &Pool<Postgres>, token: Uuid) -> Result<()> {
    sqlx::query!("DELETE FROM players WHERE token = $1", token)
        .execute(pool)
        .await
        .context("deleting player")?;

    Ok(())
}

#[derive(Debug, Serialize)]
pub struct DBPlayer {
    pub id: Uuid,
}

pub async fn get_player_by_token(pool: &Pool<Postgres>, token: Uuid) -> Result<Option<DBPlayer>> {
    sqlx::query_as!(
        DBPlayer,
        r#"
        SELECT id
        FROM players
        WHERE token = $1
    "#,
        token
    )
    .fetch_optional(pool)
    .await
    .context("Getting player by token")
}

pub async fn get_players_in_game(pool: &Pool<Postgres>, game_id: Uuid) -> Result<Vec<DBPlayer>> {
    sqlx::query_as!(
        DBPlayer,
        r#"
        SELECT players.id
        FROM game_players
        JOIN players on players.id = game_players.player_id
        WHERE game_players.game_id = $1
    "#,
        game_id
    )
    .fetch_all(pool)
    .await
    .context("Getting all players in a game")
}
