mod world;
mod tile;
mod asset;

pub mod level;
pub mod terrain;
pub mod constants;
pub mod object;
pub mod character;
pub mod movement_manager;

pub use self::world::World;
pub use self::tile::Tile;
pub use self::asset::{Asset, AssetType};
pub use self::movement_manager::{MovementManager, Direction, Status};

use self::constants::{
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
};

#[derive(Clone, Copy, PartialEq)]
pub struct Coordinate (pub f64, pub f64);

impl Coordinate {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position (f64, f64);

impl Position {
    pub fn row(&self) -> f64 {
        self.0
    }
    pub fn col(&self) -> f64 {
        self.1
    }
    pub fn is_in_tile_map(&self) -> bool {
        self.0 >= 0f64 && self.0 < (WORLD_HEIGHT_IN_TILES as f64) && self.1 >= 0f64 && self.1 < (WORLD_WIDTH_IN_TILES as f64)
    }
}
