use axum::{
    extract::Path,
    http::HeaderMap,
    response::{
        Sse,
        sse::{Event, KeepAlive},
    },
};
use futures_util::{Stream, stream};
use serde::Serialize;
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn get_lobby_route(
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
            let lobby_response = LobbyResponse {
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
pub struct LobbyResponse {
    pub game_code: i32,
    pub player_id: String,
}
