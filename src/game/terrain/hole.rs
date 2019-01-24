// use web_sys::console::log_1;
// log_1(&format!("{}", self.objects.len()).into());
use crate::audio::{SFX, AudioPlayer};
use super::Terrain;
use crate::game::{Asset, Direction, StatusManager, Position, World};
use crate::game::constants::{HOLE_X_OFFSET, HOLE_FILLED_X_OFFSET, TILE_SIZE};

pub struct Hole {
    delta_time: f64,
    time: f64,
    scheduled_falling_time: Option<f64>,
    asset: Asset,
    status_manager: StatusManager,
    is_filled: bool,
}

impl Terrain for Hole {
    fn asset(&self) -> &Asset {
        &self.asset
    }
    fn status_manager(&self) -> &StatusManager {
        &self.status_manager
    }
    fn set_falling_schedule(&mut self, dt: f64) {
        self.scheduled_falling_time = Some(dt);
    }
    fn is_filled(&self) -> bool {
        self.is_filled
    }
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        self.delta_time += now - self.time;
        self.time = now;
        if !self.is_filled {
            if self.asset.get_x_offset() != HOLE_X_OFFSET {
                self.asset.set_x_offset(HOLE_X_OFFSET);
            }
            if let Some(scheduled_time) = self.scheduled_falling_time {
                if self.delta_time >= scheduled_time {
                    self.handle_falling(world, audio);
                    self.delta_time = 0f64;
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
    pub fn new(position: Position, asset: Asset) -> Hole {
        let status_manager = StatusManager::new(position, Direction::Down, TILE_SIZE, TILE_SIZE);
        Hole {
            asset,
            status_manager,
            is_filled: false,
            scheduled_falling_time: None,
            time: 0f64,
            delta_time: 0f64,
        }
    }
    fn handle_falling(&mut self, world: &World, audio: &mut Box<dyn AudioPlayer>) {
        let object = world.get_object_by_position(&self.status_manager.position);
        let mut player = world.player().borrow_mut();
        if let Some(object) = object {
            let mut object = object.borrow_mut();
            let mut object_attribute = object.attribute_manager();
            if object_attribute.is_filler {
                self.is_filled = true;
            }
            object_attribute.is_visible = false;
            audio.play_sfx(SFX::RockFall);
        }
        if player.status_manager().position == self.status_manager.position {
            player.fall();
        }
        self.scheduled_falling_time = None;
    }
}