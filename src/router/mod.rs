mod change_player;
mod create_game;
mod game_routes;
mod get_lobby;
mod get_player;
mod get_ships;
mod healthcheck;
mod join_game;
mod players;

use crate::router::{create_game::create_game_route, healthcheck::healthcheck};
use axum::{
    Extension, Router,
    routing::{delete, get, post, put},
};
use sqlx::{Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_router(pg_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/games", post(create_game_route))
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
        .layer(Extension(pg_pool))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
