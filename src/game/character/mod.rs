use crate::game::{Asset, Coordinate};
mod player;
pub use self::player::Player;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Character {
    fn get_asset(&self) -> &Asset;
    fn get_coordinate(&self) -> &Coordinate;
    fn set_direction(&mut self, direction: Direction);
    fn update(&mut self);
}