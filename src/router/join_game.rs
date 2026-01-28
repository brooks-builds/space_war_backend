use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::{self, get_game::get_game_by_code, join_game::join_game};

pub async fn join_game_route(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(data): Json<JoinGameData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let game = match get_game_by_code(data.code, &db_pool).await {
        Ok(Some(game)) => game,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                "Game with provided code not found".to_owned(),
            ));
        }
        Err(error) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}"))),
    };
    let player = match db::create_player::create_player(&data.player_name, &db_pool).await {
        Ok(player) => player,
        Err(error) => {
            eprintln!("Error creating a player: {error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };

    if let Err(error) = join_game(game.id, player.id, &db_pool).await {
        eprintln!("Error having a player join a game: {error:?}");

        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
    }

    let join_game_response = JoinGameResponse {
        token: player.token,
    };

    Ok((StatusCode::CREATED, Json(join_game_response)))
}

#[derive(Debug, Deserialize)]
pub struct JoinGameData {
    player_name: String,
    code: i32,
}

#[derive(Debug, Serialize)]
pub struct JoinGameResponse {
    pub token: Uuid,
}
