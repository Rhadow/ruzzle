// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::AudioPlayer;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::utils::check_collision;
use crate::game::constants::{
    PROJECTILE_SIZE,
    PROJECTILE_MOVE_TIME,
};

pub struct Projectile {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
    time: f64,
    delta_time: f64,
    projector_id: String,
}

impl Object for Projectile {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn walk(&mut self, direction: Direction, _world: &World) {
        let next_position = self.status_manager.get_next_position_by_direction(&direction);
        self.status_manager.walk_to(next_position);
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.delta_time += now - self.time;
        self.time = now;
        match self.status_manager.status {
            Status::Idle => {
                let direction = self.status_manager.direction.clone();
                self.delta_time = 0f64;
                self.walk(direction, world);
            }
            Status::Walking => {
                for object in world.get_objects() {
                    if object.try_borrow_mut().is_ok() {
                        let mut object = object.borrow_mut();
                        let is_object_projectile = object.attribute_manager().is_projectile;
                        let is_object_visible = object.attribute_manager().is_visible;
                        let object_id = object.attribute_manager().id.clone();
                        if object_id.to_string() != self.projector_id && is_object_visible {
                            let is_collided = check_collision(object.status_manager(), &self.status_manager);
                            if is_collided {
                                self.attribute_manager.is_visible = false;
                                if is_object_projectile {
                                    object.attribute_manager().is_visible = false;
                                }
                            }
                        }
                    }
                }
                if !self.status_manager.position.is_in_tile_map() {
                    self.attribute_manager.is_visible = false;
                }
                self.animate_walking(audio);
            },
            _ => (),
        }
    }
}

impl Projectile {
    pub fn new(position: Position, asset: Asset, direction: Direction, projector_id: String, id: String) -> Projectile {
        let status_manager = StatusManager::new(position, direction, PROJECTILE_SIZE * 2f64, PROJECTILE_SIZE * 2f64);
        let attribute_manager = AttributeManager {
            id,
            is_visible: true,
            can_step_on: false,
            is_pushable: false,
            is_filler: false,
            is_rotatable: false,
            is_projectile: true,
            is_projecting: false,
        };
        Projectile {
            asset,
            status_manager,
            attribute_manager,
            time: 0f64,
            delta_time: 0f64,
            projector_id,
        }
    }
    fn animate_walking (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, PROJECTILE_MOVE_TIME);
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}