use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use time::format_description;
use uuid::Uuid;

use crate::db::{
    self,
    create_game::{DBCreatedGame, DBCreatedGameStatus},
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
        Json(CreateGameResponse::from(created_game)),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameData {
    pub player_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameResponse {
    pub id: Uuid,
    pub status: DBCreatedGameStatus,
    pub created_by_id: Uuid,
    pub created_at: String,
    pub code: i32,
}

impl From<DBCreatedGame> for CreateGameResponse {
    fn from(value: DBCreatedGame) -> Self {
        let time_formatter = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
        ).unwrap();

        Self {
            id: value.id,
            status: value.status,
            created_by_id: value.created_by_id,
            created_at: value.created_at.format(&time_formatter).unwrap(),
            code: value.code,
        }
    }
}
