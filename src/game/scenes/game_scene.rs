// use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::World;
use crate::controller::Controller;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
    ACTION_KEY,
    TILE_SIZE,
    BACK_BUTTON_WIDTH,
    BACK_BUTTON_HEIGHT,
    RESET_BUTTON_WIDTH,
    RESET_BUTTON_HEIGHT,
};
use crate::audio::AudioPlayer;

pub struct GameScene {
    scene_type: SceneType,
    next_scene_type: Option<SceneType>,
    action_timestamp: f64,
    mouse_down_coordinate: Option<(f64, f64)>,
}

impl Scene for GameScene {
    fn scene_type(&self) -> &SceneType {
        &self.scene_type
    }
    fn render(&self, renderer: &Renderer, world: &World, _completed_levels: &Vec<bool>) {
        renderer.clear_screen();
        self.render_menu(renderer);
        self.render_game(renderer, world);
    }
    fn update(&mut self, world: &mut World, controller: &mut Controller, audio: &mut Box<dyn AudioPlayer>, completed_levels: &mut Vec<bool>, now: f64) {
        if world.is_completed {
            if let Some(level_number) = world.level_number {
                if !completed_levels[level_number] {
                    completed_levels[level_number] = true;
                }
            }
            self.set_next_scene_type(SceneType::LevelSelection);
        } else {
            self.check_direction_event(controller, world);
            self.check_action_event(controller, world);
            world.update(now, audio);
        }
    }
    fn next_scene_type(&self) -> &Option<SceneType> {
        &self.next_scene_type
    }
    fn set_next_scene_type(&mut self, scene_type: SceneType) {
        self.next_scene_type = Some(scene_type);
    }
    fn on_mouse_up(&mut self, mouse_x: f64, mouse_y: f64, world: &mut World, _current_level_page: &mut usize) {
        let is_menu_disabled = world.player().borrow().at_exit();
        if let Some((down_x, down_y)) = self.mouse_down_coordinate {
            if !is_menu_disabled {
                if self.is_back_btn_pressed(down_x, down_y, mouse_x, mouse_y) {
                    self.set_next_scene_type(SceneType::LevelSelection);
                }
                if self.is_reset_btn_pressed(down_x, down_y, mouse_x, mouse_y) {
                    let level = world.level_number.unwrap();
                    world.init_level(level);
                }
            }
        }
        self.mouse_down_coordinate = None;
    }
    fn on_mouse_down(&mut self, mouse_x: f64, mouse_y: f64, _world: &mut World) {
        if self.mouse_down_coordinate == None {
            self.mouse_down_coordinate = Some((mouse_x, mouse_y));
        }
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        GameScene {
            scene_type: SceneType::Game,
            next_scene_type: None,
            action_timestamp: 0f64,
            mouse_down_coordinate: None,
        }
    }
    fn check_direction_event(&mut self, controller: &mut Controller, world: &mut World) {
        let key_map = &controller.key_map;
        let mut direction_key = None;
        let mut most_recent_timestamp = 0f64;
        for (key, &value) in key_map {
            if key == ARROW_DOWN || key == ARROW_UP || key == ARROW_RIGHT || key == ARROW_LEFT {
                if let Some(timestamp) = value {
                    if timestamp > most_recent_timestamp {
                        most_recent_timestamp = timestamp;
                        direction_key = Some(key);
                    }
                }
            }
        }
        if let Some(direction_key) = direction_key {
            world.handle_direction_event(direction_key);
        }
    }
    fn check_action_event(&mut self, controller: &mut Controller, world: &mut World) {
        if let Some(value) = controller.key_map.get(ACTION_KEY) {
            if let Some(timestamp) = value {
                if *timestamp != self.action_timestamp {
                    world.handle_action_event(ACTION_KEY);
                }
                self.action_timestamp = *timestamp;
            }
        }
    }
    fn render_game(&self, renderer: &Renderer, world: &World) {
        let tile_map = world.tile_map();
        let characters = world.get_characters();
        let objects = world.get_objects();
        for row in 0..WORLD_HEIGHT_IN_TILES {
            for col in 0..WORLD_WIDTH_IN_TILES {
                let idx = world.get_index(row, col);
                let tile = &tile_map[idx];
                if let Some(terrain) = tile.borrow().terrain() {
                    renderer.draw_terrain(terrain);
                }
            }
        }
        renderer.draw_objects(objects);
        renderer.draw_characters(characters);
    }
    fn render_menu(&self, renderer: &Renderer) {
        for col in 0..WORLD_HEIGHT_IN_TILES {
            let x = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
            let y = col as f64 * TILE_SIZE;
            let menu_background_color = JsValue::from_str("#393852");
            let reset_btn_color = JsValue::from_str("#ff4711");
            let back_btn_color = JsValue::from_str("#4782c9");
            let mut color = menu_background_color;
            if col == WORLD_HEIGHT_IN_TILES - 1 {
                color = back_btn_color;
            } else if col == WORLD_HEIGHT_IN_TILES - 2 {
                color = reset_btn_color;
            }
            renderer.draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, &color);
        }
    }

    fn is_back_btn_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        let x0 = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let x1 = x0 + BACK_BUTTON_WIDTH;
        let y1 = WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE;
        let y0 = y1 - BACK_BUTTON_HEIGHT;
        let mut result = false;

        if down_x >= x0 && down_x <= x1 && up_x >= x0 && up_x <= x1 &&
            down_y >= y0 && down_y <= y1 && up_y >= y0 && up_y <= y1 {
            result = true;
        }
        result
    }
    fn is_reset_btn_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        let x0 = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let x1 = x0 + RESET_BUTTON_WIDTH;
        let y1 = WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE - BACK_BUTTON_HEIGHT;
        let y0 = y1 - RESET_BUTTON_HEIGHT;
        let mut result = false;

        if down_x >= x0 && down_x <= x1 && up_x >= x0 && up_x <= x1 &&
            down_y >= y0 && down_y <= y1 && up_y >= y0 && up_y <= y1 {
            result = true;
        }
        result
    }
}