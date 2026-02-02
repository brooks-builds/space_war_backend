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
    let player = match db::players::get_player_by_token(&pool, token).await {
        Ok(Some(player)) => player,
        Ok(None) => {
            return Err((
                StatusCode::NOT_FOUND,
                "No player with that token and/or ID found".to_owned(),
            ));
        }
        Err(error) => {
            eprintln!("{error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error:?}")));
        }
    };

    // if the player is the host of the game
    //   then
    //     set another player to be the host
    //     if another player doesn't exist
    //       then
    //         delete the game
    // delete the player

    if let Some(game) = match db::games::get_game_created_by_player(&pool, player.id).await {
        Ok(game) => game,
        Err(error) => {
            eprintln!("{error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    } {
        let players_in_game = match db::players::get_players_in_game(&pool, game.id).await {
            Ok(players) => players,
            Err(error) => {
                eprintln!("{error}");

                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
            }
        };
        if let Some(new_host) = players_in_game
            .into_iter()
            .find(|game_player| game_player.id != player.id)
        {
            if let Err(error) = db::games::change_game_host(&pool, new_host.id, game.id).await {
                eprintln!("{error:?}");

                return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
            }
        } else if let Err(error) = db::games::delete_game(&pool, game.id).await {
            eprintln!("{error:?}");

            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    }

    if let Err(error) = db::players::delete_player(&pool, token).await {
        eprintln!("{error:?}");

        Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")))
    } else {
        Ok(StatusCode::OK)
    }
}
