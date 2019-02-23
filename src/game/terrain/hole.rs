// use web_sys::console::log_1;
// log_1(&format!("{}", self.objects.len()).into());
use std::cell::RefCell;
use crate::audio::{AudioPlayer};
use super::Terrain;
use crate::game::{Asset, Direction, Status, StatusManager, Position, World};
use crate::game::constants::{HOLE_X_OFFSET, HOLE_FILLED_X_OFFSET, TILE_SIZE, HOLE_FALL_THRESHOLD};
use crate::game::character::Character;
use crate::game::object::Object;
use crate::utils::get_object_coverage;

pub struct Hole {
    asset: Asset,
    status_manager: StatusManager,
    is_filled: bool,
}

impl Terrain for Hole {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn is_filled(&self) -> bool {
        self.is_filled
    }
    fn update(&mut self, _now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        if !self.is_filled {
            if self.asset.get_x_offset() != HOLE_X_OFFSET {
                self.asset.set_x_offset(HOLE_X_OFFSET);
            }
            for obj in world.get_objects().iter() {
                let mut is_object_projectile = false;
                {
                    let mut object = obj.borrow_mut();
                    is_object_projectile = object.attribute_manager().is_projectile;
                }
                if get_object_coverage(&self.status_manager, obj.borrow_mut().status_manager()) >= HOLE_FALL_THRESHOLD && !is_object_projectile {
                    self.handle_object_falling(obj, audio);
                }
            }
            for character in world.get_characters().iter() {
                if get_object_coverage(&self.status_manager, character.borrow_mut().status_manager()) >= HOLE_FALL_THRESHOLD {
                    self.handle_character_falling(character, audio);
                }
            }
        } else {
            if self.asset.get_x_offset() != HOLE_FILLED_X_OFFSET {
                self.asset.set_x_offset(HOLE_FILLED_X_OFFSET);
            }
        }
    }
}

impl Hole {
    pub fn new(position: Position, asset: Asset) -> Hole {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        Hole {
            asset,
            status_manager,
            is_filled: false,
        }
    }
    fn handle_character_falling(&mut self, character: &RefCell<Box<dyn Character>>, audio: &mut Box<dyn AudioPlayer>) {
        let mut character = character.borrow_mut();
        let character_status = character.status_manager().status;
        let is_character_recovering = character_status == Status::Dead || character_status == Status::Respawning;
        if !is_character_recovering {
            character.handle_fall(audio);
        }
    }
    fn handle_object_falling(&mut self, object: &RefCell<Box<dyn Object>>, audio: &mut Box<dyn AudioPlayer>) {
        let mut object = object.borrow_mut();
        {
            let object_attribute = object.attribute_manager();
            if object_attribute.is_filler {
                self.is_filled = true;
            }
        }
        object.handle_fall(audio);
    }
}