use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, StatusManager, Position};
use crate::game::constants::{
    TILE_SIZE,
};

pub struct SpawningPoint {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
}

impl Object for SpawningPoint {
    fn asset(&mut self) -> &mut Asset {
        &mut self.asset
    }
    fn status_manager(&mut self) -> &mut StatusManager {
        &mut self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
}

impl SpawningPoint {
    pub fn new(position: Position, asset: Asset, id: String) -> SpawningPoint {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: true,
            can_fly_through: true,
            is_visible: true,
            is_pushable: false,
            is_filler: false,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: false,
            is_breakable: false,
            burning_level: 0,
            burn_down_time: 0f64,
            burning_point: 0f64,
            temperature: 0f64,
            heat: 1f64,
        };
        SpawningPoint {
            asset,
            status_manager,
            attribute_manager,
        }
    }
}