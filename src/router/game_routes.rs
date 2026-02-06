use axum::{
    Extension,
    extract::Path,
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

#[axum::debug_handler]
pub async fn game_stream(
    Path(game_id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::unfold((game_id, pool), |(game_id, pool)| async move {
        let game_stream = GameStream {};

        Some((Event::default().json_data(game_stream), (game_id, pool)))
    })
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Debug, Serialize)]
pub struct GameStream {}
