use crate::db::{self, games::DBGame, players::DBPlayer};
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
                    db::create_game::DBCreatedGameStatus::Playing => run_game(game, &pool).await?,
                    db::create_game::DBCreatedGameStatus::GameOver => todo!(),
                }
            }

            sleep(Duration::from_secs(10)).await;
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
    let all_ships = db::get_ships::get_all_ships(pool).await?;

    if players.len() == ready_count {
        let player_locations = init_player_locations(game.width, game.height, players.len());

        db::games::set_game_status(pool, game.id, db::create_game::DBCreatedGameStatus::Playing)
            .await?;
        db::players::unready_all_players_in_game(pool, game.id).await?;

        for player in players.iter() {
            let Some(ship) = all_ships
                .iter()
                .find(|ship| ship.name == player.ship_classname)
            else {
                continue;
            };
            let torpedoes = ship.max_torpedo_count;

            db::update_player::set_player_torpedoes(pool, player.token, torpedoes).await?;
        }

        for ((x, y), player) in player_locations.iter().zip(players) {
            db::players::set_player_position(pool, *x, *y, &player.id).await?;
        }

        db::game_turns::create_turn(pool, 1, game.id).await?;
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

async fn run_game(game: DBGame, pool: &Pool<Postgres>) -> Result<()> {
    let players = db::players::get_players_in_game(pool, game.id).await?;

    if players.iter().any(|player| !player.ready) {
        return Ok(());
    }

    let Some(game_turn) = db::game_turns::get_latest_player_turn(pool, players[0].id).await? else {
        return Ok(());
    };

    for player in players {
        let Some(player_turn) =
            db::player_turns::get_players_turn(pool, player.id, game_turn.id).await?
        else {
            continue;
        };

        if player_turn.speed_change != 0 {
            let speed = (player.speed + player_turn.speed_change).clamp(0, player.ship_max_speed);

            db::players::set_speed(pool, player.token, speed).await?;
        }

        if let Some(destination) = player_turn.destination_x.zip(player_turn.destination_y)
            && validate_destination(destination.0, destination.1, &player)
        {
            db::players::set_position(pool, destination.0, destination.1, player.token).await?;
        }
    }

    db::game_turns::mark_turn_not_active(pool, game_turn.id).await?;
    db::players::unready_all_players_in_game(pool, game.id).await?;
    db::game_turns::create_turn(pool, game_turn.turn_number + 1, game.id).await?;

    Ok(())
}

fn validate_destination(x: i32, y: i32, player: &DBPlayer) -> bool {
    let Some(player_x) = player.position_x else {
        return false;
    };
    let Some(player_y) = player.position_y else {
        return false;
    };
    let distance = distance_to(x, y, player_x, player_y);

    distance == player.speed
}

fn distance_to(first_x: i32, first_y: i32, second_x: i32, second_y: i32) -> i32 {
    let x = first_x - second_x;
    let y = first_y - second_y;

    (x.pow(2) + y.pow(2)).isqrt()
}
