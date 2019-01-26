// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::{
    TILE_SIZE,
    ROCK_MOVE_TIME,
};

pub struct Rock {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
}

impl Object for Rock {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn walk(&mut self, direction: Direction, world: &World) {
        let next_position = self.status_manager.get_next_position_by_direction(&direction);
        self.status_manager.walk_to(next_position);
        let tile = world.get_tile_by_position(&next_position).borrow_mut();
        let terrain = tile.terrain();
        if let Some(terrain) = terrain {
            let mut terrain = terrain.borrow_mut();
            if !terrain.is_filled() {
                terrain.set_falling_schedule(ROCK_MOVE_TIME);
            }
        }
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Rock {
    pub fn new(position: Position, asset: Asset, id: String) -> Rock {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: false,
            is_visible: true,
            is_pushable: true,
            is_filler: true,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: false,
            is_breakable: false,
            burning_level: 0,
            burn_down_time: 0f64,
        };
        Rock {
            asset,
            status_manager,
            attribute_manager,
        }
    }
    fn animate_idle (&mut self) {
        self.status_manager.delta_time = 0f64;
    }
    fn animate_walking (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        let delta_time = self.status_manager.delta_time;
        self.status_manager.set_next_coordinate(delta_time, ROCK_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.set_status(Status::Idle);
        }
    }
}