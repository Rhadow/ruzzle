// use web_sys::console::log_1;
use std::cell::RefCell;
use super::Cell;
use super::character::Character;
use super::movement_manager::Direction;
use super::Position;
use super::constants::{
    WORLD_WIDTH_IN_CELLS,
    WORLD_HEIGHT_IN_CELLS,
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct World {
    cells: Vec<Cell>,
    characters: Vec<RefCell<Box<dyn Character>>>,
}

impl World {
    pub fn new(cells: Vec<Cell>, characters: Vec<Box<dyn Character>>) -> World {
        let mut new_characters = vec![];
        for character in characters {
            new_characters.push(RefCell::new(character));
        }

        World {
            cells,
            characters: new_characters,
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        WORLD_WIDTH_IN_CELLS * row + col
    }

    pub fn width_in_cells(&self) -> usize {
        WORLD_WIDTH_IN_CELLS
    }

    pub fn height_in_cells(&self) -> usize {
        WORLD_HEIGHT_IN_CELLS
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn get_cell_by_position(&self, position: Position) -> &Cell {
        let idx = self.get_index(position.row() as usize, position.col() as usize);
        &self.cells[idx]
    }

    pub fn player(&self) -> &RefCell<Box<dyn Character>> {
        let idx = self.characters.len() - 1;
        &self.characters[idx]
    }

    pub fn _set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn get_characters(&self) -> &Vec<RefCell<Box<dyn Character>>> {
        &self.characters
    }

    pub fn update(&mut self, now: f64) {
        for character in &mut self.characters {
            character.borrow_mut().update(now);
        }
    }

    pub fn handle_direction_event(&mut self, key: &str) {
        let direction = match key {
            ARROW_UP => Some(Direction::Up),
            ARROW_DOWN => Some(Direction::Down),
            ARROW_LEFT => Some(Direction::Left),
            ARROW_RIGHT => Some(Direction::Right),
            _ => None
        };
        self.handle_player_movement(direction);
    }

    fn handle_player_movement(&mut self, direction: Option<Direction>) {
        if let Some(dir) = direction {
            let mut player = self.player().borrow_mut();
            player.step(dir, &self);
        }
    }
}