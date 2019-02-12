use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, Status, StatusManager, Position, World};
use crate::audio::AudioPlayer;
use crate::game::constants::{
    TILE_SIZE,
    TREE_BURNING_X_OFFSET,
    TREE_BURNING_Y_OFFSET,
    TREE_BURNING_ANIMATION_TIME,
    TREE_BURNING_ANIMATION_SPRITE_LENGTH,
    TREE_BURNING_END_X_OFFSET,
    TREE_BURNING_END_Y_OFFSET,
    TREE_BURN_DOWN_TIME,
    TREE_IGNITE_FRAMES,
};

pub struct Tree {
    asset: Asset,
    status_manager: StatusManager,
    pub attribute_manager: AttributeManager,
}

impl Object for Tree {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn update(&mut self, now: f64, world: &World, _audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        if self.attribute_manager.burning_level > 0 {
            self.handle_fire_logic(world);
            self.animate_burning();
        } else {
            self.status_manager.animation_timer = 0f64;
        }
        match self.status_manager.status {
            Status::Idle => {
                self.status_manager.delta_time = 0f64;
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
            burning_point: TREE_IGNITE_FRAMES,
            temperature: 0f64,
            heat: 1f64,
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
                self.animate_flame();
            },
            3 => {
                self.asset.set_x_offset(TREE_BURNING_END_X_OFFSET);
                self.asset.set_y_offset(TREE_BURNING_END_Y_OFFSET);
            },
            _ => (),
        }
    }
    fn animate_flame (&mut self) {
        let time_per_sprite = TREE_BURNING_ANIMATION_TIME / TREE_BURNING_ANIMATION_SPRITE_LENGTH as f64;
        let dx = (self.status_manager.animation_timer / time_per_sprite) as isize % TREE_BURNING_ANIMATION_SPRITE_LENGTH;
        self.asset.set_x_offset(TREE_BURNING_X_OFFSET + dx as f64 * 2f64);
        self.asset.set_y_offset(TREE_BURNING_Y_OFFSET);
    }
}