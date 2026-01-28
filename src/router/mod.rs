mod create_game;
mod get_lobby;
mod healthcheck;
mod join_game;

use crate::router::{create_game::create_game_route, healthcheck::healthcheck};
use axum::{
    Extension, Router,
    routing::{get, post},
};
use sqlx::{Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_router(pg_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/api/games", post(create_game_route))
        .route("/api/games/join", post(join_game::join_game_route))
        .route("/api/games/{code}/lobby", get(get_lobby::get_lobby_route))
        .route("/api/healthcheck", get(healthcheck))
        .layer(Extension(pg_pool))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
