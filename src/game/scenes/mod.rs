use crate::game::World;
use crate::canvas::CanvasKeyMap;
use crate::renderer::Renderer;
use crate::audio::AudioPlayer;

mod game_scene;

pub use self::game_scene::GameScene;

pub enum SceneType {
    Entry,
    LevelSelection,
    Game,
}

pub trait Scene {
    fn scene_type(&self) -> &SceneType;
    fn render(&self, renderer: &Renderer, world: &World);
    fn update(&mut self, world: &mut World, key_map: &mut CanvasKeyMap, audio: &mut AudioPlayer, now: f64);
}