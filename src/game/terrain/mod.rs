use crate::game::{Asset, StatusManager, World};
use crate::audio::AudioPlayer;
pub mod land;
pub mod hole;

pub use self::land::Land;
pub use self::hole::Hole;

pub trait Terrain {
    fn asset(&self) -> &Asset;
    fn status_manager(&self) -> &StatusManager;
    fn update(&mut self, _now: f64, _world: &World, _audio: &mut Box<dyn AudioPlayer>) {}
    fn is_filled(&self) -> bool {
        true
    }
}
