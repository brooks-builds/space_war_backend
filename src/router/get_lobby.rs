use axum::{
    Extension, Json,
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use futures_util::{Stream, stream};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::db::{self, get_lobby::DBLobbyPlayer};

pub async fn get_lobby_stream_route(
    Path(game_code): Path<i32>,
    headers: HeaderMap,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let player_id = headers
        .get("player_id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let stream = stream::unfold(
        (game_code, player_id),
        |(game_code, player_id)| async move {
            let lobby_response = LobbyStreamResponse {
                game_code,
                player_id: player_id.clone(),
            };
            Some((
                Event::default().json_data(lobby_response),
                (game_code, player_id),
            ))
        },
    )
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Debug, Serialize)]
pub struct LobbyStreamResponse {
    pub game_code: i32,
    pub player_id: String,
}

pub async fn get_lobby_route(
    Path(game_code): Path<i32>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<LobbyResponse>, (StatusCode, String)> {
    let game = match db::get_game::get_game_by_code(game_code, &pool).await {
        Ok(Some(game)) => game,
        Ok(None) => return Err((StatusCode::NOT_FOUND, "Game not found".to_owned())),
        Err(error) => {
            eprintln!("Error getting game: {error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };
    let players_in_lobby = match db::get_lobby::get_players_in_lobby(game.id, &pool).await {
        Ok(players) => players,
        Err(error) => {
            eprintln!("Error getting players in lobby: {error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };
    let lobby_response = LobbyResponse {
        players: players_in_lobby.into_iter().map(Into::into).collect(),
    };

    Ok(Json(lobby_response))
}

#[derive(Debug, Serialize)]
pub struct LobbyResponse {
    players: Vec<LobbyPlayer>,
}

#[derive(Debug, Serialize)]
pub struct LobbyPlayer {
    pub name: String,
    pub id: Uuid,
}

impl From<DBLobbyPlayer> for LobbyPlayer {
    fn from(db_player: DBLobbyPlayer) -> Self {
        Self {
            id: db_player.id,
            name: db_player.name,
        }
    }
}
