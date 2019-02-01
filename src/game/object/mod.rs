use crate::game::{Asset, Direction, StatusManager, Status, World};
use crate::game::character::Player;
use crate::audio::{SFX, AudioPlayer};
pub mod tree;
pub mod rock;
pub mod exit;
pub mod projector;
pub mod projectile;
pub mod fire_source;
pub mod wall;
pub mod breakable_wall;
pub mod spawning_point;
pub use self::tree::Tree;
pub use self::rock::Rock;
pub use self::exit::Exit;
pub use self::projector::Projector;
pub use self::projectile::Projectile;
pub use self::fire_source::FireSource;
pub use self::wall::Wall;
pub use self::breakable_wall::BreakableWall;
pub use self::spawning_point::SpawningPoint;

pub trait Object {
    fn asset(&mut self) -> &mut Asset;
    fn status_manager(&mut self) -> &mut StatusManager;
    fn attribute_manager(&mut self) -> &mut AttributeManager;
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut Box<dyn AudioPlayer>) {}
    fn handle_fall(&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.attribute_manager().is_visible = false;
        audio.play_sfx(SFX::RockFall);
    }
    fn interact(&mut self, _player: &mut Player) {}
    fn walk(&mut self, direction: Direction, _world: &World) {
        let next_position = self.status_manager().get_next_position_by_direction(&direction);
        self.status_manager().walk_to(next_position);
    }
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
    fn rotate(&mut self) {
        match self.status_manager().direction {
            Direction::Up => {
                self.status_manager().direction = Direction::Right;
            },
            Direction::Right => {
                self.status_manager().direction = Direction::Down;
            },
            Direction::Down => {
                self.status_manager().direction = Direction::Left;
            },
            Direction::Left => {
                self.status_manager().direction = Direction::Up;
            },
        }
    }
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
