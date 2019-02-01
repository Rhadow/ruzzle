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
    PLAYER_DOWN_X_OFFSET,
    PLAYER_DOWN_Y_OFFSET,
    PLAYER_UP_X_OFFSET,
    PLAYER_UP_Y_OFFSET,
    PLAYER_LEFT_X_OFFSET,
    PLAYER_LEFT_Y_OFFSET,
    PLAYER_RIGHT_X_OFFSET,
    PLAYER_RIGHT_Y_OFFSET,
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
    PLAYER_DEAD_ANIMATION_TIME,
    PLAYER_DEAD_ANIMATION_SPRITE_LENGTH,
    PLAYER_DEAD_X_OFFSET,
    PLAYER_RESPAWNING_ANIMATION_TIME,
    PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH,
    PLAYER_EXITING_ANIMATION_TIME,
    PLAYER_EXITING_ANIMATION_SPRITE_LENGTH,
    PLAYER_ROTATION_LAG,
};

pub struct Player {
    delta_rotate_time: f64,
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
        if self.status_manager.direction != direction {
            self.status_manager.set_direction(direction);
            self.delta_rotate_time = 0f64;
        }
        if self.delta_rotate_time > PLAYER_ROTATION_LAG {
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
    }
    fn rotate_item(&mut self, world: &World) {
        let target_position = self.status_manager.get_next_position_by_direction(&self.status_manager.direction);
        if target_position.is_in_tile_map() {
            let object = world.get_object_by_position(&target_position);
            if let Some(object) = object {
                let mut object = object.borrow_mut();
                let is_object_rotatable = object.attribute_manager().is_rotatable;
                if is_object_rotatable {
                    object.rotate();
                }
            }
        }
    }
    fn handle_fall(&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_status(Status::Dead);
        audio.play_sfx(SFX::Dead);
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        if self.status_manager.time != 0f64 {
            self.status_manager.delta_time += now - self.status_manager.time;
            self.delta_rotate_time += now - self.status_manager.time;
        }
        self.status_manager.time = now;
        match self.status_manager.direction {
            Direction::Down => self.asset.set_y_offset(PLAYER_DOWN_Y_OFFSET),
            Direction::Right => self.asset.set_y_offset(PLAYER_RIGHT_Y_OFFSET),
            Direction::Up => self.asset.set_y_offset(PLAYER_UP_Y_OFFSET),
            Direction::Left => self.asset.set_y_offset(PLAYER_LEFT_Y_OFFSET),
        }
        match self.status_manager.status {
            Status::Idle => self.animate_idle(),
            Status::Walking => self.animate_moving(audio),
            Status::Pushing => self.animate_moving(audio),
            Status::Dead => self.animate_dead(audio),
            Status::Respawning => self.animate_respawning(audio),
            Status::Exiting => self.animate_exiting(world, audio),
            Status::LevelComplete => (),
        }
    }
    fn handle_death(&mut self, audio: &mut Box<dyn AudioPlayer>) {
        match self.status_manager.status {
            Status::Idle | Status::Walking | Status::Pushing => {
                self.status_manager.set_status(Status::Dead);
                audio.play_sfx(SFX::Dead);
            },
            _ => (),
        }
    }
}

impl Player {
    pub fn new(position: Position, direction: Direction) -> Player {
        let asset = Asset::new(
            AssetType::Character,
            PLAYER_DOWN_X_OFFSET,
            PLAYER_DOWN_Y_OFFSET,
            PLAYER_WIDTH,
            PLAYER_HEIGHT,
            Some((PLAYER_UP_X_OFFSET, PLAYER_RIGHT_X_OFFSET, PLAYER_DOWN_X_OFFSET, PLAYER_LEFT_X_OFFSET)),
            Some((PLAYER_UP_Y_OFFSET, PLAYER_RIGHT_Y_OFFSET, PLAYER_DOWN_Y_OFFSET, PLAYER_LEFT_Y_OFFSET)),
        );
        let status_manager = StatusManager::new(position, direction, PLAYER_WIDTH, PLAYER_HEIGHT);
        Player {
            asset,
            status_manager,
            delta_rotate_time: 0f64,
            at_exit: false,
        }
    }

