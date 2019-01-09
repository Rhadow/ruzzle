use crate::game::Asset;

pub mod grassland;
pub mod wooden_path;

pub use self::grassland::GrassLand;
pub use self::wooden_path::WoodenPath;

pub trait Terrain: Asset {
    fn update(&self) {}
}
