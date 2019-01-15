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

#[derive(PartialEq)]
pub enum Status {
    Idle,
    Walking,
    Pushing,
    Falling,
    Respawning,
}

pub struct MovementManager {
    pub status: Status,
    pub coordinate: Coordinate,
    pub position: Position,
    pub last_position: Position,
    pub direction: Direction,
}

impl MovementManager {
    pub fn new(position: Position, direction: Direction) -> MovementManager {
        let coordinate = MovementManager::position_to_coordinate(position);
        MovementManager {
            status: Status::Idle,
            coordinate,
            position,
            last_position: position,
            direction
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if direction != self.direction {
            self.direction = direction;
        }
    }

    pub fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
        self.coordinate = MovementManager::position_to_coordinate(self.position);
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
        let src_coordinate = MovementManager::position_to_coordinate(self.last_position);
        let dst_coordinate = MovementManager::position_to_coordinate(self.position);
        let mut distance_ratio = delta_time / total_move_time;
        if distance_ratio >= 1f64 {
            distance_ratio = 1f64;
        }
        self.coordinate = coordinate_lerp(src_coordinate, dst_coordinate, distance_ratio);
    }

    pub fn is_coordinate_equal_position(&self) -> bool {
        self.coordinate == MovementManager::position_to_coordinate(self.position)
    }

    pub fn position_to_coordinate(position: Position) -> Coordinate {
        Coordinate(position.col() * TILE_SIZE, position.row() * TILE_SIZE)
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
}
