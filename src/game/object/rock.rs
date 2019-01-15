// use web_sys::console::log_1;
use super::Object;
use crate::audio::{SFX, AudioPlayer};
use crate::game::{Asset, AssetType, Direction, MovementManager, Position, World};
use crate::game::movement_manager::Status;
use crate::game::constants::{
    ROCK_X_OFFSET,
    ROCK_Y_OFFSET,
    ROCK_SIZE,
    ROCK_MOVE_TIME,
};

pub struct Rock {
    is_visible: bool,
    delta_time: f64,
    time: f64,
    asset: Asset,
    is_pushable: bool,
    movement_manager: MovementManager,
}

impl Object for Rock {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
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
        let next_position = self.movement_manager.get_next_position_by_direction(&direction);
        self.movement_manager.walk_to(next_position);
        let tile = world.get_tile_by_position(&next_position).borrow_mut();
        let terrain = tile.terrain();
        if let Some(terrain) = terrain {
            let mut terrain = terrain.borrow_mut();
            if !terrain.is_filled() {
                terrain.set_falling_schedule(ROCK_MOVE_TIME);
            }
        }
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut AudioPlayer) {
        self.delta_time += now - self.time;
        self.time = now;
        match self.movement_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
    }
}

impl Rock {
    pub fn new(position: Position) -> Rock {
        let asset = Asset::new(
            AssetType::Environment,
            ROCK_X_OFFSET,
            ROCK_Y_OFFSET,
            ROCK_SIZE,
            ROCK_SIZE,
        );
        let movement_manager = MovementManager::new(position, Direction::Down);
        Rock {
            is_visible: true,
            asset,
            delta_time: 0f64,
            is_pushable: true,
            movement_manager,
            time: 0f64,
        }
    }
    fn animate_idle (&mut self) {
        self.delta_time = 0f64;
    }
    fn animate_walking (&mut self, audio: &mut AudioPlayer) {
        self.movement_manager.set_next_coordinate(self.delta_time, ROCK_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        if self.movement_manager.is_coordinate_equal_position() {
            self.movement_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}