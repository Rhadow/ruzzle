use crate::game::{Asset, Direction, StatusManager, Status, World};
use crate::game::character::Player;
use crate::audio::AudioPlayer;
pub mod tree;
pub mod rock;
pub mod exit;
pub mod projector;
pub mod projectile;
pub mod fire_source;
pub mod wall;
pub mod breakable_wall;
pub use self::tree::Tree;
pub use self::rock::Rock;
pub use self::exit::Exit;
pub use self::projector::Projector;
pub use self::projectile::Projectile;
pub use self::fire_source::FireSource;
pub use self::wall::Wall;
pub use self::breakable_wall::BreakableWall;

pub trait Object {
    fn asset(&self) -> &Asset;
    fn status_manager(&mut self) -> &mut StatusManager;
    fn attribute_manager(&mut self) -> &mut AttributeManager;
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut Box<dyn AudioPlayer>) {}
    fn walk(&mut self, _direction: Direction, _world: &World) {}
    fn interact(&mut self, _player: &mut Player) {}
    fn can_move_to(&mut self, direction: &Direction, world: &World) -> bool {
        let next_position = self.status_manager().get_next_position_by_direction(&direction);
        if !next_position.is_in_tile_map() {
            return false;
        }
        if self.status_manager().status != Status::Idle {
            return false;
        }
        let object = world.get_object_by_position(&next_position);
        let mut has_object = false;
        if let Some(_) = object {
            has_object = true;
        }
        !has_object
    }
    fn rotate(&mut self) {}
}

pub struct AttributeManager {
    pub id: String,
    pub can_step_on: bool,
    pub can_fly_through: bool,
    pub is_visible: bool,
    pub is_pushable: bool,
    pub is_filler: bool,
    pub is_rotatable: bool,
    pub is_projecting: bool,
    pub is_projectile: bool,
    pub is_burnable: bool,
    pub is_breakable: bool,
    pub burning_level: isize,
    pub burn_down_time: f64,
}
