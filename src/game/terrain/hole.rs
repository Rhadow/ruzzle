// use web_sys::console::log_1;
// log_1(&format!("{}", self.objects.len()).into());
use super::Terrain;
use crate::game::{Asset, AssetType, Direction, MovementManager, Position, World};
use crate::game::constants::{
    HOLE_X_OFFSET,
    HOLE_FILLED_X_OFFSET,
    HOLE_Y_OFFSET,
    HOLE_SIZE
};

pub struct Hole {
    delta_time: f64,
    time: f64,
    scheduled_falling_time: Option<f64>,
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
    fn set_falling_schedule(&mut self, dt: f64) {
        self.scheduled_falling_time = Some(dt);
    }
    fn is_filled(&self) -> bool {
        self.is_filled
    }
    fn update(&mut self, now: f64, world: &World) {
        self.delta_time += now - self.time;
        self.time = now;
        if !self.is_filled {
            if self.asset.get_x_offset() != HOLE_X_OFFSET {
                self.asset.set_x_offset(HOLE_X_OFFSET);
            }
            if let Some(scheduled_time) = self.scheduled_falling_time {
                if self.delta_time >= scheduled_time {
                    self.handle_falling(world);
                }
            } else {
                self.delta_time = 0f64;
            }
        } else {
            if self.asset.get_x_offset() != HOLE_FILLED_X_OFFSET {
                self.asset.set_x_offset(HOLE_FILLED_X_OFFSET);
            }
            self.delta_time = 0f64;
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
            scheduled_falling_time: None,
            time: 0f64,
            delta_time: 0f64,
        }
    }
    fn handle_falling(&mut self, world: &World) {
        let object = world.get_object_by_position(&self.movement_manager.position);
        if let Some(object) = object {
            let mut object = object.borrow_mut();
            self.is_filled = true;
            object.set_visible(false);
        }
    }
}