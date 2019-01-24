// use web_sys::console::log_1;
// log_1(&format!("{}", x).into())
use std::cell::RefCell;
use super::Character;
use crate::utils::check_collision;
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
    delta_time: f64,
    delta_rotate_time: f64,
    time: f64,
    asset: Asset,
    status_manager: StatusManager,
    pub at_exit: bool,
    is_falling: bool,
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
                if object.is_rotatable() {
                    object.rotate();
                }
            }
        }
    }
    fn fall(&mut self) {
        self.set_status(Status::Dead);
        self.is_falling = true;
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.delta_time += now - self.time;
        self.delta_rotate_time += now - self.time;
        self.time = now;
        self.handle_collision(world);
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
            Status::Dead => self.animate_dead(audio),
            Status::Respawning => self.animate_respawning(audio),
            Status::Exiting => self.animate_exiting(world, audio),
            Status::LevelComplete => (),
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
        let status_manager = StatusManager::new(position, direction, PLAYER_WIDTH * 2f64, PLAYER_HEIGHT * 2f64);
        Player {
            asset,
            status_manager,
            time: now,
            delta_time: 0f64,
            delta_rotate_time: 0f64,
            at_exit: false,
            is_falling: false,
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

    fn animate_moving (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.set_next_coordinate(self.delta_time, PLAYER_MOVE_TIME);
        match self.status_manager.status {
            Status::Walking => self.update_walking_sprite(),
            Status::Pushing => self.update_pushing_sprite(),
            _ => (),
        }
        if self.status_manager.is_arrived_at_position() {
            if self.at_exit {
                self.set_status(Status::Exiting);
            } else {
                self.set_status(Status::Idle);
            }
        }
    }

    fn animate_dead (&mut self, audio: &mut Box<dyn AudioPlayer>) {
        self.update_dead_sprite();
        audio.play_sfx(SFX::Dead);
        if self.delta_time >= PLAYER_DEAD_ANIMATION_TIME {
            self.set_status(Status::Respawning);
            if self.is_falling {
                let new_position = self.status_manager.last_position;
                self.status_manager.set_position(new_position);
                self.is_falling = false;
            } else {
                let position = self.status_manager.position;
                self.status_manager.set_position(position);
            }
        }
    }

    fn animate_respawning (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.update_respawning_sprite();
        if self.delta_time >= PLAYER_RESPAWNING_ANIMATION_TIME {
            self.set_status(Status::Idle);
        }
    }

    fn animate_exiting (&mut self, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.update_exiting_sprite();
        audio.play_sfx(SFX::Fanfare);
        if self.delta_time >= PLAYER_EXITING_ANIMATION_TIME {
            self.set_status(Status::LevelComplete);
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

    fn update_dead_sprite(&mut self) {
        let sprite_dt = PLAYER_DEAD_ANIMATION_TIME / PLAYER_DEAD_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.delta_time / sprite_dt) as isize % PLAYER_DEAD_ANIMATION_SPRITE_LENGTH;
        // FIX: The hard coded offsets needs to be removed once formal assets are done.
        if dx == 0 {
            self.asset.set_x_offset(PLAYER_BASE_X_OFFSET + PLAYER_DEAD_X_OFFSET + dx as f64);
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
        self.set_status(Status::Pushing);
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

    fn handle_collision(&mut self, world: &World) {
        for object in world.get_objects().iter() {
            let mut object = object.borrow_mut();
            if object.is_projectile() {
                let is_collapsed = check_collision(object.status_manager(), &self.status_manager);
                if is_collapsed {
                    match self.status_manager.status {
                        Status::Idle | Status::Walking | Status::Pushing => {
                            self.set_status(Status::Dead);
                        },
                        _ => (),
                    }
                    object.set_visible(false);
                }
            }
        }
    }

    fn set_status(&mut self, new_status: Status) {
        self.delta_time = 0f64;
        self.status_manager.status = new_status;
    }
}