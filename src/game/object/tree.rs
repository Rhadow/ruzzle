use super::Object;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position, World};
use crate::game::constants::{
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE
};

pub struct Tree {
    asset: Asset,
    movement_manager: MovementManager,
}

impl Object for Tree {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
    fn step(&mut self, _direction: Direction, _world: &World) {}
    fn update(&mut self, _now: f64) {}
}

impl Tree {
    pub fn new(position: Position) -> Tree {
        let asset = Asset::new(
            AssetType::Object,
            TREE_X_OFFSET,
            TREE_Y_OFFSET,
            TREE_SIZE,
            TREE_SIZE,
        );
        let movement_manager = MovementManager::new(position, Direction::Down);
        Tree {
            asset,
            movement_manager,
        }
    }
}