use super::Terrain;
use crate::game::{Asset, Direction, StatusManager, Position};

pub struct Land {
    asset: Asset,
    status_manager: StatusManager,
}

impl Terrain for Land {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
}

impl Land {
    pub fn new(position: Position, asset: Asset) -> Land {
        let status_manager = StatusManager::new(position, Direction::Down);
        Land {
            asset,
            status_manager
        }
    }
}