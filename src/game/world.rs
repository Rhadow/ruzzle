use super::Cell;
use super::character::Character;
use super::movement_manager::Direction;
use super::constants::{
    WORLD_WIDTH_IN_CELLS,
    WORLD_HEIGHT_IN_CELLS,
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct World {
    width_in_cells: usize,
    height_in_cells: usize,
    cells: Vec<Cell>,
    characters: Vec<Box<dyn Character>>,
}

impl World {
    pub fn new(cells: Vec<Cell>, characters: Vec<Box<dyn Character>>) -> World {
        World {
            width_in_cells: WORLD_WIDTH_IN_CELLS,
            height_in_cells: WORLD_HEIGHT_IN_CELLS,
            cells,
            characters,
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        self.width_in_cells * row + col
    }

    pub fn width_in_cells(&self) -> usize {
        self.width_in_cells
    }

    pub fn height_in_cells(&self) -> usize {
        self.height_in_cells
    }

    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    pub fn player(&mut self) -> &mut Box<dyn Character> {
        let idx = self.characters.len() - 1;
        &mut self.characters[idx]
    }

    pub fn _set_cells(&mut self, cells: Vec<Cell>) {
        self.cells = cells;
    }

    pub fn get_characters(&self) -> &Vec<Box<dyn Character>> {
        &self.characters
    }

    pub fn update(&mut self) {
        for character in &mut self.characters {
            character.update();
        }
    }

    pub fn handle_key_down_event(&mut self, key: &str) {
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
        let player = self.player();
        if let Some(dir) = direction {
            player.move_by_direction(dir);
        }
    }
}