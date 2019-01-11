use super::Object;
use crate::game::{Asset, AssetType, Coordinate, Position};
use crate::game::constants::{
    CELL_SIZE,
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE
};

pub struct Tree {
    asset: Asset,
    position: Position,
    coordinate: Coordinate
}

impl Object for Tree {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
    fn get_position(&self) -> &Position {
        &self.position
    }
    fn update(&self) {}
}

impl Tree {
    pub fn new(position: Position) -> Tree {
        let coordinate = Coordinate(position.col() * CELL_SIZE, position.row() * CELL_SIZE);
        let asset = Asset::new(
            AssetType::Object,
            TREE_X_OFFSET,
            TREE_Y_OFFSET,
            TREE_SIZE,
            TREE_SIZE,
        );
        Tree {
            asset,
            position,
            coordinate,
        }
    }
}