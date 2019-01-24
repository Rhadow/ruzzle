use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, StatusManager, Position, World};
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
    fn can_move_to(&self, _direction: &Direction, _world: &World) -> bool {
        false
    }
}

impl Wall {
    pub fn new(position: Position, asset: Asset, id: String) -> Wall {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            is_visible: true,
            can_step_on: false,
            is_pushable: false,
            is_filler: true,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
        };
        Wall {
            attribute_manager,
            asset,
            status_manager,
        }
    }
}