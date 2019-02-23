// use web_sys::console::log_1;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::controller::Controller;
use crate::audio::AudioPlayer;
use crate::game::World;
use crate::game::{Asset, AssetType};
use crate::game::constants::{
    TILE_SIZE,
    WINDOW_WIDTH_IN_TILES,
    WINDOW_HEIGHT_IN_TILES,
    START_BUTTON_X_OFFSET,
    START_BUTTON_Y_OFFSET,
    START_BUTTON_WIDTH,
    START_BUTTON_HEIGHT,
    START_BUTTON_FLASH_FREQUENCY,
    TITLE_X_OFFSET,
    TITLE_Y_OFFSET,
    TITLE_WIDTH,
    TITLE_HEIGHT,
    BACKGROUND_X_OFFSET,
    BACKGROUND_Y_OFFSET,
    BACKGROUND_WIDTH,
    BACKGROUND_HEIGHT,
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
        renderer.clear_screen();
        self.draw_background(renderer);
        self.draw_title(renderer);
        self.draw_hint(renderer);
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
    fn draw_background(&self, renderer: &Renderer) {
        let asset = Asset::new(
            AssetType::RuzzleBackground,
            BACKGROUND_X_OFFSET,
            BACKGROUND_Y_OFFSET,
            BACKGROUND_WIDTH,
            BACKGROUND_HEIGHT,
            None,
            None,
        );
        renderer.draw_asset_by_coordinate(&asset, 0f64, 0f64, BACKGROUND_WIDTH, BACKGROUND_HEIGHT);
    }
    fn draw_title(&self, renderer: &Renderer) {
        let asset = Asset::new(
            AssetType::RuzzleUI,
            TITLE_X_OFFSET,
            TITLE_Y_OFFSET,
            TITLE_WIDTH,
            TITLE_HEIGHT,
            None,
            None,
        );
        let title_render_width = TITLE_WIDTH * 2.5f64;
        let title_render_height = TITLE_HEIGHT * 2.5f64;
        let x = (self.width - title_render_width) / 2f64;
        let y = self.height * 0.15f64;
        renderer.draw_asset_by_coordinate(&asset, x, y, title_render_width, title_render_height);
    }
    fn draw_hint(&self, renderer: &Renderer) {
        if self.delta_time < START_BUTTON_FLASH_FREQUENCY {
            let asset = Asset::new(
                AssetType::RuzzleUI,
                START_BUTTON_X_OFFSET,
                START_BUTTON_Y_OFFSET,
                START_BUTTON_WIDTH,
                START_BUTTON_HEIGHT,
                None,
                None,
            );
            let start_btn_render_width = START_BUTTON_WIDTH;
            let start_btn_render_height = START_BUTTON_HEIGHT;
            let start_btn_x = (self.width - start_btn_render_width) / 2f64;
            let start_btn_y = self.height * 0.75f64;
            renderer.draw_asset_by_coordinate(&asset, start_btn_x, start_btn_y, start_btn_render_width, start_btn_render_height);
        }
    }
}