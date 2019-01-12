// use web_sys::console::log_1;
use super::Object;
use crate::game::{Asset, AssetType, Coordinate, Direction, MovementManager, Position, World};
use crate::game::movement_manager::Status;
use crate::game::constants::{
    ROCK_X_OFFSET,
    ROCK_Y_OFFSET,
    ROCK_SIZE,
    ROCK_MOVE_TIME,
};
use crate::utils::coordinate_lerp;

pub struct Rock {
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
    fn is_pushable(&self) -> bool {
        self.is_pushable
    }
    fn can_move_to(&self, direction: &Direction, world: &World) -> bool {
        let (d_row, d_col) = match direction {
            Direction::Up => (-1f64, 0f64),
            Direction::Down => (1f64, 0f64),
            Direction::Left => (0f64, -1f64),
            Direction::Right => (0f64, 1f64),
        };
        let new_position = Position(self.movement_manager.position.row() + d_row, self.movement_manager.position.col() + d_col);
        if !new_position.is_in_tile_map() {
            return false;
        }
        if self.movement_manager.status != Status::Idle {
            return false;
        }
        let object = world.get_object_by_position(&new_position);
        let mut has_object = false;
        let mut object_walkable = false;
        if let Some(object) = object {
            has_object = true;
            object_walkable = object.borrow().is_walkable();
        }
        !has_object || object_walkable
    }
    fn step(&mut self, direction: Direction, _world: &World) {
        let (d_row, d_col) = match direction {
            Direction::Up => (-1f64, 0f64),
            Direction::Down => (1f64, 0f64),
            Direction::Left => (0f64, -1f64),
            Direction::Right => (0f64, 1f64),
        };
        let new_position = Position(self.movement_manager.position.row() + d_row, self.movement_manager.position.col() + d_col);
        self.movement_manager.walk_to(new_position);
    }
    fn update(&mut self, now: f64) {
        self.delta_time += now - self.time;
        self.time = now;
        match self.movement_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(),
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
    fn animate_walking (&mut self) {
        let dst_coordinate = MovementManager::position_to_coordinate(self.movement_manager.position);
        self.movement_manager.coordinate = self.calculate_next_coordinate(dst_coordinate);
        if self.movement_manager.coordinate == dst_coordinate {
            self.movement_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }
    fn calculate_next_coordinate(&self, dst_coordinate: Coordinate) -> Coordinate {
        let src_coordinate = MovementManager::position_to_coordinate(self.movement_manager.last_position);
        let mut distance_ratio = self.delta_time / ROCK_MOVE_TIME;
        if distance_ratio >= 1f64 {
            distance_ratio = 1f64;
        }
        coordinate_lerp(src_coordinate, dst_coordinate, distance_ratio)
    }
}