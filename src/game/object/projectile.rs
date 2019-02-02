// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::AudioPlayer;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::status_manager::Status;
use crate::utils::check_collision;
use crate::game::constants::{
    PROJECTILE_SIZE,
    PROJECTILE_MOVE_TIME,
    PROJECTILE_BURNING_X_OFFSET,
    PROJECTILE_BURNING_Y_OFFSET,
    SMOKE_ANIMATION_TIME,
    SMOKE_ANIMATION_SPRITE_LENGTH,
    SMOKE_BASE_X_OFFSET,
    SMOKE_BASE_Y_OFFSET,
    SMOKE_SIZE,
};

pub struct Projectile {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
    projector_id: String,
}

impl Object for Projectile {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        if self.attribute_manager.burning_level == 1 {
            self.asset.set_x_offset(PROJECTILE_BURNING_X_OFFSET);
            self.asset.set_y_offset(PROJECTILE_BURNING_Y_OFFSET);
            self.attribute_manager.burning_level += 1;
        }
        match self.status_manager.status {
            Status::Idle => {
                let direction = self.status_manager.direction.clone();
                self.status_manager.delta_time = 0f64;
                self.walk(direction, world);
            }
            Status::Walking => {
                self.animate_walking(audio);
                self.handle_walking_logic(world, audio);
            },
            Status::Dead => {
                self.animate_dead(audio);
            },
            _ => (),
        }
    }
}

impl Projectile {
    pub fn new(position: Position, asset: Asset, direction: Direction, projector_id: String, id: String) -> Projectile {
        let status_manager = StatusManager::new(position, direction, PROJECTILE_SIZE / 2f64, PROJECTILE_SIZE / 2f64);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: false,
            is_visible: true,
            is_pushable: false,
            is_filler: false,
            is_rotatable: false,
            is_projectile: true,
            is_projecting: false,
            is_burnable: true,
            is_breakable: true,
            burning_level: 0,
            burn_down_time: 0f64,
            ignite_time: 0f64,
        };
        Projectile {
            asset,
            status_manager,
            attribute_manager,
            projector_id,
        }
    }
    fn animate_walking (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        let delta_time = self.status_manager.delta_time;
        self.status_manager.set_next_coordinate(delta_time, PROJECTILE_MOVE_TIME);
        if self.status_manager.is_arrived_at_position() {
            self.status_manager.set_status(Status::Idle);
        }
    }
    fn handle_walking_logic(&mut self, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.handle_object_collision(world);
        if self.status_manager.status != Status::Dead {
            self.handle_character_collision(world, audio);
        }
        if !self.status_manager.coordinate.is_in_tile_map() {
            self.attribute_manager.is_visible = false;
        }
    }
    fn animate_dead (&mut self, _audio: &mut Box<dyn AudioPlayer>) {
        self.update_dead_sprite();
        if self.status_manager.delta_time >= SMOKE_ANIMATION_TIME {
            self.attribute_manager.is_visible = false;
        }
    }
    fn update_dead_sprite(&mut self) {
        let sprite_dt = SMOKE_ANIMATION_TIME / SMOKE_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.delta_time / sprite_dt) as isize % SMOKE_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(SMOKE_BASE_X_OFFSET + (dx * 2) as f64);
        self.asset.set_y_offset(SMOKE_BASE_Y_OFFSET);
        self.asset.set_width(SMOKE_SIZE);
        self.asset.set_height(SMOKE_SIZE);
    }

    fn handle_character_collision(&mut self, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        for character in world.get_characters() {
            let mut character = character.borrow_mut();
            let is_collided = check_collision(character.status_manager(), &self.status_manager);
            if is_collided {
                character.handle_death(audio);
                self.status_manager.set_status(Status::Dead);
            }
        }
    }

    fn handle_object_collision(&mut self, world: &World) {
        for object in world.get_objects() {
            if object.try_borrow_mut().is_err() {
                continue;
            }
            let mut object = object.borrow_mut();
            let is_object_breakable = object.attribute_manager().is_breakable;
            let is_object_visible = object.attribute_manager().is_visible;
            let is_object_burnable = object.attribute_manager().is_burnable;
            let is_object_burning = object.attribute_manager().burning_level > 0;
            let object_id = object.attribute_manager().id.clone();
            let is_object_parent_projector = object_id.to_string() == self.projector_id;
            let is_collided = check_collision(object.status_manager(), &self.status_manager);
            let can_fly_through = object.attribute_manager().can_fly_through;
            if is_collided && !is_object_parent_projector && is_object_visible {
                if is_object_breakable {
                    object.status_manager().set_status(Status::Dead);
                }
                if self.attribute_manager.burning_level > 0 && is_object_burnable && !is_object_burning {
                    object.attribute_manager().burning_level += 1;
                    let object_status = object.status_manager().status;
                    if object_status != Status::Walking {
                        object.status_manager().delta_time = 0f64;
                    }
                }
                if !can_fly_through {
                    self.status_manager.set_status(Status::Dead);
                }
            }
        }
    }
}