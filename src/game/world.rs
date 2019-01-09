use super::Cell;
use super::constants::{
    WORLD_WIDTH_IN_CELLS,
    WORLD_HEIGHT_IN_CELLS
};

pub struct World {
    width: usize,
    height: usize,
    cells: Vec<Cell>
}

impl World {
    pub fn new() -> World {
        World {
            width: WORLD_WIDTH_IN_CELLS,
            height: WORLD_HEIGHT_IN_CELLS,
            cells: vec![]
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

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }
}