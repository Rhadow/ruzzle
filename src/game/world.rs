// use web_sys::console::log_1;
// log_1(&format!("{}", self.objects.len()).into());
use std::cell::RefCell;
use crate::audio::AudioPlayer;
use super::Tile;
use super::character::{Character, Player};
use super::object::Object;
use crate::game::level::LevelManager;
use super::status_manager::{
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
    ACTION_KEY,
};

pub struct World {
    tile_map: Vec<RefCell<Tile>>,
    objects: Vec<RefCell<Box<dyn Object>>>,
    characters: Vec<RefCell<Box<dyn Character>>>,
    pub is_completed: bool,
    pub level_number: Option<usize>,
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
            is_completed: false,
            level_number: None,
        }
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        WORLD_WIDTH_IN_TILES * row + col
    }

    pub fn tile_map(&self) -> &Vec<RefCell<Tile>> {
        &self.tile_map
    }

    pub fn set_tile_map(&mut self, tile_map: Vec<Tile>) {
        let mut new_tile_map = vec![];
        for tile in tile_map {
            new_tile_map.push(RefCell::new(tile));
        }
        self.tile_map = new_tile_map;
    }

    pub fn get_object_by_position(&self, position: &Position) -> Option<&RefCell<Box<dyn Object>>> {
        let result = None;
        for object in &self.objects {
            if object.try_borrow().is_ok() {
                if object.borrow().is_visible() {
                    let object_position = object.borrow().status_manager().position;
                    if object_position.row() == position.row() && object_position.col() == position.col() {
                        return Some(object);
                    }
                }
            }
        }
        return result;
    }

    pub fn player(&self) -> &RefCell<Box<dyn Character>> {
        let idx = self.characters.len() - 1;
        &self.characters[idx]
    }

    pub fn get_characters(&self) -> &Vec<RefCell<Box<dyn Character>>> {
        &self.characters
    }

    pub fn set_characters(&mut self, characters: Vec<Box<dyn Character>>) {
        let mut new_characters = vec![];
        for character in characters {
            new_characters.push(RefCell::new(character));
        }
        self.characters = new_characters;
    }

    pub fn get_objects(&self) -> &Vec<RefCell<Box<dyn Object>>> {
        &self.objects
    }

    pub fn set_objects(&mut self, objects: Vec<RefCell<Box<dyn Object>>>) {
        self.objects = objects;
    }

    fn remove_invisible_items(&mut self) {
        self.objects.iter()
            .position(|o| !o.borrow().is_visible())
            .map(|idx| self.objects.remove(idx));
    }

    pub fn update(&mut self, now: f64, audio: &mut Box<dyn AudioPlayer>) {
        self.remove_invisible_items();
        for tile in &self.tile_map {
            tile.borrow_mut().update(now, self, audio);
        }
        for object in &self.objects {
            object.borrow_mut().update(now, self, audio);
        }
        for character in &self.characters {
            character.borrow_mut().update(now, self, audio);
        }
        if self.player().borrow().status_manager().status == Status::LevelComplete {
            self.is_completed = true;
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
            if player.status_manager().status == Status::Idle && !player.at_exit() {
                player.walk(dir, &self);
            }
        }
    }

    pub fn handle_action_event(&mut self, key: &str) {
        let mut player = self.player().borrow_mut();
        match key {
            ACTION_KEY => player.rotate_item(self),
            _ => (),
        }
    }

    pub fn get_tile_by_position(&self, position: &Position) -> &RefCell<Tile> {
        let idx = self.get_index(position.row() as usize, position.col() as usize);
        &self.tile_map[idx]
    }

    pub fn init_level(&mut self, level: usize) {
        let mut level_manager = LevelManager::new();
        let (level_cells, objects) = level_manager.construct_level(level);
        let player_position = level_manager.get_player_position().unwrap();
        let player = Box::new(Player::new(player_position, Direction::Down, 0f64)) as Box<dyn Character>;
        self.set_tile_map(level_cells);
        self.set_objects(objects);
        self.set_characters(vec![player]);
        self.is_completed = false;
        self.level_number = Some(level);
    }
}