// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::{
    CANNON_MOVE_TIME,
    CANNON_WIDTH,
    CANNON_HEIGHT,
    CANNON_UP_X_OFFSET,
    CANNON_ROTATION_ANIMATION_TIME,
    CANNON_ROTATION_ANIMATION_SPRITE_LENGTH,
};

pub struct Projector {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
    projection_timer: f64,
    projection_cycle_time: f64,
}

impl Object for Projector {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        if self.status_manager.time != 0f64 {
            self.projection_timer += now - self.status_manager.time;
        }
        self.status_manager.update_time(now);
        self.handle_projection(audio);
        self.update_rotation_animation();
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Projector {
    pub fn new(position: Position, direction: Direction, asset: Asset, id: String, projection_cycle_time: f64) -> Projector {
        let status_manager = StatusManager::new(position, direction, CANNON_WIDTH, CANNON_HEIGHT);
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
            burning_point: 0f64,
            temperature: 0f64,
            heat: 1f64,
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
        self.status_manager.delta_time = 0f64;
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
    fn update_rotation_animation(&mut self) {
        let direction = self.status_manager().direction;
        let target_x_offset = self.asset.get_x_offset_by_direction(direction);
        let current_x_offset = self.asset.get_x_offset();
        if current_x_offset != target_x_offset {
            if self.status_manager.animation_timer >= CANNON_ROTATION_ANIMATION_TIME {
                self.asset.set_x_offset(target_x_offset);
                self.status_manager.animation_timer = 0f64;
            } else {
                let x_offset = if target_x_offset - 2f64 >= 0f64 {
                    target_x_offset - 2f64
                 } else {
                    CANNON_UP_X_OFFSET + (CANNON_ROTATION_ANIMATION_SPRITE_LENGTH * 2) as f64 - 2f64
                 };
                self.asset.set_x_offset(x_offset);
            }
        } else {
            self.status_manager.animation_timer = 0f64;
        }
    }
    fn handle_projection(&mut self, audio: &mut Box<dyn AudioPlayer>) {
        if self.projection_timer >= self.projection_cycle_time {
            self.attribute_manager.is_projecting = true;
            self.projection_timer = 0f64;
            audio.play_sfx(SFX::Projecting);
        }
    }
}