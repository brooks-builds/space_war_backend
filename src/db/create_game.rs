use eyre::{Context, Result};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn create_game(player_name: &str, pool: &Pool<Postgres>) -> Result<DBCreatedGame> {
    let created_player = super::create_player::create_player(player_name, pool).await?;

    sqlx::query_as!(
        DBCreatedGame,
        r#"insert into games (created_by_id) values ($1) returning id, status as "status: _", created_by_id "#,
        created_player.id
    )
    .fetch_one(pool)
    .await
    .context("creating game")
}

#[derive(Debug, Serialize)]
pub struct DBCreatedGame {
    pub id: Uuid,
    pub status: DBCreatedGameStatus,
    pub created_by_id: Uuid,
}

#[derive(sqlx::Type, Debug, Serialize)]
#[sqlx(type_name = "game_status", rename_all = "lowercase")]
pub enum DBCreatedGameStatus {
    Lobby,
    Playing,
    GameOver,
}

impl From<()> for DBCreatedGameStatus {
    fn from(_value: ()) -> Self {
        Self::Lobby
    }
}
