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
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::time::Duration;
use time::OffsetDateTime;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::db::{self, create_game::DBCreatedGameStatus, games::DBGame, players::DBPlayer};

#[axum::debug_handler]
pub async fn game_stream(
    Path(game_id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = stream::unfold((game_id, pool), |(game_id, pool)| async move {
        let game = db::games::get_game_by_id(&pool, game_id)
            .await
            .unwrap()
            .unwrap();
        let players = db::players::get_players_in_game(&pool, game_id)
            .await
            .unwrap();
        let game_stream = GameStream {
            game: game.into(),
            players: players.into_iter().map(Into::into).collect(),
        };

        Some((Event::default().json_data(game_stream), (game_id, pool)))
    })
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[derive(Debug, Serialize)]
pub struct GameStream {
    pub game: Game,
    pub players: Vec<GamePlayer>,
}

#[derive(Debug, Serialize)]
pub struct Game {
    pub id: Uuid,
    pub status: DBCreatedGameStatus,
    pub created_at: OffsetDateTime,
    pub host_id: Uuid,
    pub width: i32,
    pub height: i32,
    pub turn_number: i32,
}

impl From<DBGame> for Game {
    fn from(value: DBGame) -> Self {
        Self {
            id: value.id,
            status: value.status,
            created_at: value.created_at,
            host_id: value.host_id,
            width: value.width,
            height: value.height,
            turn_number: value.turn_number.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GamePlayer {
    pub id: Uuid,
    pub name: String,
    pub ship: char,
    pub ship_max_speed: i32,
    pub color: String,
    pub ready: bool,
    pub position_x: i32,
    pub position_y: i32,
    pub ship_classname: String,
}

impl From<DBPlayer> for GamePlayer {
    fn from(mut value: DBPlayer) -> Self {
        Self {
            id: value.id,
            name: value.name,
            ship: value.ship_char.pop().unwrap_or_default(),
            ship_max_speed: value.ship_max_speed,
            color: value.color,
            ready: value.ready,
            position_x: value.position_x.unwrap_or_default(),
            position_y: value.position_y.unwrap_or_default(),
            ship_classname: value.ship_classname,
        }
    }
}

pub async fn command(
    Extension(pool): Extension<Pool<Postgres>>,
    Extension(player): Extension<DBPlayer>,
    Json(command_body): Json<CommandRequestBody>,
) -> Result<StatusCode, (StatusCode, String)> {
    if player.ready {
        return Err((
            StatusCode::BAD_REQUEST,
            "Command already given this turn".to_owned(),
        ));
    }

    let turn = match db::game_turns::get_latest_player_turn(&pool, player.id).await {
        Ok(Some(turn)) => turn,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                "No turns for the game found".to_owned(),
            ));
        }
        Err(error) => {
            eprintln!("{error:?}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };

    if !turn.active {
        return Err((StatusCode::BAD_REQUEST, "Turn is not active".to_owned()));
    }

    let speed_change = command_body.speed_change.unwrap_or_default().clamp(-1, 1);
    let destination_x = command_body.destination.map(|destination| destination.0);
    let destination_y = command_body.destination.map(|destination| destination.1);

    if let Err(error) = db::player_turns::create_turn(
        &pool,
        player.id,
        turn.id,
        speed_change,
        destination_x,
        destination_y,
    )
    .await
    {
        eprintln!("{error:?}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
    }

    if let Err(error) = db::players::ready_up(&pool, player.token).await {
        eprintln!("{error:?}");
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
    }

    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize)]
pub struct CommandRequestBody {
    pub speed_change: Option<i32>,
    pub destination: Option<(i32, i32)>,
}
