mod world;
mod cell;

pub mod level;
pub mod terrain;
pub mod constants;
pub mod object;

pub use self::world::World;
pub use self::cell::Cell;

pub enum AssetType {
    Environment,
    Character,
    Object,
}


pub trait Asset {
    fn get_asset_type(&self) -> &AssetType;
    fn get_asset_x_offset(&self) -> f64;
    fn get_asset_y_offset(&self) -> f64;
    fn get_asset_width(&self) -> f64;
    fn get_asset_height(&self) -> f64;
}