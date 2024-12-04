use std::ops::Add;

use crate::Vec2;

#[derive(Debug, Clone)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<&Direction> for Vec2 {
    fn from(direction: &Direction) -> Self {
        match direction {
            Direction::N => Vec2::new(0, -1),
            Direction::NE => Vec2::new(1, -1),
            Direction::E => Vec2::new(1, 0),
            Direction::SE => Vec2::new(1, 1),
            Direction::S => Vec2::new(0, 1),
            Direction::SW => Vec2::new(-1, 1),
            Direction::W => Vec2::new(-1, 0),
            Direction::NW => Vec2::new(-1, -1),
        }
    }
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        Vec2::from(&direction)
    }
}

impl Add<&Direction> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: &Direction) -> Self::Output {
        let rhs: Vec2 = rhs.into();
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for &Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Direction) -> Self::Output {
        self + &rhs
    }
}
