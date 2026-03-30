mod simulate;
pub mod vector;

use crate::{
    db::{self, games::DBGame, players::DBPlayer},
    game::vector::Vector,
};
use eyre::Result;
use rand::{RngExt, rng};
use sqlx::{Pool, Postgres, types::Json};
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
                    db::create_game::DBCreatedGameStatus::GameOver => run_game_over(),
                }
            }

            sleep(Duration::from_secs(10)).await;
        }
    })
}

async fn run_game_lobby(pool: &Pool<Postgres>, game: &DBGame) -> Result<()> {
    let players = db::players::get_players_in_game(pool, game.id).await?;
    dbg!(&players);
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
            let hitpoints = ship.max_hitpoints;

            db::update_player::set_player_torpedoes(pool, player.token, torpedoes).await?;
            db::update_player::set_player_hitpoints(pool, player.token, hitpoints).await?;
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
    let players = db::players::get_players_in_game(pool, game.id)
        .await?
        .into_iter()
        .filter(|player| player.hitpoints.is_some_and(|hitpoints| hitpoints > 0))
        .collect::<Vec<DBPlayer>>();

    if players.len() <= 1 {
        dbg!("game is over, 1 or less players still alive");
        db::games::set_game_status(
            pool,
            game.id,
            db::create_game::DBCreatedGameStatus::GameOver,
        )
        .await?;
        return Ok(());
    }

    if players.iter().any(|player| !player.ready) {
        return Ok(());
    }

    let Some(game_turn) = db::game_turns::get_latest_player_turn(pool, players[0].id).await? else {
        return Ok(());
    };

    for player in players.iter() {
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
            && validate_destination(destination.0, destination.1, player)
        {
            db::players::set_position(pool, destination.0, destination.1, player.token).await?;
        }

        for other_player in players.iter() {
            if other_player.id == player.id {
                continue;
            }

            let Some(other_player_turn) =
                db::player_turns::get_players_turn(pool, other_player.id, game_turn.id).await?
            else {
                continue;
            };
            let torpedo_travel_steps = player_turn
                .torpedo_travel_steps
                .as_ref()
                .map(|steps| steps.0.clone());
            let other_ship_steps = other_player_turn.ship_travel_steps.map(|steps| steps.0);
            let other_ship_location = Vector::new(
                other_player_turn.destination_x.unwrap_or_default(),
                other_player_turn.destination_y.unwrap_or_default(),
            );

            if let Some(torpedo_step_index) = simulate::player_torpedo_hits(
                torpedo_travel_steps.clone(),
                other_ship_steps,
                other_ship_location,
            ) && let Some(torpedo_steps) = &torpedo_travel_steps
            {
                let shortened_torpedo_steps = &torpedo_steps[..torpedo_step_index];

                db::update_player::update_player_hitpoints(pool, other_player.token, -5).await?;
                db::player_turns::update_torpedo_steps(
                    pool,
                    player_turn.id,
                    Some(Json(shortened_torpedo_steps.to_vec())),
                )
                .await?;
            }
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

fn run_game_over() {}
