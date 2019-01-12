use crate::game::{Asset, Direction, MovementManager, World};
pub mod tree;
pub use self::tree::Tree;

pub trait Object {
    fn asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn update(&mut self, now: f64);
    fn step(&mut self, direction: Direction, world: &World);
    fn is_walkable(&self) -> bool;
}

