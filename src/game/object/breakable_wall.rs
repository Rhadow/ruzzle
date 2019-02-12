// use web_sys::console::log_1;
use super::{AttributeManager, Object};
use crate::audio::AudioPlayer;
use crate::game::{Asset, AssetType, Direction, Status, StatusManager, Position, World};
use crate::game::constants::{
    BREAKABLE_WALL_WIDTH,
    BREAKABLE_WALL_HEIGHT,
    SMOKE_ANIMATION_TIME,
    SMOKE_ANIMATION_SPRITE_LENGTH,
    SMOKE_BASE_X_OFFSET,
    SMOKE_BASE_Y_OFFSET,
    SMOKE_SIZE,
};

pub struct BreakableWall {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
}

impl Object for BreakableWall {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn update(&mut self, now: f64, _world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.status_manager.update_time(now);
        match self.status_manager.status {
            Status::Dead => {
                self.animate_dead(audio);
            }
            _ => (),
        }
    }
}

impl BreakableWall {
    pub fn new(position: Position, asset: Asset, id: String) -> BreakableWall {
        let status_manager = StatusManager::new(position, Direction::Down, BREAKABLE_WALL_WIDTH, BREAKABLE_WALL_HEIGHT);
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
            is_burnable: false,
            is_breakable: true,
            burning_level: 0,
            burn_down_time: 0f64,
            burning_point: 0f64,
            temperature: 0f64,
            heat: 1f64,
        };
        BreakableWall {
            asset,
            status_manager,
            attribute_manager,
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
        self.asset.set_asset_type(AssetType::RuzzleObject);
        self.asset.set_x_offset(SMOKE_BASE_X_OFFSET + (dx * 2) as f64);
        self.asset.set_y_offset(SMOKE_BASE_Y_OFFSET);
        self.asset.set_width(SMOKE_SIZE);
        self.asset.set_height(SMOKE_SIZE);
    }
}