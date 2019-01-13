use super::Terrain;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position};
use crate::game::constants::{
    WOODEN_PATH_X_OFFSET,
    WOODEN_PATH_Y_OFFSET,
    WOODEN_PATH_SIZE
};

pub struct WoodenPath {
    asset: Asset,
    movement_manager: MovementManager,
}

impl Terrain for WoodenPath {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
}

impl WoodenPath {
    pub fn new(position: Position) -> WoodenPath {
        let asset = Asset::new(
            AssetType::Environment,
            WOODEN_PATH_X_OFFSET,
            WOODEN_PATH_Y_OFFSET,
            WOODEN_PATH_SIZE,
            WOODEN_PATH_SIZE,
        );
        let movement_manager = MovementManager::new(position, Direction::Down);
        WoodenPath {
            asset,
            movement_manager,
        }
    }
}