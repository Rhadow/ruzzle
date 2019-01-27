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
    fn render(&self, renderer: &Renderer, world: &World, completed_levels: &Vec<bool>);
    fn next_scene_type(&self) -> &Option<SceneType> {
        &None
    }
    fn set_next_scene_type(&mut self, _scene_type: SceneType) {}
    fn update(&mut self, _world: &mut World, _controller: &mut Controller, _audio: &mut Box<AudioPlayer>, _completed_levels: &mut Vec<bool>, _now: f64) {}
    fn on_mouse_up(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World, _current_level_page: &mut usize) {}
    fn on_mouse_down(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World) {}
    fn is_mouse_inside_box(&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
        let mut result = false;
        if down_x >= x0 && down_x <= x1 && up_x >= x0 && up_x <= x1 &&
            down_y >= y0 && down_y <= y1 && up_y >= y0 && up_y <= y1 {
            result = true;
        }
        result
    }
}