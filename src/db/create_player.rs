use eyre::{Context, Result};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_player(name: &str, pool: &Pool<Postgres>) -> Result<DBCreatePlayer> {
    sqlx::query_as!(
        DBCreatePlayer,
        "insert into players (name) values ($1) returning id, token",
        name
    )
    .fetch_one(pool)
    .await
    .context("creating player in database")
}

#[derive(Debug, Serialize)]
pub struct DBCreatePlayer {
    pub id: uuid::Uuid,
    pub token: Uuid,
}
