// use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::game::level::LEVELS;
use crate::game::constants::{
    TILE_SIZE,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
    LEVEL_BUTTON_WIDTH,
    LEVEL_BUTTON_HEIGHT,
    LEVEL_BUTTON_MARGIN,
    LEVELS_PER_PAGE,
    ROW_PER_PAGE,
};

pub struct LevelSelectionScene {
    scene_type: SceneType,
    width: f64,
    height: f64,
    next_scene_type: Option<SceneType>,
    current_page: usize,
}

impl Scene for LevelSelectionScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, _world: &World) {
        renderer.clear_screen();
        renderer.draw_rectangle(0f64, 0f64, self.width, self.height, &JsValue::from_str("#0d9263"));
        let horizontal_padding = (self.width - (LEVELS_PER_PAGE as f64 / ROW_PER_PAGE as f64) * (LEVEL_BUTTON_WIDTH + LEVEL_BUTTON_MARGIN)) / 2f64;
        let vertical_padding = (self.height - (ROW_PER_PAGE as f64) * (LEVEL_BUTTON_HEIGHT + LEVEL_BUTTON_MARGIN)) / 2f64;
        let levels = self.get_levels_by_page();
        for (index, _level) in levels.iter().enumerate() {
            let x = horizontal_padding + (index % (LEVELS_PER_PAGE / ROW_PER_PAGE)) as f64 * (LEVEL_BUTTON_WIDTH + LEVEL_BUTTON_MARGIN);
            let y = vertical_padding + (index as isize / (LEVELS_PER_PAGE / ROW_PER_PAGE) as isize) as f64 * (LEVEL_BUTTON_HEIGHT + LEVEL_BUTTON_MARGIN);
            renderer.draw_rectangle(x, y, LEVEL_BUTTON_WIDTH, LEVEL_BUTTON_HEIGHT, &JsValue::from_str("#d4ce46"));
        }
    }
    fn on_mouse_up(&mut self, _mouse_x: f64, _mouse_y: f64, _world: &mut World) {
        self.set_next_scene_type(SceneType::Entry);
    }
    fn next_scene_type(&self) -> &Option<SceneType> {
        &self.next_scene_type
    }
    fn set_next_scene_type(&mut self, scene_type: SceneType) {
        self.next_scene_type = Some(scene_type);
    }
}

impl LevelSelectionScene {
    pub fn new() -> LevelSelectionScene {
        LevelSelectionScene {
            scene_type: SceneType::LevelSelection,
            width: WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE,
            height: WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE,
            next_scene_type: None,
            current_page: 0,
        }
    }

    fn get_levels_by_page(&self) -> Vec<usize> {
        let start = self.current_page * LEVELS_PER_PAGE;
        let mut end = start + LEVELS_PER_PAGE - 1;
        if end >= LEVELS.len() {
            end = LEVELS.len() - 1;
        }
        (start..=end).collect()
    }
}