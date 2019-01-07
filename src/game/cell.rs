extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

const CELL_SIZE: usize = 16;

#[wasm_bindgen]
pub struct Cell {
    terrain: usize,
    item: usize,
    size: usize
}

#[wasm_bindgen]
impl Cell {
    pub fn new(terrain: usize, item: usize) -> Cell {
        Cell {
            terrain,
            item,
            size: CELL_SIZE
        }
    }

    pub fn terrain(&self) -> usize {
        self.terrain
    }

    pub fn item(&self) -> usize {
        self.item
    }

    pub fn size(&self) -> usize {
        self.size
    }
}