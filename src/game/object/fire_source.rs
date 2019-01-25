use super::{AttributeManager, Object};
use crate::audio::AudioPlayer;
use crate::utils::check_collision;
use crate::game::{Asset, Direction, Status, StatusManager, Position, World};
use crate::game::constants::{
    TILE_SIZE,
    FIRE_SOURCE_X_OFFSET,
    FIRE_ANIMATION_TIME,
    FIRE_ANIMATION_SPRITE_LENGTH,
};

pub struct FireSource {
    asset: Asset,
    status_manager: StatusManager,
    pub attribute_manager: AttributeManager,
    time: f64,
    delta_time: f64,
}

impl Object for FireSource {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn update(&mut self, now: f64, world: &World, _audio: &mut Box<dyn AudioPlayer>) {
        self.delta_time += now - self.time;
        self.time = now;
        self.handle_collision(world);
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            _ => (),
        }
    }
}

impl FireSource {
    pub fn new(position: Position, asset: Asset, id: String) -> FireSource {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE / 2f64, TILE_SIZE / 2f64);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: true,
            is_visible: true,
            is_pushable: false,
            is_filler: false,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: false,
            burning_level: 0,
        };
        FireSource {
            attribute_manager,
            asset,
            status_manager,
            time: 0f64,
            delta_time: 0f64,
        }
    }
    fn animate_idle (&mut self) {
        let time_per_sprite = FIRE_ANIMATION_TIME / FIRE_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / time_per_sprite) as isize % FIRE_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(FIRE_SOURCE_X_OFFSET + dx as f64);
    }
    fn handle_collision(&mut self, world: &World) {
        for object in world.get_objects() {
            if object.try_borrow_mut().is_err() {
                continue;
            }
            let mut object = object.borrow_mut();
            let is_object_burnable = object.attribute_manager().is_burnable;
            let is_object_visible = object.attribute_manager().is_visible;
            if is_object_visible {
                let is_collided = check_collision(object.status_manager(), &self.status_manager);
                if is_collided && is_object_burnable {
                    object.attribute_manager().burning_level = 1;
                }
            }
        }
    }
}