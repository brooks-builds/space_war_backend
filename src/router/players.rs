use std::str::FromStr;

use axum::{
    Extension,
    http::{HeaderMap, StatusCode},
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::db;

pub async fn delete_player(
    Extension(pool): Extension<Pool<Postgres>>,
    headers: HeaderMap,
) -> Result<StatusCode, (StatusCode, String)> {
    let Some(token) = headers.get("token") else {
        return Err((StatusCode::UNAUTHORIZED, "Missing token header".to_owned()));
    };
    let Ok(token) = token.to_str() else {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Token header is not a string".to_owned(),
        ));
    };
    let token = match Uuid::from_str(token) {
        Ok(token) => token,
        Err(error) => {
            eprintln!("{error:?}");

            return Err((StatusCode::UNAUTHORIZED, format!("{error}")));
        }
    };

    if let Err(error) = db::players::delete_player(&pool, token).await {
        eprintln!("{error:?}");

        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")))
    } else {
        Ok(StatusCode::OK)
    }
}
