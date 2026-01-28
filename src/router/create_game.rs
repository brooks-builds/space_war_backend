use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::{
    self,
    create_game::{DBCreatedGame, DBCreatedGameStatus},
    create_player::DBCreatePlayer,
};

pub async fn create_game_route(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(data): Json<CreateGameData>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let player = match db::create_player::create_player(&data.player_name, &db_pool).await {
        Ok(player) => player,
        Err(error) => {
            eprintln!("Error creating player: {error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };
    let created_game = match db::create_game::create_game(player.id, &db_pool).await {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error creating game: {error:?}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };

    if let Err(error) = db::join_game::join_game(created_game.id, player.id, &db_pool).await {
        eprintln!("Error jointing game after creating it: {error:?}");

        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
    }

    Ok((
        StatusCode::CREATED,
        Json(CreateGameResponse::from((created_game, player))),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameData {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub game_id: Uuid,
    pub status: DBCreatedGameStatus,
    pub player_id: Uuid,
    pub token: Uuid,
    pub game_code: i32,
}

impl From<(DBCreatedGame, DBCreatePlayer)> for CreateGameResponse {
    fn from((db_created_game, db_create_player): (DBCreatedGame, DBCreatePlayer)) -> Self {
        Self {
            game_id: db_created_game.id,
            status: db_created_game.status,
            player_id: db_create_player.id,
            token: db_create_player.token,
            game_code: db_created_game.code,
        }
    }
}
