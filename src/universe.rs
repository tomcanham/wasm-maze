use crate::a_star::Pathfinder;
use crate::cell::Cell;
use crate::maze_generator::MazeGenerator;
use crate::utils;
use wasm_bindgen::prelude::*;

type JSCoordinates = js_sys::Uint32Array;

fn to_js_coords((row, col): (u32, u32)) -> JSCoordinates {
    let coords = [row, col];
    JSCoordinates::from(&coords[..])
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    #[wasm_bindgen(skip)]
    pub cells: Vec<Cell>,
    #[wasm_bindgen(skip)]
    pub start_coordinates: (u32, u32),
    #[wasm_bindgen(skip)]
    pub end_coordinates: (u32, u32),
    #[wasm_bindgen(skip)]
    pub current: (u32, u32),
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let start_coordinates = (0, 0);
        let end_coordinates = (width - 1, height - 1);

        let maze_generator = MazeGenerator {
            width,
            height,
            start_coordinates,
        };

        let cells = maze_generator.generate();

        Universe {
            width,
            height,
            cells,
            start_coordinates,
            end_coordinates,
            current: start_coordinates,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen(getter = startCoordinates)]
    pub fn start_coordinates_js(&self) -> JSCoordinates {
        to_js_coords(self.start_coordinates)
    }

    pub fn start_index(&self) -> usize {
        (self.start_coordinates.0 * self.width + self.start_coordinates.1) as usize
    }

    #[wasm_bindgen(getter = endCoordinates)]
    pub fn end_coordinates_js(&self) -> JSCoordinates {
        to_js_coords(self.end_coordinates)
    }

    pub fn end_index(&self) -> usize {
        (self.end_coordinates.0 * self.width + self.end_coordinates.1) as usize
    }

    #[wasm_bindgen(getter = current)]
    pub fn current_coordinates_js(&self) -> JSCoordinates {
        to_js_coords(self.current)
    }

    pub fn current_index(&self) -> usize {
        (self.current.0 * self.width + self.current.1) as usize
    }

    // get the cells as a pointer to the first element
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn get_pathfinder(&self) -> Pathfinder {
        Pathfinder::new(
            self.height,
            self.width,
            self.start_index(),
            self.end_index(),
            self.cells.clone(),
        )
    }
}
