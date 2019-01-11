use crate::game::{Asset, Coordinate, Position};
pub mod tree;
pub use self::tree::Tree;

pub trait Object {
    fn get_asset(&self) -> &Asset;
    fn get_coordinate(&self) -> &Coordinate;
    fn get_position(&self) -> &Position;
    fn update(&self);
}

