use std::cmp::Reverse;
use std::collections::BinaryHeap;
use wasm_bindgen::prelude::*;

use crate::cell::Cell;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }
pub struct PathfinderNode {
    index: usize,
    f_score: f32,
}

impl PartialEq for PathfinderNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl std::fmt::Display for PathfinderNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "PathfinderNode(index: {}, score: {})",
            self.index, self.f_score
        )
    }
}

impl Eq for PathfinderNode {}

impl PartialOrd for PathfinderNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if other.f_score.is_infinite() && !self.f_score.is_infinite() {
            return Some(std::cmp::Ordering::Less);
        }

        if self.f_score.is_infinite() && !other.f_score.is_infinite() {
            return Some(std::cmp::Ordering::Greater);
        }

        self.f_score.partial_cmp(&other.f_score)
    }
}

impl Ord for PathfinderNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[wasm_bindgen]
pub struct Pathfinder {
    height: u32,
    width: u32,
    start_index: usize,
    end_index: usize,
    cells: Vec<Cell>,
    open_set: BinaryHeap<Reverse<PathfinderNode>>,
    closed_set: Vec<usize>,
    came_from: Vec<Option<usize>>,
    g_score: Vec<f32>,
    f_score: Vec<f32>,
    done: bool,
    current: usize,
}

