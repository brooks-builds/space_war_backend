use eyre::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use sqlx::{
    Pool, Postgres, query, query_as,
    types::{Json, JsonRawValue},
};
use uuid::Uuid;

use crate::game::vector::Vector;

pub async fn create_turn(
    pool: &Pool<Postgres>,
    player_id: Uuid,
    game_turn_id: Uuid,
    speed_change: i32,
    destination_x: Option<i32>,
    destination_y: Option<i32>,
    torpedo_target_x: Option<i32>,
    torpedo_target_y: Option<i32>,
    ship_steps: Option<Vec<Vector>>,
) -> Result<()> {
    query!(
        r#"
            INSERT INTO player_turns
                (
                    player_id,
                    game_turn_id,
                    speed_change,
                    destination_x,
                    destination_y,
                    torpedo_target_x,
                    torpedo_target_y,
                    ship_travel_steps
                )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        player_id,
        game_turn_id,
        speed_change,
        destination_x,
        destination_y,
        torpedo_target_x,
        torpedo_target_y,
        ship_steps.map(|steps| Json(steps)) as _,
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
                speed_change,
                destination_x,
                destination_y,
                torpedo_target_x,
                torpedo_target_y,
                turn_number,
                player_id,
                ship_travel_steps AS "ship_travel_steps: Json<Vec<Vector>>"
            FROM player_turns
            JOIN game_turns on game_turns.id = player_turns.game_turn_id
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

pub async fn get_all_turns_for_game(
    pool: &Pool<Postgres>,
    game_id: &Uuid,
) -> Result<Vec<DBPlayerTurn>> {
    query_as!(
        DBPlayerTurn,
        r#"
            SELECT
                speed_change,
                destination_x,
                destination_y,
                torpedo_target_x,
                torpedo_target_y,
                turn_number,
                player_id,
                ship_travel_steps AS "ship_travel_steps: Json<Vec<Vector>>"
            FROM player_turns
            JOIN game_turns on game_turns.id = player_turns.game_turn_id
            WHERE game_turns.game_id = $1
        "#,
        game_id
    )
    .fetch_all(pool)
    .await
    .context("Fetching all player turns by game id")
}

#[derive(Debug, Deserialize)]
pub struct DBPlayerTurn {
    pub speed_change: i32,
    pub destination_x: Option<i32>,
    pub destination_y: Option<i32>,
    pub torpedo_target_x: Option<i32>,
    pub torpedo_target_y: Option<i32>,
    pub turn_number: i32,
    pub player_id: Uuid,
    pub ship_travel_steps: Option<Json<Vec<Vector>>>,
}
