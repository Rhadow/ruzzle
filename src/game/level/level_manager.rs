/*
    Environment:
    "G" : GrassLand,
    "WP": WoodenPath,
    "H" : Hole,

    Objects:
    "T": Tree,
    "R": Rock,
    "C": Chest (Exit),
*/
use std::cell::RefCell;
use crate::game::{
    Asset,
    AssetType,
    Tile,
    Position
};
use crate::game::terrain::{
    Terrain,
    Land,
    Hole,
};
use crate::game::object::{
    Object,
    Wall,
    Rock,
    Exit,
};
use crate::game::constants::{
    WORLD_WIDTH_IN_TILES,
    TOTAL_TILES,
    // Terrain
    GRASS_LAND_X_OFFSET,
    GRASS_LAND_Y_OFFSET,
    GRASS_LAND_SIZE,
    WOODEN_PATH_X_OFFSET,
    WOODEN_PATH_Y_OFFSET,
    WOODEN_PATH_SIZE,
    HOLE_X_OFFSET,
    HOLE_Y_OFFSET,
    HOLE_SIZE,
    // Object
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE,
    ROCK_X_OFFSET,
    ROCK_Y_OFFSET,
    ROCK_SIZE,
    CHEST_X_OFFSET,
    CHEST_Y_OFFSET,
    CHEST_SIZE,
};
use super::level00::LEVEL00;
use super::level01::LEVEL01;

pub type Level = ([&'static str; TOTAL_TILES], [&'static str; TOTAL_TILES], Position);
pub const LEVELS: [Level; 10] = [
    LEVEL00,
    LEVEL01,
    LEVEL00,
    LEVEL01,
    LEVEL00,
    LEVEL01,
    LEVEL00,
    LEVEL01,
    LEVEL00,
    LEVEL01,
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
                "G" => self.create_grass_land(position),
                "WP" => self.create_wooden_path(position),
                "H" => self.create_hole(position),
                _ => None
            };

            let object: Option<RefCell<Box<dyn Object>>> = match String::from(object.trim()).to_uppercase().as_str() {
                "T" => self.create_tree(position),
                "R" => self.create_rock(position),
                "C" => self.create_chest(position),
                _ => None
            };

            if let Some(object) = object {
                all_objects.push(object);
            }
            tiles.push(Tile::new(terrain));
        }
        (tiles, all_objects)
    }

    fn create_grass_land(&self, position: Position) -> Option<RefCell<Box<dyn Terrain>>> {
        let asset = Asset::new(
            AssetType::Environment,
            GRASS_LAND_X_OFFSET,
            GRASS_LAND_Y_OFFSET,
            GRASS_LAND_SIZE,
            GRASS_LAND_SIZE,
        );
        Some(RefCell::new(Box::new(Land::new(position, asset))))
    }

    fn create_wooden_path(&self, position: Position) -> Option<RefCell<Box<dyn Terrain>>> {
        let asset = Asset::new(
            AssetType::Environment,
            WOODEN_PATH_X_OFFSET,
            WOODEN_PATH_Y_OFFSET,
            WOODEN_PATH_SIZE,
            WOODEN_PATH_SIZE,
        );
        Some(RefCell::new(Box::new(Land::new(position, asset))))
    }

    fn create_hole(&self, position: Position) -> Option<RefCell<Box<dyn Terrain>>> {
        let asset = Asset::new(
            AssetType::Environment,
            HOLE_X_OFFSET,
            HOLE_Y_OFFSET,
            HOLE_SIZE,
            HOLE_SIZE,
        );
        Some(RefCell::new(Box::new(Hole::new(position, asset))))
    }

    fn create_tree(&self, position: Position) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Object,
            TREE_X_OFFSET,
            TREE_Y_OFFSET,
            TREE_SIZE,
            TREE_SIZE,
        );
        Some(RefCell::new(Box::new(Wall::new(position, asset))))
    }

    fn create_rock(&self, position: Position) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Environment,
            ROCK_X_OFFSET,
            ROCK_Y_OFFSET,
            ROCK_SIZE,
            ROCK_SIZE,
        );
        Some(RefCell::new(Box::new(Rock::new(position, asset))))
    }

    fn create_chest(&self, position: Position) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Object,
            CHEST_X_OFFSET,
            CHEST_Y_OFFSET,
            CHEST_SIZE,
            CHEST_SIZE,
        );
        Some(RefCell::new(Box::new(Exit::new(position, asset))))
    }
}