#[wasm_bindgen]
impl Pathfinder {
    pub fn new(
        height: u32,
        width: u32,
        start_index: usize,
        end_index: usize,
        cells: Vec<Cell>,
    ) -> Pathfinder {
        let universe_size = (height * width) as usize; // the number of cells in the labyrinth
        let mut open_set = BinaryHeap::with_capacity(universe_size);

        open_set.push(Reverse(PathfinderNode {
            index: start_index,
            f_score: 0.0,
        }));

        let mut came_from = Vec::new();
        came_from.resize(universe_size, None);

        let mut g_score = Vec::new();
        g_score.resize(universe_size, std::f32::INFINITY);
        g_score[start_index] = 0.0;

        let mut f_score = Vec::new();
        f_score.resize(universe_size, std::f32::INFINITY);
        f_score[start_index] = (width + height) as f32; // largest possible manhattan distance

        Pathfinder {
            height,
            width,
            start_index,
            end_index,
            cells,
            open_set,
            closed_set: Vec::new(),
            came_from,
            g_score,
            f_score,
            done: false,
            current: start_index,
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn tick(&mut self) -> Option<usize> {
        if self.done {
            // log("pathfinder is done");
            return None;
        }

        if self.current == self.end_index || self.open_set.is_empty() {
            // log("pathfinder is done -- end index or open set is empty");
            self.done = true;
            return None;
        }

        let current = self.open_set.pop().unwrap().0;
        // log(&format!("current: {}", current));
        self.current = current.index;
        self.closed_set.push(current.index);

        let neighbors = self.neighbors();
        for neighbor in neighbors {
            // log(&format!("neighbor: {}", neighbor));
            let tentative_g_score = self.g_score[current.index] + 1.0;

            if tentative_g_score < self.g_score[neighbor] {
                self.came_from[neighbor] = Some(current.index);
                self.g_score[neighbor] = tentative_g_score;
                self.f_score[neighbor] = tentative_g_score
                    + self.manhattan_distance(
                        self.index_to_coordinates(neighbor),
                        self.index_to_coordinates(self.end_index),
                    );

                if !self.open_set_contains(neighbor) {
                    // log(&format!("pushing neighbor: {}", neighbor));
                    self.open_set.push(Reverse(PathfinderNode {
                        index: neighbor,
                        f_score: self.f_score[neighbor],
                    }));
                }
            }
        }

        Some(current.index)
    }

    #[wasm_bindgen]
    pub fn path(&self) -> js_sys::Array {
        let path = self.path_rust();

        let result = js_sys::Array::new_with_length(path.len() as u32);
        for (i, item) in path.iter().enumerate() {
            result.set(i as u32, JsValue::from(*item));
        }

        result
    }

    fn path_rust(&self) -> Vec<u32> {
        if self.done && self.current != self.end_index {
            return Vec::new();
        }

        let mut path = Vec::new();
        let mut current = self.current;

        while current != self.start_index {
            path.push(current as u32);
            current = self.came_from[current].unwrap();
        }

        path.push(self.start_index as u32);
        path.reverse();

        path
    }

    pub fn start_index(&self) -> usize {
        self.start_index
    }

    pub fn end_index(&self) -> usize {
        self.end_index
    }

    fn open_set_contains(&self, index: usize) -> bool {
        self.open_set.iter().any(|n| n.0.index == index)
    }

    fn neighbors(&self) -> Vec<usize> {
        let (row, col) = self.index_to_coordinates(self.current);

        let mut neighbors = Vec::new();
        let cell = self.cells[(row * self.width + col) as usize];

        if row > 0 && !cell.contains(Cell::TOP) {
            neighbors.push((row - 1, col));
        }

        if col > 0 && !cell.contains(Cell::LEFT) {
            neighbors.push((row, col - 1));
        }

        if row < self.height - 1 && !cell.contains(Cell::BOTTOM) {
            neighbors.push((row + 1, col));
        }

        if col < self.width - 1 && !cell.contains(Cell::RIGHT) {
            neighbors.push((row, col + 1));
        }

        neighbors
            .into_iter()
            .map(|(row, col)| (row * self.width + col) as usize)
            .collect()
    }

    fn manhattan_distance(&self, (y1, x1): (u32, u32), (y2, x2): (u32, u32)) -> f32 {
        let x_distance = if x1 > x2 { x1 - x2 } else { x2 - x1 };
        let y_distance = if y1 > y2 { y1 - y2 } else { y2 - y1 };

        (x_distance + y_distance) as f32
    }

    fn index_to_coordinates(&self, index: usize) -> (u32, u32) {
        (
            self.index_to_coordinates_row(index),
            self.index_to_coordinates_col(index),
        )
    }

    pub fn index_to_coordinates_row(&self, index: usize) -> u32 {
        index as u32 / self.width
    }

    pub fn index_to_coordinates_col(&self, index: usize) -> u32 {
        index as u32 % self.width
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_with_path() {
        let height = 2;
        let width = 2;
        let cells = vec![
            Cell::TOP | Cell::LEFT | Cell::RIGHT,
            Cell::TOP | Cell::LEFT | Cell::RIGHT,
            Cell::BOTTOM | Cell::LEFT,
            Cell::BOTTOM | Cell::RIGHT,
        ];

        let start_index = 0;
        let end_index = 1;

        let mut pathfinder = Pathfinder::new(height, width, start_index, end_index, cells);

        assert_eq!(pathfinder.tick(), Some(start_index));
        assert_eq!(pathfinder.tick(), Some(2));
        assert_eq!(pathfinder.tick(), Some(3));
        assert_eq!(pathfinder.tick(), Some(end_index));
        pathfinder.tick(); // noop
        pathfinder.tick(); // noop
        pathfinder.tick(); // noop

        assert_eq!(pathfinder.done(), true);
        let path = pathfinder.path_rust();
        assert_eq!(path.len(), 4);

        assert_eq!(path[0], start_index as u32);
        assert_eq!(path[3], end_index as u32);
    }

    #[test]
    pub fn test_no_path() {
        let height = 2;
        let width = 2;
        let cells = vec![
            Cell::TOP | Cell::LEFT | Cell::RIGHT,
            Cell::TOP | Cell::LEFT | Cell::RIGHT | Cell::BOTTOM, // target cell is unreachable
            Cell::BOTTOM | Cell::LEFT,
            Cell::BOTTOM | Cell::RIGHT | Cell::TOP,
        ];

        let start_index = 0;
        let end_index = 1;

        let mut pathfinder = Pathfinder::new(height, width, start_index, end_index, cells);

        assert_eq!(pathfinder.tick(), Some(start_index));
        assert_eq!(pathfinder.tick(), Some(2));
        assert_eq!(pathfinder.tick(), Some(3));
        assert_eq!(pathfinder.tick(), None);
        assert_eq!(pathfinder.tick(), None); // for good measure
        assert_eq!(pathfinder.tick(), None);
        assert_eq!(pathfinder.tick(), None);

        assert_eq!(pathfinder.done(), true);
        let path = pathfinder.path_rust();
        assert_eq!(path.len(), 0);
    }

    #[test]
    pub fn test_big_maze() {
        let height = 100;
        let width = 100;
        let maze = crate::maze_generator::MazeGenerator {
            height,
            width,
            start_coordinates: (0, 0),
        };
        let cells = maze.generate();

        let start_index = 0;
        let end_index = (height * width - 1) as usize;

        let mut pathfinder = Pathfinder::new(height, width, start_index, end_index, cells);

        // run for a while -- path is guaranteed to be found in less than height * width ticks
        for _ in 0..(height * width) {
            pathfinder.tick();
        }

        assert_eq!(pathfinder.done(), true);
        assert_eq!(pathfinder.current, end_index);

        let path = pathfinder.path_rust();
        assert!(path.len() < (height * width) as usize);
    }
}
