use crate::{
    db::{games::DBGame, player_turns::DBPlayerTurn, players::DBPlayer},
    game::vector::Vector,
};

pub fn simulate_turn(players: &[(DBPlayer, DBPlayerTurn)], game: &DBGame) {
    let mut player_ids = vec![];
    let mut locations = vec![];
    let mut flying_to = vec![];
    // let mut ship_steps = vec![];
    let mut speeds = vec![];
    let mut torpedo_start_positions = vec![];
    let mut torpedo_targets = vec![];

    for (player, turn) in players.iter() {
        let location = player
            .position_x
            .zip(player.position_y)
            .map(|(x, y)| Vector::new(x, y));
        let player_destination = turn
            .destination_x
            .zip(turn.destination_y)
            .map(|(x, y)| Vector::new(x, y));
        let torpedo_target = turn
            .torpedo_target_x
            .zip(turn.torpedo_target_y)
            .map(|(x, y)| Vector::new(x, y));
        let torpedo_start_position = location.map(|location| location);

        player_ids.push(player.id);
        locations.push(location);
        flying_to.push(player_destination);
        speeds.push(player.speed);
        torpedo_start_positions.push(torpedo_start_position);
        torpedo_targets.push(torpedo_target);
    }
}
