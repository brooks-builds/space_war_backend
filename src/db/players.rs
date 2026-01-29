use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn delete_player(pool: &Pool<Postgres>, token: Uuid) -> Result<()> {
    sqlx::query!("DELETE FROM players WHERE token = $1", token)
        .execute(pool)
        .await
        .context("deleting player")?;

    Ok(())
}
