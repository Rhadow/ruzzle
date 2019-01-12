use crate::game::{Coordinate, Position};
use crate::game::constants::TILE_SIZE;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
pub enum Status {
    Idle,
    Walking
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

    pub fn walk_to(&mut self, new_position: Position) {
        self.status = Status::Walking;
        self.last_position = self.position;
        self.position = new_position;
    }

    pub fn position_to_coordinate(position: Position) -> Coordinate {
        Coordinate(position.col() * TILE_SIZE, position.row() * TILE_SIZE)
    }
}
