use super::Object;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE
};

pub struct Tree {
    asset_type: AssetType,
    asset_x_offset: f64,
    asset_y_offset: f64,
    asset_width: f64,
    asset_height: f64,
}

impl Asset for Tree {
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

impl Object for Tree {
    fn update(&self) {}
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            asset_type: AssetType::Object,
            asset_x_offset: TREE_X_OFFSET,
            asset_y_offset: TREE_Y_OFFSET,
            asset_width: TREE_SIZE,
            asset_height: TREE_SIZE,
        }
    }
}