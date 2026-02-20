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

#[derive(Debug, Serialize, Clone)]
pub struct DBPlayer {
    pub id: Uuid,
    pub name: String,
    pub ship_char: String,
    pub ship_max_speed: i32,
    pub color: String,
    pub ready: bool,
    pub position_x: Option<i32>,
    pub position_y: Option<i32>,
    pub speed: i32,
    pub token: Uuid,
    pub ship_classname: String,
    pub torpedo_count: i32,
}

pub async fn get_player_by_token(pool: &Pool<Postgres>, token: Uuid) -> Result<Option<DBPlayer>> {
    sqlx::query_as!(
        DBPlayer,
        r#"
        SELECT
            players.id,
            players.name,
            ships.character AS ship_char,
            ships.max_speed AS ship_max_speed,
            colors.name AS color,
            players.ready,
            players.position_x,
            players.position_y,
            players.speed,
            players.token,
            ships.name AS ship_classname,
            torpedo_count
        FROM players
        JOIN ships on ships.id = players.ship_id
        JOIN colors on colors.id = players.color_id
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
        SELECT
            players.id,
            players.name,
            ships.character AS ship_char,
            ships.max_speed AS ship_max_speed,
            colors.name AS color,
            players.ready,
            players.position_x,
            players.position_y,
            players.speed,
            players.token,
            ships.name AS ship_classname,
            players.torpedo_count
        FROM game_players
        JOIN players on players.id = game_players.player_id
        JOIN ships ON ships.id = players.ship_id
        JOIN colors ON colors.id = players.color_id
        WHERE game_players.game_id = $1
    "#,
        game_id
    )
    .fetch_all(pool)
    .await
    .context("Getting all players in a game")
}

pub async fn ready_up(pool: &Pool<Postgres>, token: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET ready = true
            WHERE token = $1
        "#,
        token
    )
    .execute(pool)
    .await
    .context("Readying up a player")?;

    Ok(())
}

pub async fn unready_all_players_in_game(pool: &Pool<Postgres>, game_id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET ready = false
            FROM game_players
            WHERE game_players.game_id = $1
            AND players.id = game_players.player_id
        "#,
        game_id
    )
    .execute(pool)
    .await
    .context("Marking all players in a game as not ready")?;

    Ok(())
}

pub async fn set_player_position(
    pool: &Pool<Postgres>,
    x: i32,
    y: i32,
    player_id: &Uuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET position_x = $1, position_y = $2
            WHERE id = $3
        "#,
        x,
        y,
        player_id
    )
    .execute(pool)
    .await
    .context("Setting player location")?;
    Ok(())
}

pub async fn set_speed(pool: &Pool<Postgres>, token: Uuid, speed: i32) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET speed = $2
            WHERE token = $1
        "#,
        token,
        speed
    )
    .execute(pool)
    .await
    .context("Increasing player speed")?;

    Ok(())
}

pub async fn set_position(pool: &Pool<Postgres>, x: i32, y: i32, token: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET position_x = $1, position_y = $2
            WHERE token = $3
        "#,
        x,
        y,
        token
    )
    .execute(pool)
    .await
    .context("Setting player location")?;

    Ok(())
}
