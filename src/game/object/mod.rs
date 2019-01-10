use crate::game::{Asset, Coordinate};
pub mod tree;
pub use self::tree::Tree;

pub trait Object {
    fn get_asset(&self) -> &Asset;
    fn get_coordinate(&self) -> &Coordinate;
    fn update(&self);
}

