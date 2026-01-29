use axum::{
    Extension, Json,
    extract::Path,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db;

pub async fn change_player_ship(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(ship_id): Path<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(token) = headers.get("token") else {
        return Err((StatusCode::UNAUTHORIZED, "Missing token header".to_owned()));
    };
    let token = match Uuid::parse_str(token.to_str().unwrap_or_default()) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("{error:?}");

            return Err((
                StatusCode::BAD_REQUEST,
                "provided token was not a valid token".to_owned(),
            ));
        }
    };
    if let Err(error) = db::update_player::change_player_ship(&pool, ship_id, token).await {
        eprintln!("{error:?}");

        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")))
    } else {
        Ok(StatusCode::OK)
    }
}

pub async fn change_player_color(
    Extension(pool): Extension<Pool<Postgres>>,
    headers: HeaderMap,
    Json(data): Json<ChangePlayerColorData>,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(token) = headers.get("token") else {
        return Err((StatusCode::UNAUTHORIZED, "Missing token header".to_owned()));
    };
    let token = match Uuid::parse_str(token.to_str().unwrap_or_default()) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("{error:?}");

            return Err((
                StatusCode::BAD_REQUEST,
                "provided token was not a valid token".to_owned(),
            ));
        }
    };

    if let Err(error) = db::update_player::change_player_color(&pool, token, data.color_id).await {
        eprintln!("{error:?}");

        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")))
    } else {
        Ok(StatusCode::OK)
    }
}

#[derive(Debug, Deserialize)]
pub struct ChangePlayerColorData {
    color_id: Uuid,
}
