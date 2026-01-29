use eyre::{Context, Result};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub async fn get_players_in_lobby(
    game_id: Uuid,
    pool: &Pool<Postgres>,
) -> Result<Vec<DBLobbyPlayer>> {
    sqlx::query_as!(
        DBLobbyPlayer,
        r#"
            SELECT players.id, players.name, ships.name AS ship_class, ships.character AS ship_char, colors.name AS color
            FROM game_players
            JOIN games on games.id = game_players.game_id
            JOIN players on players.id = game_players.player_id
            JOIN ships on ships.id = players.ship_id
            JOIN colors on colors.id = players.color_id
            WHERE games.id = $1 AND games.status = 'lobby';"#,
        game_id
    )
    .fetch_all(pool)
    .await
    .context("getting all players of a game")
}

pub struct DBLobbyPlayer {
    pub id: Uuid,
    pub name: String,
    pub ship_class: String,
    pub ship_char: String,
    pub color: String,
}
