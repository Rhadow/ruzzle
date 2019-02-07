/*
    Environment:
    "G1" : GrassLand1,
    "G2" : GrassLand2,
    "G3" : GrassLand3,
    "G4" : GrassLand4,
    "WP": WoodenPath,
    "H" : Hole,

    Objects:
    "T": Tree,
    "R": Rock,
    "C": Chest (Exit),
    "W": Wall,
    "DS": Slow cannon facing down,
    "US": Slow cannon facing up,
    "LS": Slow cannon facing left,
    "RS": Slow cannon facing right,
    "DF": Fast cannon facing down,
    "UF": Fast cannon facing up,
    "LF": Fast cannon facing left,
    "RF": Fast cannon facing right,
    "BF": Burning Fire Source,
    "NF": Non-Burning Fire Source,
    "BW": BreakableWall,
    "SP": Spawning point,
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
    SpawningPoint,
};
use crate::game::constants::{
    WORLD_WIDTH_IN_TILES,
    TOTAL_TILES,
    // Terrain
    GRASS_LAND_ONE_X_OFFSET,
    GRASS_LAND_ONE_Y_OFFSET,
    GRASS_LAND_TWO_X_OFFSET,
    GRASS_LAND_TWO_Y_OFFSET,
    GRASS_LAND_THREE_X_OFFSET,
    GRASS_LAND_THREE_Y_OFFSET,
    GRASS_LAND_FOUR_X_OFFSET,
    GRASS_LAND_FOUR_Y_OFFSET,
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
    CANNON_WIDTH,
    CANNON_HEIGHT,
    SLOW_CANNON_PROJECT_CYCLE,
    FAST_CANNON_PROJECT_CYCLE,
    FIRE_SOURCE_OFF_X_OFFSET,
    FIRE_SOURCE_OFF_Y_OFFSET,
    FIRE_SOURCE_ON_X_OFFSET,
    FIRE_SOURCE_ON_Y_OFFSET,
    FIRE_SOURCE_SIZE,
    WALL_X_OFFSET,
    WALL_Y_OFFSET,
    WALL_WIDTH,
    WALL_HEIGHT,
    BREAKABLE_WALL_X_OFFSET,
    BREAKABLE_WALL_Y_OFFSET,
    BREAKABLE_WALL_WIDTH,
    BREAKABLE_WALL_HEIGHT,
    SPAWNING_POINT_X_OFFSET,
    SPAWNING_POINT_Y_OFFSET,
    SPAWNING_POINT_WIDTH,
    SPAWNING_POINT_HEIGHT,
};
use super::level00::LEVEL00;
use super::level01::LEVEL01;
use super::level03::LEVEL03;
use super::level04::LEVEL04;
use super::level06::LEVEL06;

pub type Level = ([&'static str; TOTAL_TILES], [&'static str; TOTAL_TILES], Position);
pub const LEVELS: [Level; 10] = [
    LEVEL00,
    LEVEL01,
    LEVEL00,
    LEVEL03,
    LEVEL04,
    LEVEL00,
    LEVEL06,
    LEVEL00,
    LEVEL00,
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
                "G1" => self.create_grass_land(position, 1),
                "G2" => self.create_grass_land(position, 2),
                "G3" => self.create_grass_land(position, 3),
                "G4" => self.create_grass_land(position, 4),
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
                "DS" => self.create_cannon(position, Direction::Down, uuid(), SLOW_CANNON_PROJECT_CYCLE),
                "US" => self.create_cannon(position, Direction::Up, uuid(), SLOW_CANNON_PROJECT_CYCLE),
                "LS" => self.create_cannon(position, Direction::Left, uuid(), SLOW_CANNON_PROJECT_CYCLE),
                "RS" => self.create_cannon(position, Direction::Right, uuid(), SLOW_CANNON_PROJECT_CYCLE),
                "DF" => self.create_cannon(position, Direction::Down, uuid(), FAST_CANNON_PROJECT_CYCLE),
                "UF" => self.create_cannon(position, Direction::Up, uuid(), FAST_CANNON_PROJECT_CYCLE),
                "LF" => self.create_cannon(position, Direction::Left, uuid(), FAST_CANNON_PROJECT_CYCLE),
                "RF" => self.create_cannon(position, Direction::Right, uuid(), FAST_CANNON_PROJECT_CYCLE),
                "BF" => self.create_fire_source(position, uuid(), 1),
                "NF" => self.create_fire_source(position, uuid(), 0),
                "SP" => self.create_spawning_point(position, uuid()),
                _ => None
            };

            if let Some(object) = object {
                all_objects.push(object);
            }
            tiles.push(Tile::new(terrain));
        }
        (tiles, all_objects)
    }

    fn create_grass_land(&self, position: Position, land_type: i8) -> Option<RefCell<Box<dyn Terrain>>> {
        let offsets = match land_type {
            1 => Some((GRASS_LAND_ONE_X_OFFSET, GRASS_LAND_ONE_Y_OFFSET)),
            2 => Some((GRASS_LAND_TWO_X_OFFSET, GRASS_LAND_TWO_Y_OFFSET)),
            3 => Some((GRASS_LAND_THREE_X_OFFSET, GRASS_LAND_THREE_Y_OFFSET)),
            4 => Some((GRASS_LAND_FOUR_X_OFFSET, GRASS_LAND_FOUR_Y_OFFSET)),
            _ => None
        };
        if let Some((x_offset, y_offset)) = offsets {
            let asset = Asset::new(
                AssetType::RuzzleEnvironment,
                x_offset,
                y_offset,
                GRASS_LAND_SIZE,
                GRASS_LAND_SIZE,
                None,
                None,
            );
            Some(RefCell::new(Box::new(Land::new(position, asset))))
        } else {
            None
        }
    }

    fn create_wooden_path(&self, position: Position) -> Option<RefCell<Box<dyn Terrain>>> {
        let asset = Asset::new(
            AssetType::Environment,
            WOODEN_PATH_X_OFFSET,
            WOODEN_PATH_Y_OFFSET,
            WOODEN_PATH_SIZE,
            WOODEN_PATH_SIZE,
            None,
            None,
        );
        Some(RefCell::new(Box::new(Land::new(position, asset))))
    }

    fn create_hole(&self, position: Position) -> Option<RefCell<Box<dyn Terrain>>> {
        let asset = Asset::new(
            AssetType::RuzzleEnvironment,
            HOLE_X_OFFSET,
            HOLE_Y_OFFSET,
            HOLE_SIZE,
            HOLE_SIZE,
            None,
            None,
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
            None,
            None,
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
            None,
            None,
        );
        Some(RefCell::new(Box::new(Rock::new(position, asset, id))))
    }

    fn create_chest(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            CHEST_X_OFFSET,
            CHEST_Y_OFFSET,
            CHEST_SIZE,
            CHEST_SIZE,
            None,
            None,
        );
        Some(RefCell::new(Box::new(Exit::new(position, asset, id))))
    }

    fn create_cannon(&self, position: Position, direction: Direction, id: String, projection_cycle_time: f64) -> Option<RefCell<Box<dyn Object>>> {
        let (x, y) = match direction {
            Direction::Down => (
                CANNON_DOWN_X_OFFSET,
                CANNON_DOWN_Y_OFFSET,
            ),
            Direction::Up => (
                CANNON_UP_X_OFFSET,
                CANNON_UP_Y_OFFSET,
            ),
            Direction::Right => (
                CANNON_RIGHT_X_OFFSET,
                CANNON_RIGHT_Y_OFFSET,
            ),
            Direction::Left => (
                CANNON_LEFT_X_OFFSET,
                CANNON_LEFT_Y_OFFSET,
            ),
        };
        let asset = Asset::new(
            AssetType::RuzzleObject,
            x,
            y,
            CANNON_WIDTH,
            CANNON_HEIGHT,
            Some((CANNON_UP_X_OFFSET, CANNON_RIGHT_X_OFFSET, CANNON_DOWN_X_OFFSET, CANNON_LEFT_X_OFFSET)),
            Some((CANNON_UP_Y_OFFSET, CANNON_RIGHT_Y_OFFSET, CANNON_DOWN_Y_OFFSET, CANNON_LEFT_Y_OFFSET)),
        );
        Some(RefCell::new(Box::new(Projector::new(position, direction, asset, id, projection_cycle_time))))
    }
    fn create_fire_source(&self, position: Position, id: String, burning_level: isize) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            if burning_level > 0 { FIRE_SOURCE_ON_X_OFFSET } else {FIRE_SOURCE_OFF_X_OFFSET},
            if burning_level > 0 { FIRE_SOURCE_ON_Y_OFFSET } else {FIRE_SOURCE_OFF_Y_OFFSET},
            FIRE_SOURCE_SIZE,
            FIRE_SOURCE_SIZE,
            None,
            None,
        );
        Some(RefCell::new(Box::new(FireSource::new(position, asset, id, burning_level))))
    }
    fn create_wall(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            WALL_X_OFFSET,
            WALL_Y_OFFSET,
            WALL_WIDTH,
            WALL_HEIGHT,
            None,
            None,
        );
        Some(RefCell::new(Box::new(Wall::new(position, asset, id))))
    }
    fn create_breakable_wall(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            BREAKABLE_WALL_X_OFFSET,
            BREAKABLE_WALL_Y_OFFSET,
            BREAKABLE_WALL_WIDTH,
            BREAKABLE_WALL_HEIGHT,
            None,
            None,
        );
        Some(RefCell::new(Box::new(BreakableWall::new(position, asset, id))))
    }
    fn create_spawning_point(&self, position: Position, id: String) -> Option<RefCell<Box<dyn Object>>> {
        let asset = Asset::new(
            AssetType::RuzzleObject,
            SPAWNING_POINT_X_OFFSET,
            SPAWNING_POINT_Y_OFFSET,
            SPAWNING_POINT_WIDTH,
            SPAWNING_POINT_HEIGHT,
            None,
            None,
        );
        Some(RefCell::new(Box::new(SpawningPoint::new(position, asset, id))))
    }
}