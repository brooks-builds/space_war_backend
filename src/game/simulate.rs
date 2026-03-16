use eyre::Result;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{db::player_turns::get_all_player_turns_by_game_turn_id, game::vector::Vector};

pub fn player_torpedo_hits(
    torpedo_steps: Option<Vec<Vector>>,
    ship_steps: Option<Vec<Vector>>,
    ship_end_position: Vector,
) -> Option<usize> {
    let ship_steps = ship_steps.unwrap_or_else(|| vec![ship_end_position]);

    for (torpedo_step_index, torpedo_step) in torpedo_steps?.iter().enumerate() {
        let ship_step = ship_steps
            .get(torpedo_step_index)
            .unwrap_or_else(|| &ship_end_position);

        if torpedo_step == ship_step {
            return Some(torpedo_step_index);
        }
    }

    None
}

mod tests {
    use crate::game::vector::Vector;

    use super::*;

    #[test]
    fn player_fires_at_other_who_does_not_move() {
        let torpedo_steps = Some(vec![
            Vector::new(0, 0),
            Vector::new(1, 0),
            Vector::new(2, 0),
            Vector::new(3, 0),
            Vector::new(4, 0),
            Vector::new(5, 0),
        ]);
        let other_ship_steps = None;
        let other_ship_location = Vector::new(5, 0);
        let expected = Some(5_usize);
        let torpedo_hits =
            player_torpedo_hits(torpedo_steps, other_ship_steps, other_ship_location);

        assert_eq!(torpedo_hits, expected);
    }

    #[test]
    fn player_fires_at_other_who_moves_away() {
        let torpedo_steps = Some(vec![
            Vector::new(0, 0),
            Vector::new(1, 0),
            Vector::new(2, 0),
            Vector::new(3, 0),
            Vector::new(4, 0),
            Vector::new(5, 0),
        ]);
        let other_ship_steps = Some(vec![
            Vector::new(5, 0),
            Vector::new(6, 0),
            Vector::new(7, 0),
        ]);
        let other_ship_location = Vector::new(7, 0);
        let expected = None;
        let torpedo_hits =
            player_torpedo_hits(torpedo_steps, other_ship_steps, other_ship_location);

        assert_eq!(torpedo_hits, expected);
    }

    #[test]
    fn player_does_not_fire() {
        let torpedo_steps = None;
        let other_ship_steps = None;
        let other_ship_location = Vector::new(5, 0);
        let expected = None;
        let torpedo_hits =
            player_torpedo_hits(torpedo_steps, other_ship_steps, other_ship_location);

        assert_eq!(torpedo_hits, expected);
    }
}
