mod create_game;
mod healthcheck;

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
        .route("/api/healthcheck", get(healthcheck))
        .layer(Extension(pg_pool))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
