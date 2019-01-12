// use web_sys::console::log_1;
use std::cell::RefCell;
use super::Tile;
use super::character::Character;
use super::movement_manager::Direction;
use super::Position;
use super::constants::{
    WORLD_WIDTH_IN_TILES,
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct World {
    tile_map: Vec<Tile>,
    characters: Vec<RefCell<Box<dyn Character>>>,
}

impl World {
    pub fn new(tile_map: Vec<Tile>, characters: Vec<Box<dyn Character>>) -> World {
        let mut new_characters = vec![];
        for character in characters {
            new_characters.push(RefCell::new(character));
        }

        World {
            tile_map,
            characters: new_characters,
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        WORLD_WIDTH_IN_TILES * row + col
    }

    pub fn tile_map(&self) -> &Vec<Tile> {
        &self.tile_map
    }

    pub fn get_tile_by_position(&self, position: Position) -> &Tile {
        let idx = self.get_index(position.row() as usize, position.col() as usize);
        &self.tile_map[idx]
    }

    pub fn player(&self) -> &RefCell<Box<dyn Character>> {
        let idx = self.characters.len() - 1;
        &self.characters[idx]
    }

    pub fn _set_tile_map(&mut self, tile_map: Vec<Tile>) {
        self.tile_map = tile_map;
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