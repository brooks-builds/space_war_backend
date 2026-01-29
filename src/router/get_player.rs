use axum::{Extension, Json, http::StatusCode};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db::{self, colors::DBColor};

pub async fn get_player_colors(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<ColorResponse>>, (StatusCode, String)> {
    let colors = match db::colors::get_colors(&pool).await {
        Ok(colors) => colors,
        Err(error) => {
            eprintln!("{error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };
    let colors = colors.into_iter().map(Into::into).collect();

    Ok(Json(colors))
}

#[derive(Debug, Serialize)]
pub struct ColorResponse {
    id: Uuid,
    name: String,
}

impl From<DBColor> for ColorResponse {
    fn from(value: DBColor) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
