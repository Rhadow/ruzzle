// use web_sys::console::log_1;
use super::Object;
use crate::audio::AudioPlayer;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::utils::check_collision;
use crate::game::constants::{
    PROJECTILE_SIZE,
    PROJECTILE_MOVE_TIME,
};

pub struct Projectile {
    id: String,
    is_visible: bool,
    delta_time: f64,
    time: f64,
    asset: Asset,
    status_manager: StatusManager,
    projector_id: String,
}

impl Object for Projectile {
    fn id(&self) -> &String {
        &self.id
    }
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
    fn is_projectile(&self) -> bool {
        true
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
                        if object.id().to_string() != self.projector_id && object.is_visible() {
                            let is_collided = check_collision(object.status_manager(), &self.status_manager);
                            if is_collided {
                                self.is_visible = false;
                                if object.is_projectile() {
                                    object.set_visible(false);
                                }
                            }
                        }
                    }
                }
                if !self.status_manager.position.is_in_tile_map() {
                    self.is_visible = false;
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
        Projectile {
            id,
            projector_id,
            is_visible: true,
            asset,
            status_manager,
            delta_time: 0f64,
            time: 0f64,
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