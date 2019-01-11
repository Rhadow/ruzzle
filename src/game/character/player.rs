use std::time::SystemTime;
use super::Character;
use crate::game::{Asset, AssetType, MovementManager, Position, Direction, Status};
use crate::game::constants::{
    PLAYER_BASE_X_OFFSET,
    PLAYER_BASE_Y_OFFSET,
    PLAYER_WIDTH,
    PLAYER_HEIGHT,
    PLAYER_MOVE_TIME,
    PLAYER_ANIMATION_TIME
};

pub struct Player {
    delta_time: u64,
    time: SystemTime,
    asset: Asset,
    movement_manager: MovementManager
}

impl Character for Player {
    fn asset(&self) -> &Asset {
        &self.asset
    }

    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }

    fn move_by_direction(&mut self, direction: Direction) {
        if self.movement_manager.status == Status::Idle {
            self.movement_manager.status = Status::Walking;
            self.movement_manager.move_by_direction(direction);
        }
    }

    fn update(&mut self) {
        let now = SystemTime::now();
        self.delta_time += self.get_dt(now);
        self.time = now;
        match self.movement_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(),
        }
    }
}

impl Player {
    pub fn new(position: Position, direction: Direction) -> Player {
        let asset = Asset::new(
            AssetType::Character,
            PLAYER_BASE_X_OFFSET,
            PLAYER_BASE_Y_OFFSET,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
        );
        let movement_manager = MovementManager::new(position, direction);
        Player {
            asset,
            movement_manager,
            time: SystemTime::now(),
            delta_time: 0u64,
        }
    }

    fn animate_idle (&mut self) {
        match self.movement_manager.direction {
            Direction::Down => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 0f64),
            Direction::Right => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 2f64),
            Direction::Up => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 4f64),
            Direction::Left => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 6f64),
        }
        self.delta_time = 0;
    }

    fn animate_walking (&mut self) {
        let target_coordinate = MovementManager::position_to_coordinate(self.movement_manager.position);
        if self.movement_manager.coordinate == target_coordinate {
            self.movement_manager.status = Status::Idle;
            self.delta_time = 0;
        } else {
            let source_coordinate = MovementManager::position_to_coordinate(self.movement_manager.last_position);
            let distance_ratio = (self.delta_time as f64) / PLAYER_MOVE_TIME;
        }
    }

    fn get_dt(&self, now: SystemTime) -> u64 {
        let time_diff = now.duration_since(self.time).unwrap();
        time_diff.as_secs() * 1000 + time_diff.subsec_nanos() as u64 / 1_000_000
    }
}