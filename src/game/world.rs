// use web_sys::console::log_1;
// log_1(&format!("{}", self.objects.len()).into());
use std::cell::RefCell;
use crate::audio::WebAudioPlayer;
use super::Tile;
use super::character::{Character};
use super::object::Object;
use super::movement_manager::{
    Direction,
    Status
};
use super::Position;
use super::constants::{
    WORLD_WIDTH_IN_TILES,
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct World {
    tile_map: Vec<RefCell<Tile>>,
    objects: Vec<RefCell<Box<dyn Object>>>,
    characters: Vec<RefCell<Box<dyn Character>>>,
}

impl World {
    pub fn new(tile_map: Vec<Tile>, objects: Vec<RefCell<Box<dyn Object>>>, characters: Vec<Box<dyn Character>>) -> World {
        let mut new_characters = vec![];
        let mut new_tile_map = vec![];
        for character in characters {
            new_characters.push(RefCell::new(character));
        }
        for tile in tile_map {
            new_tile_map.push(RefCell::new(tile));
        }

        World {
            tile_map: new_tile_map,
            objects,
            characters: new_characters,
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        WORLD_WIDTH_IN_TILES * row + col
    }

    pub fn tile_map(&self) -> &Vec<RefCell<Tile>> {
        &self.tile_map
    }

    pub fn get_object_by_position(&self, position: &Position) -> Option<&RefCell<Box<dyn Object>>> {
        let result = None;
        for object in &self.objects {
            if object.borrow().is_visible() {
                let object_position = object.borrow().movement_manager().position;
                if object_position.row() == position.row() && object_position.col() == position.col() {
                    return Some(object);
                }
            }
        }
        return result;
    }

    pub fn player(&self) -> &RefCell<Box<dyn Character>> {
        let idx = self.characters.len() - 1;
        &self.characters[idx]
    }

    pub fn _set_tile_map(&mut self, tile_map: Vec<Tile>) {
        let mut new_tile_map = vec![];
        for tile in tile_map {
            new_tile_map.push(RefCell::new(tile));
        }
        self.tile_map = new_tile_map;
    }

    pub fn get_characters(&self) -> &Vec<RefCell<Box<dyn Character>>> {
        &self.characters
    }

    pub fn get_objects(&self) -> &Vec<RefCell<Box<dyn Object>>> {
        &self.objects
    }

    fn remove_invisible_items(&mut self) {
        self.objects.iter()
            .position(|o| !o.borrow().is_visible())
            .map(|idx| self.objects.remove(idx));
    }

    pub fn update(&mut self, now: f64, audio: &mut WebAudioPlayer) {
        self.remove_invisible_items();
        for tile in &self.tile_map {
            tile.borrow_mut().update(now, &self);
        }
        for object in &self.objects {
            object.borrow_mut().update(now, &self);
        }
        for character in &self.characters {
            character.borrow_mut().update(now, &self, audio);
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
            if player.movement_manager().status == Status::Idle {
                player.walk(dir, &self);
            }
        }
    }

    pub fn get_tile_by_position(&self, position: &Position) -> &RefCell<Tile> {
        let idx = self.get_index(position.row() as usize, position.col() as usize);
        &self.tile_map[idx]
    }
}