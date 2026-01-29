use axum::{Extension, Json, http::StatusCode};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::{self, get_ships::DBShip};

pub async fn get_ships(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<GetShipsResponse>>, (StatusCode, String)> {
    let ships = match db::get_ships::get_all_ships(&pool).await {
        Ok(ships) => ships,
        Err(error) => {
            eprintln!("Error getting all ships: {error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };
    let ships = ships.into_iter().map(Into::into).collect();

    Ok(Json(ships))
}

#[derive(Debug, Serialize)]
pub struct GetShipsResponse {
    id: Uuid,
    class_name: String,
    character: char,
}

impl From<DBShip> for GetShipsResponse {
    fn from(mut value: DBShip) -> Self {
        Self {
            id: value.id,
            class_name: value.name,
            character: value.character.pop().unwrap_or('*'),
        }
    }
}
