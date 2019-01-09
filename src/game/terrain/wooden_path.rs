use super::Terrain;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    WOODEN_PATH_X_OFFSET,
    WOODEN_PATH_Y_OFFSET,
    WOODEN_PATH_SIZE
};

pub struct WoodenPath {
    asset_type: AssetType,
    asset_x_offset: f64,
    asset_y_offset: f64,
    asset_width: f64,
    asset_height: f64,
}

impl Asset for WoodenPath {
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

impl Terrain for WoodenPath {
    fn update(&self) {}
}

impl WoodenPath {
    pub fn new() -> WoodenPath {
        WoodenPath {
            asset_type: AssetType::Environment,
            asset_x_offset: WOODEN_PATH_X_OFFSET,
            asset_y_offset: WOODEN_PATH_Y_OFFSET,
            asset_width: WOODEN_PATH_SIZE,
            asset_height: WOODEN_PATH_SIZE,
        }
    }
}