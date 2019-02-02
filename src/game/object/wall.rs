use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, StatusManager, Position};
use crate::game::constants::{
    WALL_WIDTH,
    WALL_HEIGHT,
};

pub struct Wall {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
}

impl Object for Wall {
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

impl Wall {
    pub fn new(position: Position, asset: Asset, id: String) -> Wall {
        let status_manager = StatusManager::new(position, Direction::Down, WALL_WIDTH, WALL_HEIGHT);
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
            is_breakable: false,
            burning_level: 0,
            burn_down_time: 0f64,
            ignite_time: 0f64,
        };
        Wall {
            asset,
            status_manager,
            attribute_manager,
        }
    }
}