use eyre::{Context, Result};
use serde::Deserialize;
use sqlx::{Pool, Postgres, query, query_as};
use uuid::Uuid;

pub async fn create_turn(
    pool: &Pool<Postgres>,
    player_id: Uuid,
    game_turn_id: Uuid,
    speed_change: i32,
) -> Result<()> {
    query!(
        r#"
            INSERT INTO player_turns (player_id, game_turn_id, speed_change)
            VALUES ($1, $2, $3)
        "#,
        player_id,
        game_turn_id,
        speed_change
    )
    .execute(pool)
    .await
    .context("Creating player turn")?;

    Ok(())
}

pub async fn get_players_turn(
    pool: &Pool<Postgres>,
    player_id: Uuid,
    game_turn_id: Uuid,
) -> Result<Option<DBPlayerTurn>> {
    query_as!(
        DBPlayerTurn,
        r#"
            SELECT
                id,
                player_id,
                game_turn_id,
                speed_change
            FROM player_turns
            WHERE player_id = $1
            AND game_turn_id = $2
        "#,
        player_id,
        game_turn_id
    )
    .fetch_optional(pool)
    .await
    .context("Getting turn for a player")
}

#[derive(Debug, Deserialize)]
pub struct DBPlayerTurn {
    pub id: Uuid,
    pub player_id: Uuid,
    pub game_turn_id: Uuid,
    pub speed_change: i32,
}
