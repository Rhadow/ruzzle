use crate::game::{Asset, Direction, MovementManager, Status, World};
use crate::audio::AudioPlayer;
pub mod tree;
pub mod rock;
pub use self::tree::Tree;
pub use self::rock::Rock;

pub trait Object {
    fn asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn is_visible(&self) -> bool;
    fn set_visible(&mut self, _visible: bool);
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut AudioPlayer) {}
    fn walk(&mut self, _direction: Direction, _world: &World) {}
    fn can_move_to(&self, direction: &Direction, world: &World) -> bool {
        let next_position = self.movement_manager().get_next_position_by_direction(&direction);
        if !next_position.is_in_tile_map() {
            return false;
        }
        if self.movement_manager().status != Status::Idle {
            return false;
        }
        let object = world.get_object_by_position(&next_position);
        let mut has_object = false;
        let mut can_step_on = false;
        if let Some(object) = object {
            has_object = true;
            can_step_on = object.borrow().can_step_on();
        }
        !has_object || can_step_on
    }
    fn can_step_on(&self) -> bool {
        false
    }
    fn is_pushable(&self) -> bool{
        false
    }
}

