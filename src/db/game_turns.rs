use eyre::{Context, Result};
use serde::Deserialize;
use sqlx::{Pool, Postgres, query, query_as};
use uuid::Uuid;

pub async fn create_turn(pool: &Pool<Postgres>, turn_number: i32, game_id: Uuid) -> Result<()> {
    query!(
        r#"
            INSERT INTO game_turns (game_id, turn_number)
            VALUES ($1, $2);
        "#,
        game_id,
        turn_number
    )
    .execute(pool)
    .await
    .context("Creating a turn for a game")?;

    Ok(())
}

pub async fn get_latest_player_turn(
    pool: &Pool<Postgres>,
    player_id: Uuid,
) -> Result<Option<DBGameTurn>> {
    query_as!(
        DBGameTurn,
        r#"
            SELECT
                game_turns.id,
                turn_number,
                active
            FROM game_turns
            JOIN game_players ON game_players.game_id = game_turns.game_id
            JOIN players ON players.id = game_players.player_id
            WHERE players.id = $1
            ORDER BY turn_number desc
            LIMIT 1
        "#,
        player_id
    )
    .fetch_optional(pool)
    .await
    .context("Getting game turn for a player")
}
#[derive(Debug, Deserialize)]
pub struct DBGameTurn {
    pub id: Uuid,
    pub turn_number: i32,
    pub active: bool,
}

pub async fn mark_turn_not_active(pool: &Pool<Postgres>, id: Uuid) -> Result<()> {
    query!(
        r#"
            UPDATE game_turns
            SET active = false
            WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await
    .context("Marking game turn not active")?;
    Ok(())
}
