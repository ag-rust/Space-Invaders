use default::*;

pub enum Direction {
    Right,
    Left,
}

impl Direction {
    pub fn flip(&self) -> Direction {
        match self {
            &Direction::Right => Direction::Left,
            &Direction::Left => Direction::Right,
        }
    }
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Right
    }
}

#[derive(Copy, Clone)]
pub enum Orientation {
    Up,
    Down,
}
