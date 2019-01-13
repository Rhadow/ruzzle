use super::Object;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position, World};
use crate::game::constants::{
    TREE_X_OFFSET,
    TREE_Y_OFFSET,
    TREE_SIZE
};

pub struct Tree {
    asset: Asset,
    is_visible: bool,
    movement_manager: MovementManager,
}

impl Object for Tree {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn is_visible(&self) -> bool {
        self.is_visible
    }
    fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
    fn can_move_to(&self, _direction: &Direction, _world: &World) -> bool {
        false
    }
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
            is_visible: true,
            asset,
            movement_manager,
        }
    }
}