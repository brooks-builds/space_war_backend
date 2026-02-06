use crate::db::{self, games::DBGame};
use eyre::Result;
use sqlx::{Pool, Postgres};
use std::{thread, time::Duration};
use tokio::time::sleep;

pub async fn run_games(pool: Pool<Postgres>) -> tokio::task::JoinHandle<Result<()>> {
    tokio::task::spawn(async move {
        loop {
            let games = db::games::get_all_games(&pool).await?;

            for game in games {
                match game.status {
                    db::create_game::DBCreatedGameStatus::Lobby => {
                        run_game_lobby(&pool, &game).await?
                    }
                    db::create_game::DBCreatedGameStatus::Playing => {}
                    db::create_game::DBCreatedGameStatus::GameOver => todo!(),
                }
            }

            sleep(Duration::from_secs(30)).await;
        }
    })
}

async fn run_game_lobby(pool: &Pool<Postgres>, game: &DBGame) -> Result<()> {
    let players = db::players::get_players_in_game(pool, game.id).await?;
    let ready_count = players.iter().fold(
        0,
        |count, player| {
            if player.ready { count + 1 } else { count }
        },
    );

    if players.len() == ready_count {
        db::games::set_game_status(pool, game.id, db::create_game::DBCreatedGameStatus::Playing)
            .await?;

        db::players::unready_all_players_in_game(pool, game.id).await?;
    }

    Ok(())
}
