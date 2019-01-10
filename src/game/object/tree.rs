use super::Object;
use crate::game::{Asset, AssetType, Coordinate};
use crate::game::constants::{
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE
};

pub struct Tree {
    asset: Asset,
    coordinate: Coordinate
}

impl Object for Tree {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
    fn update(&self) {}
}

impl Tree {
    pub fn new(coordinate: Coordinate) -> Tree {
        let asset = Asset::new(
            AssetType::Object,
            TREE_X_OFFSET,
            TREE_Y_OFFSET,
            TREE_SIZE,
            TREE_SIZE,
        );
        Tree {
            asset,
            coordinate
        }
    }
}