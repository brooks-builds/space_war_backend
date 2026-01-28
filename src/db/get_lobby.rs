use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_players_in_lobby(
    game_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<Vec<DBLobbyPlayer>> {
    sqlx::query_as!(
        DBLobbyPlayer,
        r#"SELECT players.id, players.name FROM game_players JOIN games on games.id = game_players.game_id JOIN players on players.id = game_players.player_id WHERE games.id = $1 AND games.status = 'lobby';"#,
        game_id
    ).fetch_all(pool).await.context("getting all players of a game")
}

pub struct DBLobbyPlayer {
    pub id: Uuid,
    pub name: String,
}
