use axum::{Extension, Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db;

pub async fn create_game_route(
    Extension(db_pool): Extension<Pool<Postgres>>,
    Json(data): Json<CreateGameData>,
) -> Result<impl IntoResponse, StatusCode> {
    let created_game = match db::create_game::create_game(&data.player_name, &db_pool).await {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Error creating game: {error:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok((StatusCode::CREATED, Json(created_game)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGameData {
    pub player_name: String,
}
