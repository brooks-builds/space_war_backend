use std::ops::{RemAssign, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn steps_between(&self, other: Self) -> Vec<Self> {
        let mut steps = vec![];
        let direction = other - *self;
        let ratio = direction.ratio();
        let mut previous_step = *self;

        #[allow(clippy::never_loop)]
        loop {
            let mut next_step = steps.last().unwrap_or_else(|| self).clone();
            let mut ratio = if ratio.x == 0 && ratio.y == 0 {
                direction.ratio()
            } else {
                ratio
            };

            if ratio.x > ratio.y {
                next_step.x += direction.x / direction.x.abs();
                ratio.x -= 1;
            } else if ratio.x < ratio.y {
                next_step.y += direction.y / direction.y.abs();
                ratio.y -= 1;
            } else {
                next_step.x += direction.x / direction.x.abs();
                next_step.y += direction.y.checked_div(direction.y.abs()).unwrap_or(1);
                ratio.x -= 1;
                ratio.y -= 1;
            }

            steps.push(next_step);

            if next_step == other {
                break;
            }
        }

        steps
    }

    pub fn x_larger_or_equal(&self) -> bool {
        self.x >= self.y
    }

    pub fn ratio(&self) -> Self {
        let x;
        let y;

        if self.x > self.y {
            x = self.x.checked_div(self.y).unwrap_or(1);
            y = self.x.checked_rem(self.y).unwrap_or(0);
        } else if self.x < self.y {
            y = self.y.checked_div(self.x).unwrap_or(1);
            x = if self.y == 0 {
                1
            } else {
                self.y.checked_rem(self.x).unwrap_or(0)
            };
        } else {
            x = 1;
            y = 1;
        };

        Self::new(x, y)
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

mod tests {
    #[allow(unused_imports)]
    use crate::game::vector::Vector;

    #[test]
    fn x_is_larger() {
        let first = Vector::new(5, 6);
        let second = Vector::new(6, 5);

        assert!(second.x_larger_or_equal());
        assert!(!first.x_larger_or_equal());
    }

    #[test]
    fn steps_between_x_only() {
        let start = Vector::new(0, 0);
        let end = Vector::new(5, 0);
        let expected = vec![
            Vector::new(1, 0),
            Vector::new(2, 0),
            Vector::new(3, 0),
            Vector::new(4, 0),
            Vector::new(5, 0),
        ];
        let steps = start.steps_between(end);

        assert_eq!(expected, steps);
    }

    #[test]
    fn steps_between_y_only() {
        let start = Vector::new(0, 0);
        let end = Vector::new(0, 5);
        let expected = vec![
            Vector::new(0, 1),
            Vector::new(0, 2),
            Vector::new(0, 3),
            Vector::new(0, 4),
            Vector::new(0, 5),
        ];
        let steps = start.steps_between(end);

        assert_eq!(expected, steps);
    }

    #[test]
    fn stepping_from_right_to_left() {
        let start = Vector::new(5, 0);
        let end = Vector::new(2, 0);
        let expected = vec![Vector::new(4, 0), Vector::new(3, 0), Vector::new(2, 0)];
        let steps = start.steps_between(end);

        assert_eq!(steps, expected);
    }

    #[test]
    fn stepping_diagonal_right() {
        let start = Vector::new(0, 0);
        let end = Vector::new(5, 5);
        let expected = vec![
            Vector::new(1, 1),
            Vector::new(2, 2),
            Vector::new(3, 3),
            Vector::new(4, 4),
            Vector::new(5, 5),
        ];
        let steps = start.steps_between(end);

        assert_eq!(steps, expected);
    }

    #[test]
    fn stepping_diagonal_left() {
        let start = Vector::new(5, 5);
        let end = Vector::new(0, 0);
        let expected = vec![
            Vector::new(4, 4),
            Vector::new(3, 3),
            Vector::new(2, 2),
            Vector::new(1, 1),
            Vector::new(0, 0),
        ];
        let steps = start.steps_between(end);

        assert_eq!(steps, expected);
    }

    #[test]
    fn stepping_not_even_right() {
        let start = Vector::new(0, 0);
        let end = Vector::new(5, 2);
        let expected = vec![
            Vector::new(1, 0),
            Vector::new(2, 1),
            Vector::new(3, 1),
            Vector::new(4, 2),
            Vector::new(5, 2),
        ];
        let steps = start.steps_between(end);

        assert_eq!(steps, expected);
    }

    #[test]
    fn get_ratio() {
        let vector = Vector::new(5, 2);
        let ratio = vector.ratio();
        let expected = Vector::new(2, 1);

        assert_eq!(ratio, expected);
    }

    #[test]
    fn ratio_for_diagonal() {
        let vector = Vector::new(5, 5);
        let ratio = vector.ratio();
        let expected = Vector::new(1, 1);

        assert_eq!(ratio, expected);
    }

    #[test]
    fn ratio_3_0() {
        let vector = Vector::new(3, 0);
        let ratio = vector.ratio();
        let expected = Vector::new(1, 0);

        assert_eq!(ratio, expected);
    }

    #[test]
    fn ratio_negative3_0() {
        let vector = Vector::new(-3, 0);
        let ratio = vector.ratio();
        let expected = Vector::new(1, 0);

        assert_eq!(ratio, expected);
    }
}
