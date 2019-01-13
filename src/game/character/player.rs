// use web_sys::console::log_1;
// log1(&format!("{}", x),into())
use std::cell::RefCell;
use super::Character;
use crate::game::object::Object;
use crate::game::{
    Asset,
    AssetType,
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
    PLAYER_PUSHING_X_OFFSET,
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
    fn walk(&mut self, direction: Direction, world: &World) {
        self.movement_manager.set_direction(direction.clone());
        let next_position = self.movement_manager.get_next_position_by_direction(&direction);
        if next_position.is_in_tile_map() {
            let object = world.get_object_by_position(&next_position);
            match object {
                Some(object) => {
                    self.interact(object, next_position, direction, world);
                },
                None => {
                    self.walk_to(next_position);
                }
            }
        }
    }
    fn update(&mut self, now: f64, _world: &World) {
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
            Status::Walking => self.animate_moving(),
            Status::Pushing => self.animate_moving(),
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
        if self.asset.get_x_offset() != PLAYER_BASE_X_OFFSET {
            self.asset.set_x_offset(PLAYER_BASE_X_OFFSET);
        }
        if self.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME {
            self.update_idle_sprite();
            if self.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME + PLAYER_IDLE_ANIMATION_TIME {
                self.delta_time = 0f64;
            }
        }
    }

    fn animate_moving (&mut self) {
        self.movement_manager.set_next_coordinate(self.delta_time, PLAYER_MOVE_TIME);
        match self.movement_manager.status {
            Status::Walking => self.update_walking_sprite(),
            Status::Pushing => self.update_pushing_sprite(),
            _ => (),
        }
        if self.movement_manager.is_coordinate_equal_position() {
            self.movement_manager.status = Status::Idle;
            self.delta_time = 0f64;
        }
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

    fn update_pushing_sprite(&mut self) {
        let sprite_dt = PLAYER_WALKING_ANIMATION_TIME / PLAYER_WALKING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_WALKING_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_BASE_X_OFFSET + PLAYER_PUSHING_X_OFFSET + dx as f64);
    }

    fn walk_to(&mut self, position: Position) {
        self.delta_time = 0f64;
        self.movement_manager.walk_to(position);
    }

    fn push_object(&mut self, direction: Direction, object: &RefCell<Box<Object>>, world: &World) {
        self.movement_manager.status = Status::Pushing;
        object.borrow_mut().walk(direction, world);
    }

    fn interact(&mut self, object: &RefCell<Box<Object>>, position: Position, direction: Direction, world: &World) {
        if object.borrow().can_step_on() {
            self.walk_to(position);
        } else if object.borrow().is_pushable() {
            if object.borrow().can_move_to(&direction, world) {
                self.push_object(direction, object, world);
                self.walk_to(position);
            }
        }
    }
}