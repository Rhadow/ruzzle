mod world;
mod cell;
mod asset;

pub mod canvas;
pub mod level;
pub mod terrain;
pub mod constants;
pub mod object;
pub mod character;

pub use self::world::World;
pub use self::cell::Cell;
pub use self::asset::{Asset, AssetType};

#[derive(Clone, Copy)]
pub struct Coordinate (usize, usize);

impl Coordinate {
    pub fn row(&self) -> usize {
        self.0
    }
    pub fn col(&self) -> usize {
        self.1
    }
}