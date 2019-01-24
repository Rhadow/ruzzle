// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::{SFX, AudioPlayer};
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
    CANNON_PROJECT_TIME,
};

pub struct Projector {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
    time: f64,
    delta_time: f64,
    projection_timer: f64,
}

impl Object for Projector {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn rotate(&mut self) {
        match self.status_manager.direction {
            Direction::Up => self.status_manager.direction = Direction::Right,
            Direction::Right => self.status_manager.direction = Direction::Down,
            Direction::Down => self.status_manager.direction = Direction::Left,
            Direction::Left => self.status_manager.direction = Direction::Up,
        }
        match self.status_manager.direction {
            Direction::Down => {
                self.asset.set_x_offset(CANNON_DOWN_X_OFFSET);
                self.asset.set_y_offset(CANNON_DOWN_Y_OFFSET);
                self.asset.set_width(CANNON_VERTICAL_WIDTH);
                self.asset.set_height(CANNON_VERTICAL_HEIGHT);
                self.status_manager.set_width(CANNON_VERTICAL_WIDTH * 2f64);
                self.status_manager.set_height(CANNON_VERTICAL_HEIGHT * 2f64);
            }
            Direction::Up => {
                self.asset.set_x_offset(CANNON_UP_X_OFFSET);
                self.asset.set_y_offset(CANNON_UP_Y_OFFSET);
                self.asset.set_width(CANNON_VERTICAL_WIDTH);
                self.asset.set_height(CANNON_VERTICAL_HEIGHT);
                self.status_manager.set_width(CANNON_VERTICAL_WIDTH * 2f64);
                self.status_manager.set_height(CANNON_VERTICAL_HEIGHT * 2f64);
            }
            Direction::Right => {
                self.asset.set_x_offset(CANNON_RIGHT_X_OFFSET);
                self.asset.set_y_offset(CANNON_RIGHT_Y_OFFSET);
                self.asset.set_width(CANNON_HORIZONTAL_WIDTH);
                self.asset.set_height(CANNON_HORIZONTAL_HEIGHT);
                self.status_manager.set_width(CANNON_HORIZONTAL_WIDTH * 2f64);
                self.status_manager.set_height(CANNON_HORIZONTAL_HEIGHT * 2f64);
            }
            Direction::Left => {
                self.asset.set_x_offset(CANNON_LEFT_X_OFFSET);
                self.asset.set_y_offset(CANNON_LEFT_Y_OFFSET);
                self.asset.set_width(CANNON_HORIZONTAL_WIDTH);
                self.asset.set_height(CANNON_HORIZONTAL_HEIGHT);
                self.status_manager.set_width(CANNON_HORIZONTAL_WIDTH * 2f64);
                self.status_manager.set_height(CANNON_HORIZONTAL_HEIGHT * 2f64);
            }
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
        self.projection_timer += now - self.time;
        self.time = now;
        if self.projection_timer >= CANNON_PROJECT_TIME {
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
    pub fn new(position: Position, direction: Direction, asset: Asset, id: String) -> Projector {
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
            is_visible: true,
            can_step_on: false,
            is_pushable: true,
            is_filler: false,
            is_rotatable: true,
            is_projectile: false,
            is_projecting: false,
        };
        Projector {
            asset,
            status_manager,
            attribute_manager,
            time: 0f64,
            delta_time: 0f64,
            projection_timer: 0f64,
        }
    }
    fn animate_idle (&mut self) {
        self.delta_time = 0f64;
    }
    fn animate_walking (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, CANNON_MOVE_TIME);
        audio.play_sfx(SFX::RockMove);
        // self.projection_timer = 0f64;
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
}