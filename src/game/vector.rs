use serde::{Deserialize, Serialize};
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Calculate the steps between ourselves and another Vector.
    ///
    /// Using the algorithm from https://en.wikipedia.org/wiki/Line_drawing_algorithm (thanks to xXSuperCuberXx)
    pub fn steps_between(&self, destination: Self) -> Vec<Self> {
        let mut steps = vec![];
        let dx = (destination.x - self.x) as f32;
        let dy = (destination.y - self.y) as f32;
        let step = if dx.abs() >= dy.abs() {
            dx.abs()
        } else {
            dy.abs()
        };
        let dxm = dx / step;
        let dym = dy / step;
        let mut x = self.x as f32;
        let mut y = self.y as f32;
        let mut i = 0.0;

        while i <= step {
            steps.push(Vector::new(x.round() as i32, y.round() as i32));
            x += dxm;
            y += dym;
            i += 1.0;
        }

        steps
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Vector {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate::game::vector::Vector;

    #[test]
    fn steps_between_down_right() {
        let start = Vector::new(0, 0);
        let destination = Vector::new(9, 3);
        let expected = vec![
            Vector::new(0, 0),
            Vector::new(1, 0),
            Vector::new(2, 1),
            Vector::new(3, 1),
            Vector::new(4, 1),
            Vector::new(5, 2),
            Vector::new(6, 2),
            Vector::new(7, 2),
            Vector::new(8, 3),
            Vector::new(9, 3),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_up_right() {
        let start = Vector::new(0, 3);
        let destination = Vector::new(9, 0);
        let expected = vec![
            Vector::new(0, 3),
            Vector::new(1, 3),
            Vector::new(2, 2),
            Vector::new(3, 2),
            Vector::new(4, 2),
            Vector::new(5, 1),
            Vector::new(6, 1),
            Vector::new(7, 1),
            Vector::new(8, 0),
            Vector::new(9, 0),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_down_left() {
        let start = Vector::new(9, 0);
        let destination = Vector::new(0, 3);
        let expected = vec![
            Vector::new(9, 0),
            Vector::new(8, 0),
            Vector::new(7, 1),
            Vector::new(6, 1),
            Vector::new(5, 1),
            Vector::new(4, 2),
            Vector::new(3, 2),
            Vector::new(2, 2),
            Vector::new(1, 3),
            Vector::new(0, 3),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_up_left() {
        let start = Vector::new(9, 3);
        let destination = Vector::new(0, 0);
        let expected = vec![
            Vector::new(9, 3),
            Vector::new(8, 3),
            Vector::new(7, 2),
            Vector::new(6, 2),
            Vector::new(5, 2),
            Vector::new(4, 1),
            Vector::new(3, 1),
            Vector::new(2, 1),
            Vector::new(1, 0),
            Vector::new(0, 0),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_right() {
        let start = Vector::new(0, 0);
        let destination = Vector::new(5, 0);
        let expected = vec![
            Vector::new(0, 0),
            Vector::new(1, 0),
            Vector::new(2, 0),
            Vector::new(3, 0),
            Vector::new(4, 0),
            Vector::new(5, 0),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_down() {
        let start = Vector::new(0, 0);
        let destination = Vector::new(0, 5);
        let expected = vec![
            Vector::new(0, 0),
            Vector::new(0, 1),
            Vector::new(0, 2),
            Vector::new(0, 3),
            Vector::new(0, 4),
            Vector::new(0, 5),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_left() {
        let start = Vector::new(5, 0);
        let destination = Vector::new(0, 0);
        let expected = vec![
            Vector::new(5, 0),
            Vector::new(4, 0),
            Vector::new(3, 0),
            Vector::new(2, 0),
            Vector::new(1, 0),
            Vector::new(0, 0),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }

    #[test]
    fn steps_between_up() {
        let start = Vector::new(0, 5);
        let destination = Vector::new(0, 0);
        let expected = vec![
            Vector::new(0, 5),
            Vector::new(0, 4),
            Vector::new(0, 3),
            Vector::new(0, 2),
            Vector::new(0, 1),
            Vector::new(0, 0),
        ];
        let steps = start.steps_between(destination);

        assert_eq!(steps, expected);
    }
}
