use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::game::constants::{
    TILE_SIZE,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
    START_BUTTON_WIDTH,
    START_BUTTON_HEIGHT,
};

pub struct EntryScene {
    scene_type: SceneType,
    width: f64,
    height: f64,
    next_scene_type: Option<SceneType>
}

impl Scene for EntryScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, _world: &World) {
        let start_btn_x = self.width / 2f64 - START_BUTTON_WIDTH / 2f64;
        let start_btn_y = self.height / 2f64 - START_BUTTON_HEIGHT / 2f64;
        renderer.clear_screen();
        renderer.draw_rectangle(0f64, 0f64, self.width, self.height, &JsValue::from_str("#01cafe"));
        renderer.draw_rectangle(start_btn_x, start_btn_y, START_BUTTON_WIDTH, START_BUTTON_HEIGHT, &JsValue::from_str("#ffffff"));
    }
    fn on_mouse_up(&mut self, mouse_x: f64, mouse_y: f64, world: &mut World) {
        log_1(&format!("{}, {}", mouse_x, mouse_y).into());
        self.set_next_scene_type(SceneType::Game);
        world.init_level(0);
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
            width: WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE,
            height: WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE,
            next_scene_type: None,
        }
    }
}