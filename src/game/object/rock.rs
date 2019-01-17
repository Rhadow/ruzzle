// use web_sys::console::log_1;
use super::Object;
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::ROCK_MOVE_TIME;

pub struct Rock {
    is_visible: bool,
    delta_time: f64,
    time: f64,
    asset: Asset,
    is_pushable: bool,
    status_manager: StatusManager,
}

impl Object for Rock {
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
    fn is_pushable(&self) -> bool {
        self.is_pushable
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
        self.delta_time += now - self.time;
        self.time = now;
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Rock {
    pub fn new(position: Position, asset: Asset) -> Rock {
        let status_manager = StatusManager::new(position, Direction::Down);
        Rock {
            is_visible: true,
            asset,
            delta_time: 0f64,
            is_pushable: true,
            status_manager,
            time: 0f64,
        }
    }
    fn animate_idle (&mut self) {
        self.delta_time = 0f64;
    }
    fn animate_walking (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, ROCK_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        if self.status_manager.is_coordinate_equal_position() {
            self.status_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}