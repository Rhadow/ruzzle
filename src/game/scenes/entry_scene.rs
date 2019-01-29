// use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::controller::Controller;
use crate::audio::AudioPlayer;
use crate::game::World;
use crate::game::constants::{
    TILE_SIZE,
    WINDOW_WIDTH_IN_TILES,
    WINDOW_HEIGHT_IN_TILES,
    START_BUTTON_WIDTH,
    START_BUTTON_HEIGHT,
    START_BUTTON_FLASH_FREQUENCY,
};

pub struct EntryScene {
    scene_type: SceneType,
    width: f64,
    height: f64,
    next_scene_type: Option<SceneType>,
    time: f64,
    delta_time: f64,
}

impl Scene for EntryScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn update(&mut self, _world: &mut World, _controller: &mut Controller, _audio: &mut Box<AudioPlayer>, _completed_levels: &mut Vec<bool>, now: f64) {
        if self.time != 0f64 {
            self.delta_time += now - self.time;
        }
        self.time = now;
        if self.delta_time >= START_BUTTON_FLASH_FREQUENCY * 1.5f64 {
            self.delta_time = 0f64;
        }
    }
    fn render(&self, renderer: &Renderer, _world: &World, _completed_levels: &Vec<bool>) {
        let start_btn_x = self.width / 2f64 - START_BUTTON_WIDTH / 2f64;
        let start_btn_y = self.height / 2f64 - START_BUTTON_HEIGHT / 2f64;
        renderer.clear_screen();
        renderer.draw_rectangle(0f64, 0f64, self.width, self.height, &JsValue::from_str("#01cafe"));
        if self.delta_time < START_BUTTON_FLASH_FREQUENCY {
            renderer.draw_rectangle(start_btn_x, start_btn_y, START_BUTTON_WIDTH, START_BUTTON_HEIGHT, &JsValue::from_str("#ffffff"));
        }
    }
    fn on_mouse_up(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World, _current_level_page: &mut usize) {
        self.set_next_scene_type(SceneType::LevelSelection);
    }
    fn next_scene_type(&self) -> &Option<SceneType> {
        &self.next_scene_type
    }
    fn set_next_scene_type(&mut self, scene_type: SceneType) {
        self.next_scene_type = Some(scene_type);
    }
}

impl EntryScene {
    pub fn new() -> EntryScene {
        EntryScene {
            scene_type: SceneType::Entry,
            width: WINDOW_WIDTH_IN_TILES as f64 * TILE_SIZE,
            height: WINDOW_HEIGHT_IN_TILES as f64 * TILE_SIZE,
            next_scene_type: None,
            time: 0f64,
            delta_time: 0f64,
        }
    }
}