use rand::prelude::*;

use crate::cell::Cell;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn get_random_directions() -> Vec<Direction> {
        let mut rng = rand::thread_rng();
        let mut directions = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        directions.shuffle(&mut rng);
        directions
    }

    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn get_mask(&self) -> Cell {
        match self {
            Direction::Up => !Cell::TOP,
            Direction::Down => !Cell::BOTTOM,
            Direction::Left => !Cell::LEFT,
            Direction::Right => !Cell::RIGHT,
        }
    }
}
