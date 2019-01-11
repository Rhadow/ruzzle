use crate::game::{Asset, MovementManager, Direction};
mod player;
pub use self::player::Player;

pub trait Character {
    fn asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn move_by_direction(&mut self, direction: Direction);
    fn update(&mut self, now: f64);
}