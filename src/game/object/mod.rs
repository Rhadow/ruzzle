use crate::game::{Asset, Direction, MovementManager, World};
pub mod tree;
pub mod rock;
pub use self::tree::Tree;
pub use self::rock::Rock;

pub trait Object {
    fn asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn update(&mut self, now: f64);
    fn step(&mut self, direction: Direction, world: &World);
    fn can_move_to(&self, _direction: &Direction, _world: &World) -> bool {
        false
    }
    fn is_walkable(&self) -> bool {
        false
    }
    fn is_pushable(&self) -> bool{
        false
    }
}

