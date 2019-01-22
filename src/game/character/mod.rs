use crate::audio::AudioPlayer;
use crate::game::{
    Asset,
    Direction,
    StatusManager,
    World,
};
mod player;
pub use self::player::Player;

pub trait Character {
    fn asset(&self) -> &Asset;
    fn status_manager(&self) -> &StatusManager;
    fn update(&mut self, now: f64, world: &World, audio: &mut Box<dyn AudioPlayer>);
    fn at_exit(&self) -> bool;
    fn walk(&mut self, direction: Direction, world: &World);
    fn fall(&mut self);
    fn rotate_item(&mut self, world: &World);
}