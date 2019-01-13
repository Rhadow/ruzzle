/*
    Environment:
    "G": GrassLand,
    "WP": WoodenPath,

    Objects:
    "T": Tree
*/
use std::cell::RefCell;
use crate::game::{
    Tile,
    Position
};
use crate::game::terrain::{
    Terrain,
    GrassLand,
    WoodenPath,
    Hole,
};
use crate::game::object::{
    Object,
    Tree,
    Rock,
};
use crate::game::constants::{
    WORLD_WIDTH_IN_TILES,
    TOTAL_TILES,
};
use super::level00::LEVEL00;

type Level = ([&'static str; TOTAL_TILES], [&'static str; TOTAL_TILES], Position);
const LEVELS: [Level; 1] = [
    LEVEL00,
];

pub struct LevelManager {
    player_position: Option<Position>
}

impl LevelManager {
    pub fn new() -> LevelManager {
        LevelManager {
            player_position: None
        }
    }

    fn index_to_position(&self, idx: usize) -> Position {
        let row = (idx as i32 / WORLD_WIDTH_IN_TILES as i32) as f64;
        let col = (idx as i32 % WORLD_WIDTH_IN_TILES as i32) as f64;
        Position(row, col)
    }

    pub fn get_player_position(&self) -> &Option<Position> {
        &self.player_position
    }

    pub fn set_player_position(&mut self, new_position: Option<Position>) {
        self.player_position = new_position;
    }

    pub fn construct_level(&mut self, level: usize) -> (Vec<Tile>, Vec<RefCell<Box<dyn Object>>>) {
        let mut tiles = vec![];
        let mut all_objects = vec![];
        let (terrains, objects, player_position) = LEVELS[level];
        self.set_player_position(Some(player_position));
        for i in 0..TOTAL_TILES {
            let terrain = terrains[i];
            let object = objects[i];
            let position = self.index_to_position(i);

            let terrain: Option<RefCell<Box<dyn Terrain>>> = match String::from(terrain.trim()).to_uppercase().as_str() {
                "G" => Some(RefCell::new(Box::new(GrassLand::new(position)))),
                "WP" => Some(RefCell::new(Box::new(WoodenPath::new(position)))),
                "H" => Some(RefCell::new(Box::new(Hole::new(position)))),
                _ => None
            };

            let object: Option<RefCell<Box<dyn Object>>> = match String::from(object.trim()).to_uppercase().as_str() {
                "T" => Some(RefCell::new(Box::new(Tree::new(position)))),
                "R" => Some(RefCell::new(Box::new(Rock::new(position)))),
                _ => None
            };

            if let Some(object) = object {
                all_objects.push(object);
            }
            tiles.push(Tile::new(terrain));
        }
        (tiles, all_objects)
    }
}