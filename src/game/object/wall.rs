use super::Object;
use crate::game::{Asset, Direction, StatusManager, Position, World};

pub struct Wall {
    asset: Asset,
    is_visible: bool,
    status_manager: StatusManager,
}

impl Object for Wall {
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
}

impl Wall {
    pub fn new(position: Position, asset: Asset) -> Wall {
        let status_manager = StatusManager::new(position, Direction::Down);
        Wall {
            is_visible: true,
            asset,
            status_manager,
        }
    }
}