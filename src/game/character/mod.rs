use crate::audio::AudioPlayer;
use crate::game::{
    Asset,
    Direction,
    MovementManager,
    World,
};
mod player;
pub use self::player::Player;

pub trait Character {
    fn asset(&self) -> &Asset;
    fn movement_manager(&self) -> &MovementManager;
    fn update(&mut self, now: f64, world: &World, audio: &mut AudioPlayer);
    fn walk(&mut self, direction: Direction, world: &World);
    fn fall(&mut self);
}