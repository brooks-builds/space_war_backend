mod change_player;
mod create_game;
mod game_routes;
mod get_lobby;
mod get_player;
mod get_ships;
mod healthcheck;
mod join_game;
mod players;

use crate::{
    db,
    router::{create_game::create_game_route, healthcheck::healthcheck},
};
use axum::{
    Extension, Router,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post, put},
};
use sqlx::{Pool, Postgres};
use std::str::FromStr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub fn create_router(pg_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/players/decrease_speed", post(players::decrease_speed))
        .route("/api/games/{game_id}/command", post(game_routes::command))
        .route_layer(middleware::from_fn(protect))
        .route("/api/games", post(create_game_route))
        .route("/api/games/{game_id}", get(game_routes::get_game_by_id))
        .route("/api/games/join", post(join_game::join_game_route))
        .route(
            "/api/games/{game_id}/lobby",
            get(get_lobby::get_lobby_route),
        )
        .route(
            "/api/games/{game_id}/lobby/stream",
            get(get_lobby::get_lobby_stream_route),
        )
        .route("/api/games/{game_id}/stream", get(game_routes::game_stream))
        .route("/api/ships", get(get_ships::get_ships))
        .route(
            "/api/players/ship/{ship_id}",
            put(change_player::change_player_ship),
        )
        .route("/api/players/colors", get(get_player::get_player_colors))
        .route(
            "/api/players/colors",
            put(change_player::change_player_color),
        )
        .route("/api/healthcheck", get(healthcheck))
        .route("/api/players", delete(players::delete_player))
        .route("/api/players/ready_up", put(players::ready_up))
        .route("/api/players", get(players::get_player))
        .route("/api/players/increase_speed", post(players::increase_speed))
        .layer(Extension(pg_pool))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

pub async fn protect(
    headers: HeaderMap,
    Extension(pool): Extension<Pool<Postgres>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
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
                format!("Player with provided token {token} not found"),
            ));
        }
        Err(error) => {
            eprintln!("{error:?}");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{error}")));
        }
    };

    let extensions = request.extensions_mut();

    extensions.insert(player);

    let response = next.run(request).await;

    Ok(response)
}
