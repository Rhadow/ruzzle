// use web_sys::console::log_1;
use std::cell::RefCell;
use crate::utils::coordinate_lerp;
use super::Character;
use crate::game::object::Object;
use crate::game::{
    Asset,
    AssetType,
    Coordinate,
    Direction,
    MovementManager,
    Position,
    Status,
    World,
};
use crate::game::constants::{
    PLAYER_BASE_X_OFFSET,
    PLAYER_BASE_Y_OFFSET,
    PLAYER_WIDTH,
    PLAYER_HEIGHT,
    PLAYER_MOVE_TIME,
    PLAYER_WALKING_ANIMATION_TIME,
    PLAYER_WALKING_ANIMATION_SPRITE_LENGTH,
    PLAYER_IDLE_X_OFFSET,
    PLAYER_IDLE_ANIMATION_TIME,
    PLAYER_IDLE_ANIMATION_SPRITE_LENGTH,
    PLAYER_IDLE_ANIMATION_WAITING_TIME,
};

pub struct Player {
    delta_time: f64,
    time: f64,
    asset: Asset,
    movement_manager: MovementManager,
}

impl Character for Player {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
    fn step(&mut self, dir: Direction, world: &World) {
        let (d_row, d_col) = match dir {
            Direction::Up => (-1f64, 0f64),
            Direction::Down => (1f64, 0f64),
            Direction::Left => (0f64, -1f64),
            Direction::Right => (0f64, 1f64),
        };
        self.movement_manager.set_direction(dir.clone());
        let new_position = Position(self.movement_manager.position.row() + d_row, self.movement_manager.position.col() + d_col);
        if new_position.is_in_tile_map() {
            let object = world.get_object_by_position(&new_position);
            match object {
                Some(object) => {
                    if object.borrow().is_walkable() {
                        self.walk_to(new_position);
                    } else if object.borrow().is_pushable() {
                        if object.borrow().can_move_to(&dir, world) {
                            self.push_object(dir.clone(), object, world);
                            self.walk_to(new_position);
                        }
                    }
                },
                None => {
                    self.walk_to(new_position);
                }
            }
        }
    }
    fn update(&mut self, now: f64) {
        self.delta_time += now - self.time;
        self.time = now;
        match self.movement_manager.direction {
            Direction::Down => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 0f64),
            Direction::Right => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 2f64),
            Direction::Up => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 4f64),
            Direction::Left => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 6f64),
        }
        match self.movement_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_walking(),
            Status::Pushing => self.animate_walking(),
        }
    }
}

impl Player {
    pub fn new(position: Position, direction: Direction, now: f64) -> Player {
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
            time: now,
            delta_time: 0f64,
        }
    }

    fn animate_idle (&mut self) {
        if self.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME {
            self.update_idle_sprite();
            if self.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME + PLAYER_IDLE_ANIMATION_TIME {
                self.asset.set_x_offset(PLAYER_BASE_X_OFFSET);
                self.delta_time = 0f64;
            }
        }
    }

    fn animate_walking (&mut self) {
        let dst_coordinate = MovementManager::position_to_coordinate(self.movement_manager.position);
        self.movement_manager.coordinate = self.calculate_next_coordinate(dst_coordinate);
        self.update_walking_sprite();
        if self.movement_manager.coordinate == dst_coordinate {
            self.movement_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
    }

    fn calculate_next_coordinate(&self, dst_coordinate: Coordinate) -> Coordinate {
        let src_coordinate = MovementManager::position_to_coordinate(self.movement_manager.last_position);
        let mut distance_ratio = self.delta_time / PLAYER_MOVE_TIME;
        if distance_ratio >= 1f64 {
            distance_ratio = 1f64;
        }
        coordinate_lerp(src_coordinate, dst_coordinate, distance_ratio)
    }

    fn update_idle_sprite(&mut self) {
        let sprite_dt = PLAYER_IDLE_ANIMATION_TIME / PLAYER_IDLE_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_IDLE_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_BASE_X_OFFSET + PLAYER_IDLE_X_OFFSET + dx as f64);
    }

    fn update_walking_sprite(&mut self) {
        let sprite_dt = PLAYER_WALKING_ANIMATION_TIME / PLAYER_WALKING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_WALKING_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_BASE_X_OFFSET + dx as f64);
    }

    fn walk_to(&mut self, position: Position) {
        self.delta_time = 0f64;
        self.movement_manager.walk_to(position);
    }

    fn push_object(&mut self, direction: Direction, object: &RefCell<Box<Object>>, world: &World) {
        self.delta_time = 0f64;
        self.movement_manager.status = Status::Pushing;
        object.borrow_mut().step(direction, world);
    }
}