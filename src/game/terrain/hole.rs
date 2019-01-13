use super::Terrain;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position, Status, World};
use crate::game::constants::{
    HOLE_X_OFFSET,
    HOLE_FILLED_X_OFFSET,
    HOLE_Y_OFFSET,
    HOLE_SIZE
};

pub struct Hole {
    asset: Asset,
    movement_manager: MovementManager,
    is_filled: bool,
}

impl Terrain for Hole {
    fn get_asset(&self) -> &Asset {
        &self.asset
    }
    fn movement_manager(&self) -> &MovementManager {
        &self.movement_manager
    }
    fn update(&mut self, _now: f64, world: &World) {
        if !self.is_filled {
            self.handle_falling(world);
        }
        if self.is_filled && self.asset.get_x_offset() != HOLE_FILLED_X_OFFSET {
            self.asset.set_x_offset(HOLE_FILLED_X_OFFSET);
        }
    }
}

impl Hole {
    pub fn new(position: Position) -> Hole {
        let asset = Asset::new(
            AssetType::Environment,
            HOLE_X_OFFSET,
            HOLE_Y_OFFSET,
            HOLE_SIZE,
            HOLE_SIZE,
        );
        let movement_manager = MovementManager::new(position, Direction::Down);
        Hole {
            asset,
            movement_manager,
            is_filled: false,
        }
    }

    fn handle_falling(&mut self, world: &World) {
        let object = world.get_object_by_position(&self.movement_manager.position);
        if let Some(object) = object {
            let mut object = object.borrow_mut();
            if object.movement_manager().status == Status::Idle {
                self.is_filled = true;
                object.set_visible(false);
            }
        }
    }
}