// use web_sys::console::log_1;
use super::Object;
use crate::audio::AudioPlayer;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::game::constants::{
    CANNON_MOVE_TIME,
    CANNON_UP_X_OFFSET,
    CANNON_UP_Y_OFFSET,
    CANNON_DOWN_X_OFFSET,
    CANNON_DOWN_Y_OFFSET,
    CANNON_LEFT_X_OFFSET,
    CANNON_LEFT_Y_OFFSET,
    CANNON_RIGHT_X_OFFSET,
    CANNON_RIGHT_Y_OFFSET,
    CANNON_VERTICAL_WIDTH,
    CANNON_VERTICAL_HEIGHT,
    CANNON_HORIZONTAL_WIDTH,
    CANNON_HORIZONTAL_HEIGHT,
};

pub struct Cannon {
    is_visible: bool,
    delta_time: f64,
    time: f64,
    asset: Asset,
    is_pushable: bool,
    status_manager: StatusManager,
    is_rotatable: bool,
}

impl Object for Cannon {
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
    fn is_rotatable(&self) -> bool {
        self.is_rotatable
    }
    fn rotate(&mut self) {
        match self.status_manager.direction {
            Direction::Up => self.status_manager.direction = Direction::Right,
            Direction::Right => self.status_manager.direction = Direction::Down,
            Direction::Down => self.status_manager.direction = Direction::Left,
            Direction::Left => self.status_manager.direction = Direction::Up,
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
        self.delta_time += now - self.time;
        self.time = now;
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(audio),
            _ => (),
        }
        match self.status_manager.direction {
            Direction::Down => {
                self.asset.set_x_offset(CANNON_DOWN_X_OFFSET);
                self.asset.set_y_offset(CANNON_DOWN_Y_OFFSET);
                self.asset.set_width(CANNON_VERTICAL_WIDTH);
                self.asset.set_height(CANNON_VERTICAL_HEIGHT);
            }
            Direction::Up => {
                self.asset.set_x_offset(CANNON_UP_X_OFFSET);
                self.asset.set_y_offset(CANNON_UP_Y_OFFSET);
                self.asset.set_width(CANNON_VERTICAL_WIDTH);
                self.asset.set_height(CANNON_VERTICAL_HEIGHT);
            }
            Direction::Right => {
                self.asset.set_x_offset(CANNON_RIGHT_X_OFFSET);
                self.asset.set_y_offset(CANNON_RIGHT_Y_OFFSET);
                self.asset.set_width(CANNON_HORIZONTAL_WIDTH);
                self.asset.set_height(CANNON_HORIZONTAL_HEIGHT);
            }
            Direction::Left => {
                self.asset.set_x_offset(CANNON_LEFT_X_OFFSET);
                self.asset.set_y_offset(CANNON_LEFT_Y_OFFSET);
                self.asset.set_width(CANNON_HORIZONTAL_WIDTH);
                self.asset.set_height(CANNON_HORIZONTAL_HEIGHT);
            }
        }
    }
}

impl Cannon {
    pub fn new(position: Position, direction: Direction, asset: Asset) -> Cannon {
        let status_manager = StatusManager::new(position, direction);
        Cannon {
            is_visible: true,
            asset,
            delta_time: 0f64,
            is_pushable: true,
            is_rotatable: true,
            status_manager,
            time: 0f64,
        }
    }
    fn animate_idle (&mut self) {
        self.delta_time = 0f64;
    }
    fn animate_walking (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, CANNON_MOVE_TIME);
        if self.status_manager.is_coordinate_equal_position() {
            self.status_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}