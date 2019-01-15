use crate::game::{Asset, MovementManager, World};
use crate::audio::AudioPlayer;
pub mod grassland;
pub mod wooden_path;
pub mod hole;

pub use self::grassland::GrassLand;
pub use self::wooden_path::WoodenPath;
pub use self::hole::Hole;

pub trait Terrain {
    fn get_asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn set_falling_schedule(&mut self, _dt: f64) {}
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut AudioPlayer) {}
    fn is_filled(&self) -> bool {
        true
    }
}
