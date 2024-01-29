use crate::{cell::Cell, direction::Direction};
use std::collections::HashMap;

pub type Coordinates = (u32, u32);

pub struct MazeGenerator {
    pub height: u32,
    pub width: u32,
    pub start_coordinates: Coordinates,
}

impl Default for MazeGenerator {
    fn default() -> Self {
        MazeGenerator {
            height: 64,
            width: 64,
            start_coordinates: (0, 0),
        }
    }
}

pub struct MazeGeneratorContext {
    cells: Vec<Cell>,
    visited: HashMap<Coordinates, bool>,
    width: u32,
}

impl MazeGeneratorContext {
    pub fn new(mg: &MazeGenerator) -> MazeGeneratorContext {
        MazeGeneratorContext {
            cells: vec![Cell::ALL; (mg.height * mg.width) as usize],
            visited: HashMap::new(),
            width: mg.width,
        }
    }

    pub fn clear_wall(&mut self, coordinates: Coordinates, direction: &Direction) {
        let (row, col) = coordinates;
        let index = row * self.width + col;
        self.cells[index as usize] &= direction.get_mask();
    }
}

impl MazeGenerator {
    pub fn generate(&self) -> Vec<Cell> {
        let mut ctx = MazeGeneratorContext::new(self);

        self.generate_from(&mut ctx, self.start_coordinates);
        ctx.cells
    }

    pub fn generate_from(&self, ctx: &mut MazeGeneratorContext, from: Coordinates) {
        let directions = Direction::get_random_directions();

        ctx.visited.insert(from, true);
        for direction in directions {
            let neighbor_pos = self.get_adjusted_coordinates(from, &direction);
            if let Some(to) = neighbor_pos {
                if !ctx.visited.contains_key(&to) {
                    ctx.clear_wall(from, &direction);
                    ctx.clear_wall(to, &direction.get_opposite());
                    self.generate_from(ctx, to);
                }
            }
        }
    }

    fn get_adjusted_coordinates(
        &self,
        coordinates: Coordinates,
        direction: &Direction,
    ) -> Option<Coordinates> {
        let (row, col) = coordinates;
        match direction {
            Direction::Up => {
                if row > 0 {
                    Some((row - 1, col))
                } else {
                    None
                }
            }
            Direction::Left => {
                if col > 0 {
                    Some((row, col - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if row < self.height - 1 {
                    Some((row + 1, col))
                } else {
                    None
                }
            }
            Direction::Right => {
                if col < self.width - 1 {
                    Some((row, col + 1))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate() {
        let maze_generator = MazeGenerator {
            height: 4,
            width: 4,
            ..MazeGenerator::default()
        };

        let cells = maze_generator.generate();

        assert_eq!(cells.len(), 4096);
    }
}
