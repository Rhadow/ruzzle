// use web_sys::console::log_1;
use crate::game::{Asset, Direction, StatusManager, Status, World};
use crate::game::character::Player;
use crate::audio::{SFX, AudioPlayer};
use crate::utils::check_collision;
use crate::game::constants::MAX_BURNING_LEVEL;

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
    fn interact(&mut self, _player: &mut Player) {}
    fn handle_fall(&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.attribute_manager().is_visible = false;
        audio.play_sfx(SFX::RockFall);
    }
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
    fn handle_ignite(&mut self) {
        let is_burning = self.attribute_manager().burning_level > 0;
        let is_burnable = self.attribute_manager().is_burnable;
        let ignite_timer = self.status_manager().ignite_timer;
        let ignite_time = self.attribute_manager().ignite_time;
        if is_burnable && !is_burning {
            // Ignite timer are calculated through frames instead of millisecond, think of it as temperature
            self.status_manager().ignite_timer = ignite_timer + 1f64;
            if self.status_manager().ignite_timer >= ignite_time {
                self.attribute_manager().burning_level = 1;
                self.status_manager().burning_timer = 0f64;
            }
        }
    }
    fn handle_fire_logic(&mut self, world: &World) {
        let dt_per_burning_level = self.attribute_manager().burn_down_time / MAX_BURNING_LEVEL as f64;
        for object in world.get_objects() {
            if object.try_borrow_mut().is_err() {
                continue;
            }
            let mut object = object.borrow_mut();
            let is_object_visible = object.attribute_manager().is_visible;
            if is_object_visible {
                let is_collided = check_collision(object.status_manager(), &self.status_manager());
                if is_collided {
                    object.handle_ignite();
                }
            }
        }
        if self.status_manager().burning_timer > dt_per_burning_level {
            self.attribute_manager().burning_level += 1;
            self.status_manager().burning_timer = 0f64;
            if self.attribute_manager().burning_level > MAX_BURNING_LEVEL {
                self.attribute_manager().is_visible = false;
            }
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
    pub ignite_time: f64,
}
