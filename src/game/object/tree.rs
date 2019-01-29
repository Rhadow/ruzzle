use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, Status, StatusManager, Position, World};
use crate::utils::check_collision;
use crate::audio::AudioPlayer;
use crate::game::constants::{
    TILE_SIZE,
    TREE_BURNING_X_OFFSET,
    TREE_BURNING_Y_OFFSET,
    TREE_BURNING_END_X_OFFSET,
    TREE_BURNING_END_Y_OFFSET,
    TREE_ASH_X_OFFSET,
    TREE_ASH_Y_OFFSET,
    MAX_BURNING_LEVEL,
    TREE_BURN_DOWN_TIME,
};

pub struct Tree {
    asset: Asset,
    status_manager: StatusManager,
    pub attribute_manager: AttributeManager,
}

impl Object for Tree {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn update(&mut self, now: f64, world: &World, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        match self.status_manager.status {
            Status::Idle => {
                if self.attribute_manager.burning_level > 0 {
                    self.animate_burning();
                    self.handle_burning_logic(world);
                } else {
                    self.status_manager.delta_time = 0f64;
                }
            }
            _ => (),
        }
    }
}

impl Tree {
    pub fn new(position: Position, asset: Asset, id: String) -> Tree {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: false,
            can_fly_through: false,
            is_visible: true,
            is_pushable: false,
            is_filler: true,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: true,
            is_breakable: false,
            burning_level: 0,
            burn_down_time: TREE_BURN_DOWN_TIME,
        };
        Tree {
            attribute_manager,
            asset,
            status_manager,
        }
    }
    fn animate_burning(&mut self) {
        match self.attribute_manager.burning_level {
            1 | 2 => {
                self.asset.set_x_offset(TREE_BURNING_X_OFFSET);
                self.asset.set_y_offset(TREE_BURNING_Y_OFFSET);
            },
            3 => {
                self.asset.set_x_offset(TREE_BURNING_END_X_OFFSET);
                self.asset.set_y_offset(TREE_BURNING_END_Y_OFFSET);
            },
            _ => (),
        }
    }

    fn handle_burning_logic(&mut self, world: &World) {
        let dt_per_burning_level = self.attribute_manager.burn_down_time / MAX_BURNING_LEVEL as f64;
        for object in world.get_objects() {
            if object.try_borrow_mut().is_err() {
                continue;
            }
            let mut object = object.borrow_mut();
            let is_object_burnable = object.attribute_manager().is_burnable;
            let is_object_visible = object.attribute_manager().is_visible;
            let is_object_burning = object.attribute_manager().burning_level > 0;
            let is_object_projectile = object.attribute_manager().is_projectile;
            if is_object_visible {
                let is_collided = check_collision(object.status_manager(), &self.status_manager);
                if is_collided && is_object_burnable && !is_object_burning {
                    if is_object_projectile {
                        object.attribute_manager().burning_level = 1;
                    } else {
                        let object_dt_per_burning_level = object.attribute_manager().burn_down_time / MAX_BURNING_LEVEL as f64;
                        if self.status_manager.delta_time > object_dt_per_burning_level {
                            object.attribute_manager().burning_level = 1;
                            object.status_manager().delta_time = 0f64;
                        }
                    }

                }
            }
        }
        if self.status_manager.delta_time > dt_per_burning_level && self.attribute_manager.burning_level < MAX_BURNING_LEVEL {
            self.attribute_manager.burning_level += 1;
            self.status_manager.delta_time = 0f64;
            if self.attribute_manager.burning_level >= MAX_BURNING_LEVEL {
                self.attribute_manager.is_visible = false;
            }
        }
    }
}