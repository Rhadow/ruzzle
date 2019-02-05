use crate::game::{Coordinate, Position};
use crate::game::constants::TILE_SIZE;
use crate::utils::coordinate_lerp;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Status {
    Idle,
    Walking,
    Pushing,
    Dead,
    Respawning,
    Exiting,
    LevelComplete,
}

pub struct StatusManager {
    pub status: Status,
    pub coordinate: Coordinate,
    pub position: Position,
    pub initial_position: Position,
    pub last_position: Position,
    pub direction: Direction,
    pub width: f64,
    pub height: f64,
    pub time: f64,
    pub delta_time: f64,
    pub animation_timer: f64,
}

impl StatusManager {
    pub fn new(position: Position, direction: Direction, width: f64, height: f64) -> StatusManager {
        let coordinate = StatusManager::position_to_coordinate(position, width, height);
        StatusManager {
            status: Status::Idle,
            coordinate,
            position,
            last_position: position,
            initial_position: position,
            direction,
            width,
            height,
            time: 0f64,
            delta_time: 0f64,
            animation_timer: 0f64,
        }
    }
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
        self.coordinate = StatusManager::position_to_coordinate(self.position, self.width, self.height);
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
        self.coordinate = StatusManager::position_to_coordinate(self.position, self.width, self.height);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.direction {
            self.direction = direction;
        }
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
        self.coordinate = StatusManager::position_to_coordinate(self.position, self.width, self.height);
        self.last_position = new_position;
    }

    pub fn walk_to(&mut self, new_position: Position) {
        if self.status != Status::Pushing {
            self.status = Status::Walking;
        }
        self.last_position = self.position;
        self.position = new_position;
    }

    pub fn set_next_coordinate(&mut self, delta_time: f64, total_move_time: f64) {
        let src_coordinate = StatusManager::position_to_coordinate(self.last_position, self.width, self.height);
        let dst_coordinate = StatusManager::position_to_coordinate(self.position, self.width, self.height);
        let mut distance_ratio = delta_time / total_move_time;
        if distance_ratio >= 1f64 {
            distance_ratio = 1f64;
        }
        self.coordinate = coordinate_lerp(src_coordinate, dst_coordinate, distance_ratio);
    }

    pub fn is_arrived_at_position(&self) -> bool {
        self.coordinate == StatusManager::position_to_coordinate(self.position, self.width, self.height)
    }

    pub fn position_to_coordinate(position: Position, width: f64, height: f64) -> Coordinate {
        let mut x = position.col() * TILE_SIZE;
        let mut y = position.row() * TILE_SIZE;
        x = x + (TILE_SIZE - width) / 2f64;
        y = y + (TILE_SIZE - height) / 2f64;
        Coordinate(x, y)
    }

    pub fn get_next_position_by_direction(&self, direction: &Direction) -> Position {
        let (d_row, d_col) = match direction {
            Direction::Up => (-1f64, 0f64),
            Direction::Down => (1f64, 0f64),
            Direction::Left => (0f64, -1f64),
            Direction::Right => (0f64, 1f64),
        };
        Position(self.position.row() + d_row, self.position.col() + d_col)
    }

    pub fn set_status(&mut self, new_status: Status) {
        self.delta_time = 0f64;
        self.status = new_status;
    }

    pub fn update_time(&mut self, now: f64) {
        if self.time != 0f64 {
            self.delta_time += now - self.time;
            self.animation_timer += now - self.time;
        }
        self.time = now;
    }
}
