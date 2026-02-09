use crate::db::{self, games::DBGame};
use eyre::Result;
use rand::{RngExt, rng};
use sqlx::{Pool, Postgres};
use std::time::Duration;
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
        let player_locations = init_player_locations(game.width, game.height, players.len());

        db::games::set_game_status(pool, game.id, db::create_game::DBCreatedGameStatus::Playing)
            .await?;
        db::players::unready_all_players_in_game(pool, game.id).await?;

        for ((x, y), player) in player_locations.iter().zip(players) {
            db::players::set_player_position(pool, *x, *y, &player.id).await?;
        }
    }

    Ok(())
}

fn init_player_locations(
    game_width: i32,
    game_height: i32,
    mut player_count: usize,
) -> Vec<(i32, i32)> {
    let mut locations = vec![];

    if player_count > 0 {
        locations.push((0, 0));
        player_count -= 1;
    }

    if player_count > 0 {
        locations.push((game_width - 1, 0));
        player_count -= 1;
    }

    if player_count > 0 {
        locations.push((0, game_height - 1));
        player_count -= 1;
    }

    if player_count > 0 {
        locations.push((game_width - 1, game_height - 1));
        player_count -= 1;
    }

    while player_count > 0 {
        let x = rng().random_range(0..game_width);
        let y = rng().random_range(0..game_height);

        if locations
            .iter()
            .any(|(location_x, location_y)| *location_x == x && *location_y == y)
        {
            continue;
        }

        locations.push((x, y));
        player_count -= 1;
    }

    locations
}
