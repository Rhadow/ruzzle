use super::Terrain;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    WOODEN_PATH_X_OFFSET,
    WOODEN_PATH_Y_OFFSET,
    WOODEN_PATH_SIZE
};

pub struct WoodenPath {
    asset: Asset,
}

impl Terrain for WoodenPath {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn update(&self) {}
}

impl WoodenPath {
    pub fn new() -> WoodenPath {
        let asset = Asset::new(
            AssetType::Environment,
            WOODEN_PATH_X_OFFSET,
            WOODEN_PATH_Y_OFFSET,
            WOODEN_PATH_SIZE,
            WOODEN_PATH_SIZE,
        );
        WoodenPath {
            asset
        }
    }
}