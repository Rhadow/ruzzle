use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, StatusManager, Position};
use crate::game::constants::TILE_SIZE;

pub struct Wall {
    asset: Asset,
    status_manager: StatusManager,
    pub attribute_manager: AttributeManager,
}

impl Object for Wall {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
}

impl Wall {
    pub fn new(position: Position, asset: Asset, id: String) -> Wall {
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
            is_burnable: false,
            burning_level: 0,
        };
        Wall {
            attribute_manager,
            asset,
            status_manager,
        }
    }
}