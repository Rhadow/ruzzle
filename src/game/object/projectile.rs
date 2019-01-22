// use web_sys::console::log_1;
use super::Object;
use crate::audio::AudioPlayer;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::PROJECTILE_MOVE_TIME;

pub struct Projectile {
    is_visible: bool,
    delta_time: f64,
    time: f64,
    asset: Asset,
    status_manager: StatusManager,
}

impl Object for Projectile {
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
    fn walk(&mut self, direction: Direction, world: &World) {
        if self.can_move_to(&direction, world) {
            let next_position = self.status_manager.get_next_position_by_direction(&direction);
            self.status_manager.walk_to(next_position);
        } else {
            self.is_visible = false;
        }
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
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Projectile {
    pub fn new(position: Position, asset: Asset) -> Projectile {
        let status_manager = StatusManager::new(position, Direction::Right);
        Projectile {
            is_visible: true,
            asset,
            status_manager,
            delta_time: 0f64,
            time: 0f64,
        }
    }
    fn animate_walking (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, PROJECTILE_MOVE_TIME);
        if self.status_manager.is_coordinate_equal_position() {
            self.status_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}