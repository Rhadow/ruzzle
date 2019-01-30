/*
    Environment:
    "G" : GrassLand,
    "WP": WoodenPath,
    "H" : Hole,

    Objects:
    "T": Tree,
    "R": Rock,
    "C": Chest (Exit),
    "W": Wall,
    "DN": Cannon facing down,
    "UN": Cannon facing up,
    "LN": Cannon facing left,
    "RN": Cannon facing right,
    "FS": Fire Source,
    "BW": BreakableWall,
*/
use crate::utils::uuid;
use std::cell::RefCell;
use crate::game::{
    Asset,
    AssetType,
    Tile,
    Position,
    Direction,
};
use crate::game::terrain::{
    Terrain,
    Land,
    Hole,
};
use crate::game::object::{
    Object,
    Tree,
    Rock,
    Exit,
    Projector,
    FireSource,
    Wall,
    BreakableWall,
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
    CANNON_UP_X_OFFSET,
    CANNON_UP_Y_OFFSET,
    CANNON_DOWN_X_OFFSET,
    CANNON_DOWN_Y_OFFSET,
    CANNON_LEFT_X_OFFSET,
    CANNON_LEFT_Y_OFFSET,
    CANNON_RIGHT_X_OFFSET,
    CANNON_RIGHT_Y_OFFSET,
    CANNON_VERTICAL_WIDTH,
    CANNON_VERTICAL_HEIGHT,
    CANNON_HORIZONTAL_WIDTH,
    CANNON_HORIZONTAL_HEIGHT,
    FIRE_SOURCE_X_OFFSET,
    FIRE_SOURCE_Y_OFFSET,
    FIRE_SOURCE_SIZE,
    WALL_X_OFFSET,
    WALL_Y_OFFSET,
    WALL_WIDTH,
    WALL_HEIGHT,
    BREAKABLE_WALL_X_OFFSET,
    BREAKABLE_WALL_Y_OFFSET,
    BREAKABLE_WALL_WIDTH,
    BREAKABLE_WALL_HEIGHT,
};
use super::level00::LEVEL00;
use super::level01::LEVEL01;
use super::level02::LEVEL02;

pub type Level = ([&'static str; TOTAL_TILES], [&'static str; TOTAL_TILES], Position);
pub const LEVELS: [Level; 10] = [
    LEVEL00,
    LEVEL01,
    LEVEL02,
    LEVEL00,
    LEVEL01,
    LEVEL02,
    LEVEL00,
    LEVEL01,
    LEVEL02,
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
                "G" => self.create_grass_land(position),
                "WP" => self.create_wooden_path(position),
                "H" => self.create_hole(position),
                _ => None
            };

            let object: Option<RefCell<Box<dyn Object>>> = match String::from(object.trim()).to_uppercase().as_str() {
                "T" => self.create_tree(position, uuid()),
                "R" => self.create_rock(position, uuid()),
                "C" => self.create_chest(position, uuid()),
                "W" => self.create_wall(position, uuid()),
                "BW" => self.create_breakable_wall(position, uuid()),
                "DN" => self.create_cannon(position, Direction::Down, uuid()),
                "UN" => self.create_cannon(position, Direction::Up, uuid()),
                "LN" => self.create_cannon(position, Direction::Left, uuid()),
                "RN" => self.create_cannon(position, Direction::Right, uuid()),
                "FS" => self.create_fire_source(position, uuid()),
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

    fn create_tree(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            TREE_X_OFFSET,
            TREE_Y_OFFSET,
            TREE_SIZE,
            TREE_SIZE,
        );
        Some(RefCell::new(Box::new(Tree::new(position, asset, id))))
    }

    fn create_rock(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            ROCK_X_OFFSET,
            ROCK_Y_OFFSET,
            ROCK_SIZE,
            ROCK_SIZE,
        );
        Some(RefCell::new(Box::new(Rock::new(position, asset, id))))
    }

    fn create_chest(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Object,
            CHEST_X_OFFSET,
            CHEST_Y_OFFSET,
            CHEST_SIZE,
            CHEST_SIZE,
        );
        Some(RefCell::new(Box::new(Exit::new(position, asset, id))))
    }

    fn create_cannon(&self, position: Position, direction: Direction, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let (x, y, w, h) = match direction {
            Direction::Down => (
                CANNON_DOWN_X_OFFSET,
                CANNON_DOWN_Y_OFFSET,
                CANNON_VERTICAL_WIDTH,
                CANNON_VERTICAL_HEIGHT
            ),
            Direction::Up => (
                CANNON_UP_X_OFFSET,
                CANNON_UP_Y_OFFSET,
                CANNON_VERTICAL_WIDTH,
                CANNON_VERTICAL_HEIGHT
            ),
            Direction::Right => (
                CANNON_RIGHT_X_OFFSET,
                CANNON_RIGHT_Y_OFFSET,
                CANNON_HORIZONTAL_WIDTH,
                CANNON_HORIZONTAL_HEIGHT
            ),
            Direction::Left => (
                CANNON_LEFT_X_OFFSET,
                CANNON_LEFT_Y_OFFSET,
                CANNON_HORIZONTAL_WIDTH,
                CANNON_HORIZONTAL_HEIGHT
            ),
        };
        let asset = Asset::new(
            AssetType::RuzzleObject,
            x,
            y,
            w,
            h,
        );
        Some(RefCell::new(Box::new(Projector::new(position, direction, asset, id))))
    }
    fn create_fire_source(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Object,
            FIRE_SOURCE_X_OFFSET,
            FIRE_SOURCE_Y_OFFSET,
            FIRE_SOURCE_SIZE,
            FIRE_SOURCE_SIZE,
        );
        Some(RefCell::new(Box::new(FireSource::new(position, asset, id))))
    }
    fn create_wall(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Environment,
            WALL_X_OFFSET,
            WALL_Y_OFFSET,
            WALL_WIDTH,
            WALL_HEIGHT,
        );
        Some(RefCell::new(Box::new(Wall::new(position, asset, id))))
    }
    fn create_breakable_wall(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::Environment,
            BREAKABLE_WALL_X_OFFSET,
            BREAKABLE_WALL_Y_OFFSET,
            BREAKABLE_WALL_WIDTH,
            BREAKABLE_WALL_HEIGHT,
        );
        Some(RefCell::new(Box::new(BreakableWall::new(position, asset, id))))
    }
}