use super::{AttributeManager, Object};
use crate::game::{Asset, Direction, StatusManager, Position};
use crate::game::character::Player;
use crate::game::constants::TILE_SIZE;

pub struct Exit {
    asset: Asset,
    status_manager: StatusManager,
    attribute_manager: AttributeManager,
}

impl Object for Exit {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn attribute_manager(&mut self) -> &mut AttributeManager {
        &mut self.attribute_manager
    }
    fn interact(&mut self, player: &mut Player) {
        self.attribute_manager.is_visible = false;
        player.at_exit = true;
    }
}

impl Exit {
    pub fn new(position: Position, asset: Asset, id: String) -> Exit {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        let attribute_manager = AttributeManager {
            id,
            can_step_on: true,
            can_fly_through: false,
            is_visible: true,
            is_pushable: false,
            is_filler: false,
            is_rotatable: false,
            is_projectile: false,
            is_projecting: false,
            is_burnable: false,
            burning_level: 0,
        };
        Exit {
            asset,
            status_manager,
            attribute_manager,
        }
    }
}