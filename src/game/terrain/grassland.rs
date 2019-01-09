use super::Terrain;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    GRASS_LAND_X_OFFSET,
    GRASS_LAND_Y_OFFSET,
    GRASS_LAND_SIZE
};

pub struct GrassLand {
    asset_type: AssetType,
    asset_x_offset: f64,
    asset_y_offset: f64,
    asset_width: f64,
    asset_height: f64,
}

impl Asset for GrassLand {
    fn get_asset_type(&self) -> &AssetType {
        &self.asset_type
    }

    fn get_asset_x_offset(&self) -> f64 {
        self.asset_x_offset
    }

    fn get_asset_y_offset(&self) -> f64 {
        self.asset_y_offset
    }

    fn get_asset_width(&self) -> f64 {
        self.asset_width
    }

    fn get_asset_height(&self) -> f64 {
        self.asset_height
    }
}

impl Terrain for GrassLand {
    fn update(&self) {}
}

impl GrassLand {
    pub fn new() -> GrassLand {
        GrassLand {
            asset_type: AssetType::Environment,
            asset_x_offset: GRASS_LAND_X_OFFSET,
            asset_y_offset: GRASS_LAND_Y_OFFSET,
            asset_width: GRASS_LAND_SIZE,
            asset_height: GRASS_LAND_SIZE,
        }
    }
}