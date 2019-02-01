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
    fn on_mouse_move(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World) {}
    fn is_mouse_inside_box(&self, x: f64, y: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
        let mut result = false;
        if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
            result = true;
        }
        result
    }
    fn is_pressed_inside_box(&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> bool {
        self.is_mouse_inside_box(down_x, down_y, x0, y0, x1, y1) && self.is_mouse_inside_box(up_x, up_y, x0, y0, x1, y1)
    }
}