use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn change_player_ship(pool: &Pool<Postgres>, ship_id: Uuid, token: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET ship_id = $1
            WHERE token = $2
            AND (
                SELECT status
                FROM game_players
                RIGHT OUTER JOIN games ON games.id = game_players.game_id
                RIGHT OUTER JOIN players ON players.id = game_players.player_id
                WHERE players.token = $2
            ) = 'lobby';
        "#,
        ship_id,
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn change_player_color(pool: &Pool<Postgres>, token: Uuid, color_id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET color_id = $1
            WHERE token = $2
            AND (
                SELECT status
                FROM game_players
                RIGHT OUTER JOIN games ON games.id = game_players.game_id
                RIGHT OUTER JOIN players ON players.id = game_players.player_id
                WHERE players.token = $2
            ) = 'lobby'
        "#,
        color_id,
        token
    )
    .execute(pool)
    .await
    .context("changing player color")?;

    Ok(())
}

pub async fn set_player_torpedoes(
    pool: &Pool<Postgres>,
    token: Uuid,
    torpedo_count: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET torpedo_count = $1
            WHERE token = $2
        "#,
        torpedo_count,
        token
    )
    .execute(pool)
    .await
    .context("Setting max torpedo count on player")?;

    Ok(())
}

pub async fn set_player_hitpoints(
    pool: &Pool<Postgres>,
    token: Uuid,
    hitpoints: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET hitpoints = $1
            WHERE token = $2
        "#,
        hitpoints,
        token,
    )
    .execute(pool)
    .await
    .context("Setting player hitpoints")?;

    Ok(())
}

pub async fn update_player_hitpoints(
    pool: &Pool<Postgres>,
    token: Uuid,
    hitpoint_change: i32,
) -> Result<()> {
    sqlx::query!(
        r#"
            UPDATE players
            SET hitpoints = hitpoints + $1
            WHERE token = $2
        "#,
        hitpoint_change,
        token
    )
    .execute(pool)
    .await
    .context("Updating the player hitpoints")?;

    Ok(())
}
