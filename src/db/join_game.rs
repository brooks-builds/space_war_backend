use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn join_game(game_id: Uuid, player_id: Uuid, pool: &Pool<Postgres>) -> Result<()> {
    let _result = sqlx::query_as!(
        DBJoinGame,
        "INSERT INTO game_players (game_id, player_id) VALUES ($1, $2)",
        game_id,
        player_id,
    )
    .execute(pool)
    .await
    .context("adding a player to a game");

    Ok(())
}
