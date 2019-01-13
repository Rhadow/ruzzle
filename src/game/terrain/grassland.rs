use super::Terrain;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position};
use crate::game::constants::{
    GRASS_LAND_X_OFFSET,
    GRASS_LAND_Y_OFFSET,
    GRASS_LAND_SIZE
};

pub struct GrassLand {
    asset: Asset,
    movement_manager: MovementManager,
}

impl Terrain for GrassLand {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
}

impl GrassLand {
    pub fn new(position: Position) -> GrassLand {
        let asset = Asset::new(
            AssetType::Environment,
            GRASS_LAND_X_OFFSET,
            GRASS_LAND_Y_OFFSET,
            GRASS_LAND_SIZE,
            GRASS_LAND_SIZE,
        );
        let movement_manager = MovementManager::new(position, Direction::Down);
        GrassLand {
            asset,
            movement_manager
        }
    }
}