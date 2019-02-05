use std::f64::INFINITY;
use super::{AttributeManager, Object};
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, Direction, Status, StatusManager, Position, World};
use crate::game::constants::{
    TILE_SIZE,
    FIRE_SOURCE_OFF_X_OFFSET,
    FIRE_SOURCE_ON_X_OFFSET,
    FIRE_ANIMATION_TIME,
    FIRE_ANIMATION_SPRITE_LENGTH,
    FIRE_SOURCE_MOVE_TIME,
    FIRE_SOURCE_IGNITE_FRAMES,
};

pub struct FireSource {
    asset: Asset,
    status_manager: StatusManager,
    pub attribute_manager: AttributeManager,
    animation_timer: f64,
}

impl Object for FireSource {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        if self.attribute_manager.burning_level > 0 {
            self.handle_fire_logic(world);
            self.animate_flame();
        } else {
            self.asset.set_x_offset(FIRE_SOURCE_OFF_X_OFFSET);
            self.status_manager.animation_timer = 0f64;
        }
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl FireSource {
    pub fn new(position: Position, asset: Asset, id: String, burning_level: isize) -> FireSource {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: true,
            is_visible: true,
            is_pushable: true,
            is_filler: false,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: true,
            is_breakable: false,
            burning_level,
            burn_down_time: INFINITY,
            burning_point: FIRE_SOURCE_IGNITE_FRAMES,
            temperature: 0f64,
            heat: 1f64,
        };
        FireSource {
            attribute_manager,
            asset,
            status_manager,
            animation_timer: 0f64,
        }
    }
    fn animate_flame (&mut self) {
        let time_per_sprite = FIRE_ANIMATION_TIME / FIRE_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.animation_timer / time_per_sprite) as isize % FIRE_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(FIRE_SOURCE_ON_X_OFFSET + dx as f64);
    }
    fn animate_idle (&mut self) {
        self.status_manager.delta_time = 0f64;
    }
    fn animate_walking (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        let delta_time = self.status_manager.delta_time;
        self.status_manager.set_next_coordinate(delta_time, FIRE_SOURCE_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.set_status(Status::Idle);
        }
    }
}