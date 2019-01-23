use super::Object;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::character::Player;
use crate::game::constants::TILE_SIZE;

pub struct Exit {
    asset: Asset,
    is_visible: bool,
    status_manager: StatusManager,
}

impl Object for Exit {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn can_move_to(&self, _direction: &Direction, _world: &World) -> bool {
        false
    }
    fn can_step_on(&self) -> bool {
        true
    }
    fn interact(&mut self, player: &mut Player) {
        self.is_visible = false;
        player.at_exit = true;
    }
}

impl Exit {
    pub fn new(position: Position, asset: Asset) -> Exit {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        Exit {
            is_visible: true,
            asset,
            status_manager,
        }
    }
}