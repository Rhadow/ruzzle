// use web_sys::console::log_1;
// log_1(&format!("{}", x).into())
use std::cell::RefCell;
use super::Character;
use crate::audio::{SFX, AudioPlayer};
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
    PLAYER_FALLING_ANIMATION_TIME,
    PLAYER_FALLING_ANIMATION_SPRITE_LENGTH,
    PLAYER_FALLING_X_OFFSET,
    PLAYER_RESPAWNING_ANIMATION_TIME,
    PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH,
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
                    self.walk_to(next_position, world);
                }
            }
        }
    }
    fn fall(&mut self) {
        self.movement_manager.status = Status::Falling;
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut AudioPlayer) {
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
            Status::Walking => self.animate_moving(audio),
            Status::Pushing => self.animate_moving(audio),
            Status::Falling => self.animate_falling(audio),
            Status::Respawning => self.animate_respawning(audio),
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

    fn set_idle(&mut self) {
        self.movement_manager.status = Status::Idle;
        self.delta_time = 0f64;
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

    fn animate_moving (&mut self, _audio: &mut AudioPlayer) {
        self.movement_manager.set_next_coordinate(self.delta_time, PLAYER_MOVE_TIME);
        match self.movement_manager.status {
            Status::Walking => self.update_walking_sprite(),
            Status::Pushing => self.update_pushing_sprite(),
            _ => (),
        }
        if self.movement_manager.is_coordinate_equal_position() {
            self.set_idle();
        }
    }

    fn animate_falling (&mut self, audio: &mut AudioPlayer) {
        self.update_falling_sprite();
        audio.play_sfx(SFX::Dead);
        if self.delta_time >= PLAYER_FALLING_ANIMATION_TIME {
            self.movement_manager.status = Status::Respawning;
            self.delta_time = 0f64;
            let new_position = self.movement_manager.last_position;
            self.movement_manager.set_position(new_position);
        }
    }

    fn animate_respawning (&mut self, _audio: &mut AudioPlayer) {
        self.update_respawning_sprite();
        if self.delta_time >= PLAYER_RESPAWNING_ANIMATION_TIME {
            self.set_idle();
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

    fn update_falling_sprite(&mut self) {
        let sprite_dt = PLAYER_FALLING_ANIMATION_TIME / PLAYER_FALLING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_FALLING_ANIMATION_SPRITE_LENGTH;
        // FIX: The hard coded offsets needs to be removed once formal assets are done.
        if dx == 0 {
            self.asset.set_x_offset(PLAYER_BASE_X_OFFSET + PLAYER_FALLING_X_OFFSET + dx as f64);
            self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET);
        } else {
            self.asset.set_x_offset(4f64);
            self.asset.set_y_offset(1f64);
        }
    }

    fn update_respawning_sprite(&mut self) {
        let sprite_dt = PLAYER_RESPAWNING_ANIMATION_TIME / PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH as f64;
        let dy = (self.delta_time / sprite_dt) as usize % PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH as usize;
        let y_offset = self.asset.get_y_offset() as usize;
        self.asset.set_x_offset(PLAYER_BASE_X_OFFSET);
        self.asset.set_y_offset(((y_offset + dy * 2) % 8) as f64);
    }

    fn walk_to(&mut self, position: Position, world: &World) {
        self.delta_time = 0f64;
        self.movement_manager.walk_to(position);
        let tile = world.get_tile_by_position(&position).borrow_mut();
        let terrain = tile.terrain();
        if let Some(terrain) = terrain {
            let mut terrain = terrain.borrow_mut();
            if !terrain.is_filled() {
                terrain.set_falling_schedule(PLAYER_MOVE_TIME);
            }
        }
    }

    fn push_object(&mut self, direction: Direction, object: &RefCell<Box<Object>>, world: &World) {
        self.movement_manager.status = Status::Pushing;
        object.borrow_mut().walk(direction, world);
    }

    fn interact(&mut self, object: &RefCell<Box<Object>>, position: Position, direction: Direction, world: &World) {
        if object.borrow().can_step_on() {
            self.walk_to(position, world);
        } else if object.borrow().is_pushable() {
            if object.borrow().can_move_to(&direction, world) {
                self.push_object(direction, object, world);
                self.walk_to(position, world);
            }
        }
    }
}