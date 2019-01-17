use crate::game::World;
use crate::controller::Controller;
use crate::renderer::Renderer;
use crate::audio::AudioPlayer;

mod game_scene;
mod entry_scene;
mod level_selection_scene;

pub use self::game_scene::GameScene;
pub use self::entry_scene::EntryScene;
pub use self::level_selection_scene::LevelSelectionScene;

#[derive(Clone, Copy)]
pub enum SceneType {
    Entry,
    LevelSelection,
    Game,
}

pub trait Scene {
    fn scene_type(&self) -> &SceneType;
    fn render(&self, renderer: &Renderer, world: &World);
    fn next_scene_type(&self) -> &Option<SceneType> {
        &None
    }
    fn set_next_scene_type(&mut self, _scene_type: SceneType) {}
    fn update(&mut self, _world: &mut World, _controller: &mut Controller, _audio: &mut Box<AudioPlayer>, _now: f64) {}
    fn on_mouse_up(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World) {}
}