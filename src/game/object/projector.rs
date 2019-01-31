// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::{
    CANNON_MOVE_TIME,
    CANNON_UP_X_OFFSET,
    CANNON_DOWN_X_OFFSET,
    CANNON_LEFT_X_OFFSET,
    CANNON_RIGHT_X_OFFSET,
    CANNON_VERTICAL_WIDTH,
    CANNON_VERTICAL_HEIGHT,
    CANNON_HORIZONTAL_WIDTH,
    CANNON_HORIZONTAL_HEIGHT,
    CANNON_ROTATION_ANIMATION_TIME,
};

pub struct Projector {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
    projection_timer: f64,
    projection_cycle_time: f64,
}

impl Object for Projector {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn rotate(&mut self) {
        match self.status_manager.direction {
            Direction::Up => {
                self.asset.set_x_offset(CANNON_UP_X_OFFSET + 2f64);
                self.status_manager.direction = Direction::Right;
            },
            Direction::Right => {
                self.asset.set_x_offset(CANNON_RIGHT_X_OFFSET + 2f64);
                self.status_manager.direction = Direction::Down;
            },
            Direction::Down => {
                self.asset.set_x_offset(CANNON_DOWN_X_OFFSET + 2f64);
                self.status_manager.direction = Direction::Left;
            },
            Direction::Left => {
                self.asset.set_x_offset(CANNON_LEFT_X_OFFSET + 2f64);
                self.status_manager.direction = Direction::Up;
            },
        }
    }
    fn walk(&mut self, direction: Direction, world: &World) {
        let next_position = self.status_manager.get_next_position_by_direction(&direction);
        self.status_manager.walk_to(next_position);
        let tile = world.get_tile_by_position(&next_position).borrow_mut();
        let terrain = tile.terrain();
        if let Some(terrain) = terrain {
            let mut terrain = terrain.borrow_mut();
            if !terrain.is_filled() {
                terrain.set_falling_schedule(CANNON_MOVE_TIME);
            }
        }
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        if self.status_manager.time != 0f64 {
            self.status_manager.delta_time += now - self.status_manager.time;
            self.projection_timer += now - self.status_manager.time;
        }
        self.status_manager.time = now;
        if self.projection_timer >= self.projection_cycle_time {
            self.attribute_manager.is_projecting = true;
            self.projection_timer = 0f64;
            audio.play_sfx(SFX::Projecting);
        }
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Projector {
    pub fn new(position: Position, direction: Direction, asset: Asset, id: String, projection_cycle_time: f64) -> Projector {
        let width = match direction {
            Direction::Up | Direction::Down => CANNON_VERTICAL_WIDTH,
            Direction::Left | Direction::Right => CANNON_HORIZONTAL_WIDTH,
        };
        let height = match direction {
            Direction::Up | Direction::Down => CANNON_VERTICAL_HEIGHT,
            Direction::Left | Direction::Right => CANNON_HORIZONTAL_HEIGHT,
        };
        let status_manager = StatusManager::new(position, direction, width * 2f64, height * 2f64);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: false,
            is_visible: true,
            is_pushable: true,
            is_filler: false,
            is_rotatable: true,
            is_projectile: false,
            is_projecting: false,
            is_burnable: false,
            is_breakable: false,
            burning_level: 0,
            burn_down_time: 0f64,
        };
        Projector {
            asset,
            status_manager,
            attribute_manager,
            projection_timer: 0f64,
            projection_cycle_time,
        }
    }
    fn animate_idle (&mut self) {
        let asset_x_offset = match self.status_manager.direction {
            Direction::Up => CANNON_UP_X_OFFSET,
            Direction::Right => CANNON_RIGHT_X_OFFSET,
            Direction::Down => CANNON_DOWN_X_OFFSET,
            Direction::Left => CANNON_LEFT_X_OFFSET,
        };
        if self.asset.get_x_offset() != asset_x_offset {
            if self.status_manager.delta_time >= CANNON_ROTATION_ANIMATION_TIME {
                self.asset.set_x_offset(asset_x_offset);
                self.status_manager.delta_time = 0f64;
            }
        } else {
            self.status_manager.delta_time = 0f64;
        }
    }
    fn animate_walking (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        let delta_time = self.status_manager.delta_time;
        self.status_manager.set_next_coordinate(delta_time, CANNON_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        // self.projection_timer = 0f64;
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.set_status(Status::Idle);
        }
    }
}