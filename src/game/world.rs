extern crate wasm_bindgen;

use super::Cell;

use wasm_bindgen::prelude::*;

const WORLD_WIDTH: usize = 12;
const WORLD_HEIGHT: usize = 8;

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        let cells = (0..WORLD_WIDTH * WORLD_HEIGHT).map(|i| {
            Cell::new(i, i)
        }).collect();
        World {
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
            cells
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        self.width * row + col
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}