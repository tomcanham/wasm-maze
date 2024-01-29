use crate::direction::Direction;
use bitflags::bitflags;
use wasm_bindgen::prelude::*;

bitflags! {
    #[wasm_bindgen]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Cell: u8 {
        const TOP = 0b0001;
        const BOTTOM = 0b0010;
        const LEFT = 0b0100;
        const RIGHT = 0b1000;

        const ALL = Self::TOP.bits() | Self::RIGHT.bits() | Self::BOTTOM.bits() | Self::LEFT.bits();
        const NONE = 0;
    }
}

impl Cell {
    pub fn set_wall(&mut self, direction: Direction) {
        *self |= match direction {
            Direction::Up => Cell::TOP,
            Direction::Right => Cell::RIGHT,
            Direction::Down => Cell::BOTTOM,
            Direction::Left => Cell::LEFT,
        };
    }

    pub fn clear_wall(&mut self, direction: Direction) {
        *self &= match direction {
            Direction::Up => !Cell::TOP,
            Direction::Right => !Cell::RIGHT,
            Direction::Down => !Cell::BOTTOM,
            Direction::Left => !Cell::LEFT,
        };
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let top = if self.contains(Cell::TOP) { " " } else { "_" };
        let right = if self.contains(Cell::RIGHT) { " " } else { "|" };
        let bottom = if self.contains(Cell::BOTTOM) {
            " "
        } else {
            "_"
        };
        let left = if self.contains(Cell::LEFT) { " " } else { "|" };

        write!(f, "{}{}{}{}", top, right, bottom, left)
    }
}