    fn animate_idle (&mut self) {
        if self.asset.get_x_offset() != PLAYER_DOWN_X_OFFSET {
            self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET);
        }
        if self.status_manager.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME {
            self.update_idle_sprite();
            if self.status_manager.delta_time >= PLAYER_IDLE_ANIMATION_WAITING_TIME + PLAYER_IDLE_ANIMATION_TIME {
                self.status_manager.delta_time = 0f64;
            }
        }
    }

    fn animate_moving (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        let delta_time = self.status_manager.delta_time;
        self.status_manager.set_next_coordinate(delta_time, PLAYER_MOVE_TIME);
        match self.status_manager.status {
            Status::Walking => self.update_walking_sprite(),
            Status::Pushing => self.update_pushing_sprite(),
            _ => (),
        }
        if self.status_manager.is_arrived_at_position() {
            if self.at_exit {
                self.status_manager.set_status(Status::Exiting);
            } else {
                self.status_manager.set_status(Status::Idle);
            }
        }
    }

    fn animate_dead (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.update_dead_sprite();
        if self.status_manager.delta_time >= PLAYER_DEAD_ANIMATION_TIME {
            self.status_manager.set_status(Status::Respawning);
            let new_position = self.status_manager.initial_position;
            self.status_manager.set_position(new_position);
        }
    }

    fn animate_respawning (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.update_respawning_sprite();
        if self.status_manager.delta_time >= PLAYER_RESPAWNING_ANIMATION_TIME {
            self.status_manager.set_status(Status::Idle);
        }
    }

    fn animate_exiting (&mut self, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.update_exiting_sprite();
        audio.play_sfx(SFX::Fanfare);
        if self.status_manager.delta_time >= PLAYER_EXITING_ANIMATION_TIME {
            self.status_manager.set_status(Status::LevelComplete);
        }
    }

    fn update_idle_sprite(&mut self) {
        let sprite_dt = PLAYER_IDLE_ANIMATION_TIME / PLAYER_IDLE_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % PLAYER_IDLE_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET + PLAYER_IDLE_X_OFFSET + dx as f64);
    }

    fn update_walking_sprite(&mut self) {
        let sprite_dt = PLAYER_WALKING_ANIMATION_TIME / PLAYER_WALKING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % PLAYER_WALKING_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET + dx as f64);
    }

    fn update_pushing_sprite(&mut self) {
        let sprite_dt = PLAYER_WALKING_ANIMATION_TIME / PLAYER_WALKING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % PLAYER_WALKING_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET + PLAYER_PUSHING_X_OFFSET + dx as f64);
    }

    fn update_dead_sprite(&mut self) {
        let sprite_dt = PLAYER_DEAD_ANIMATION_TIME / PLAYER_DEAD_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % PLAYER_DEAD_ANIMATION_SPRITE_LENGTH;
        // FIX: The hard coded offsets needs to be removed once formal assets are done.
        if dx == 0 {
            self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET + PLAYER_DEAD_X_OFFSET + dx as f64);
            self.asset.set_y_offset(PLAYER_DOWN_Y_OFFSET);
        } else {
            self.asset.set_x_offset(4f64);
            self.asset.set_y_offset(1f64);
        }
    }

    fn update_exiting_sprite(&mut self) {
        let sprite_dt = PLAYER_EXITING_ANIMATION_TIME / PLAYER_EXITING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % PLAYER_EXITING_ANIMATION_SPRITE_LENGTH;
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
        let dy = (self.status_manager.delta_time / sprite_dt) as usize % PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH as usize;
        let y_offset = self.asset.get_y_offset() as usize;
        self.asset.set_x_offset(PLAYER_DOWN_X_OFFSET);
        self.asset.set_y_offset(((y_offset + dy * 2) % (PLAYER_RESPAWNING_ANIMATION_SPRITE_LENGTH as usize * 2)) as f64);
    }

    fn walk_to(&mut self, position: Position, _world: &World) {
        self.status_manager.delta_time = 0f64;
        self.status_manager.walk_to(position);
    }

    fn interact(&mut self, object: &RefCell<Box<Object>>, position: Position, direction: Direction, world: &World) {
        let mut object_mut = object.borrow_mut();
        let can_object_step_on = object_mut.attribute_manager().can_step_on;
        let is_object_pushable = object_mut.attribute_manager().is_pushable;
        if can_object_step_on {
            self.walk_to(position, world);
            object_mut.interact(self);
        } else if is_object_pushable {
            if object_mut.can_move_to(&direction, world) {
                self.status_manager.set_status(Status::Pushing);
                object_mut.walk(direction, world);
                self.walk_to(position, world);
            }
        }
    }
}