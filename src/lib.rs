mod db;
mod game;
mod router;

use crate::router::create_router;
use eyre::Result;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio::net::TcpListener;

pub async fn run() -> Result<()> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;
    let db_pool = PgPoolOptions::new().connect(&database_url).await?;
    let app = create_router(db_pool.clone());
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let _game_thread = game::run_games(db_pool).await;

    axum::serve(listener, app).await?;

    Ok(())
}
