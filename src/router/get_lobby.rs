use axum::{
    Extension, Json,
    extract::Path,
    http::StatusCode,
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
    Path(game_id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::unfold((game_id, pool), |(game_id, pool)| async move {
        let players_in_lobby = db::get_lobby::get_players_in_lobby(game_id, &pool)
            .await
            .unwrap();
        let lobby_response = LobbyResponse {
            players: players_in_lobby.into_iter().map(Into::into).collect(),
        };
        Some((Event::default().json_data(lobby_response), (game_id, pool)))
    })
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

pub async fn get_lobby_route(
    Path(game_id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<LobbyResponse>, (StatusCode, String)> {
    let players_in_lobby = match db::get_lobby::get_players_in_lobby(game_id, &pool).await {
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
    pub ship_class: String,
    pub ship_character: char,
}

impl From<DBLobbyPlayer> for LobbyPlayer {
    fn from(mut db_player: DBLobbyPlayer) -> Self {
        Self {
            id: db_player.id,
            name: db_player.name,
            ship_class: db_player.ship_class,
            ship_character: db_player.ship_char.pop().unwrap_or('*'),
        }
    }
}
