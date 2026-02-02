use eyre::{Context, Result};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_game_created_by_player(
    pool: &Pool<Postgres>,
    player_id: Uuid,
) -> Result<Option<DBGame>> {
    sqlx::query_as!(DBGame, "SELECT id FROM games WHERE host_id = $1", player_id)
        .fetch_optional(pool)
        .await
        .context("Getting game owned by player")
}

#[derive(Serialize, Debug)]
pub struct DBGame {
    pub id: Uuid,
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
