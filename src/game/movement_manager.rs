use crate::game::{Coordinate, Position};
use crate::game::constants::CELL_SIZE;

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
    pub fn walk(&mut self, direction: Direction) {
        let (d_row, d_col) = match direction {
            Direction::Up => (-1f64, 0f64),
            Direction::Down => (1f64, 0f64),
            Direction::Left => (0f64, -1f64),
            Direction::Right => (0f64, 1f64),
        };
        let new_position = Position(self.position.row() + d_row, self.position.col() + d_col);
        if new_position.is_in_world() {
            self.status = Status::Walking;
            self.last_position = self.position;
            self.position = new_position;
        }
        if direction != self.direction {
            self.direction = direction;
        }
    }

    pub fn position_to_coordinate(position: Position) -> Coordinate {
        Coordinate(position.col() * CELL_SIZE, position.row() * CELL_SIZE)
    }
}
