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
    StatusManager,
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
    PLAYER_EXITING_ANIMATION_TIME,
    PLAYER_EXITING_ANIMATION_SPRITE_LENGTH,
};

pub struct Player {
    delta_time: f64,
    time: f64,
    asset: Asset,
    status_manager: StatusManager,
    pub at_exit: bool,
}

impl Character for Player {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn at_exit(&self) -> bool {
        self.at_exit
    }
    fn walk(&mut self, direction: Direction, world: &World) {
        self.status_manager.set_direction(direction.clone());
        let next_position = self.status_manager.get_next_position_by_direction(&direction);
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
        self.status_manager.status = Status::Falling;
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.delta_time += now - self.time;
        self.time = now;
        match self.status_manager.direction {
            Direction::Down => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 0f64),
            Direction::Right => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 2f64),
            Direction::Up => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 4f64),
            Direction::Left => self.asset.set_y_offset(PLAYER_BASE_Y_OFFSET + 6f64),
        }
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_moving(audio),
            Status::Pushing => self.animate_moving(audio),
            Status::Falling => self.animate_falling(audio),
            Status::Respawning => self.animate_respawning(audio),
            Status::Exiting => self.animate_exiting(world, audio),
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
        let status_manager = StatusManager::new(position, direction);
        Player {
            asset,
            status_manager,
            time: now,
            delta_time: 0f64,
            at_exit: false,
        }
    }

    fn set_idle(&mut self) {
        self.status_manager.status = Status::Idle;
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

    fn animate_moving (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, PLAYER_MOVE_TIME);
        match self.status_manager.status {
            Status::Walking => self.update_walking_sprite(),
            Status::Pushing => self.update_pushing_sprite(),
            _ => (),
        }
        if self.status_manager.is_coordinate_equal_position() {
            if self.at_exit {
                self.status_manager.status = Status::Exiting;
                self.delta_time = 0f64;
            } else {
                self.set_idle();
            }
        }
    }

    fn animate_falling (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.update_falling_sprite();
        audio.play_sfx(SFX::Dead);
        if self.delta_time >= PLAYER_FALLING_ANIMATION_TIME {
            self.status_manager.status = Status::Respawning;
            self.delta_time = 0f64;
            let new_position = self.status_manager.last_position;
            self.status_manager.set_position(new_position);
        }
    }

    fn animate_respawning (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.update_respawning_sprite();
        if self.delta_time >= PLAYER_RESPAWNING_ANIMATION_TIME {
            self.set_idle();
        }
    }

    fn animate_exiting (&mut self, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.update_exiting_sprite();
        audio.play_sfx(SFX::Fanfare);
        if self.delta_time >= PLAYER_EXITING_ANIMATION_TIME {
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

    fn update_exiting_sprite(&mut self) {
        let sprite_dt = PLAYER_EXITING_ANIMATION_TIME / PLAYER_EXITING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_EXITING_ANIMATION_SPRITE_LENGTH;
        // FIX: The hard coded offsets needs to be removed once formal assets are done.
        if dx % 2 == 0 {
            self.asset.set_x_offset(0f64);
            self.asset.set_y_offset(0f64);
        } else {
            self.asset.set_x_offset(7f64);
            self.asset.set_y_offset(0f64);
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
        self.status_manager.walk_to(position);
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
        self.status_manager.status = Status::Pushing;
        object.borrow_mut().walk(direction, world);
    }

    fn interact(&mut self, object: &RefCell<Box<Object>>, position: Position, direction: Direction, world: &World) {
        if object.borrow_mut().can_step_on() {
            self.walk_to(position, world);
            object.borrow_mut().interact(self);
        } else if object.borrow().is_pushable() {
            if object.borrow().can_move_to(&direction, world) {
                self.push_object(direction, object, world);
                self.walk_to(position, world);
            }
        }
    }
}