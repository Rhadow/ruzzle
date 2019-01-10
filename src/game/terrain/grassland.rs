use super::Terrain;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    GRASS_LAND_X_OFFSET,
    GRASS_LAND_Y_OFFSET,
    GRASS_LAND_SIZE
};

pub struct GrassLand {
    asset: Asset,
}

impl Terrain for GrassLand {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn update(&self) {}
}

impl GrassLand {
    pub fn new() -> GrassLand {
        let asset = Asset::new(
            AssetType::Environment,
            GRASS_LAND_X_OFFSET,
            GRASS_LAND_Y_OFFSET,
            GRASS_LAND_SIZE,
            GRASS_LAND_SIZE,
        );
        GrassLand {
            asset
        }
    }
}