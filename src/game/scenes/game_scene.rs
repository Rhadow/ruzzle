// use web_sys::console::log_1;
use wasm_bindgen::prelude::JsValue;
use crate::utils::is_mouse_inside_box;
use super::{SceneType, Scene};
use crate::renderer::Renderer;
use crate::game::{Asset, AssetType, World, Position, Direction};
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
    BACK_BUTTON_X_OFFSET,
    BACK_BUTTON_Y_OFFSET,
    BACK_BUTTON_SIZE,
    RESET_BUTTON_X_OFFSET,
    RESET_BUTTON_Y_OFFSET,
    RESET_BUTTON_SIZE,
};
use crate::audio::AudioPlayer;

pub struct GameScene {
    scene_type: SceneType,
    next_scene_type: Option<SceneType>,
    action_timestamp: f64,
    mouse_down_coordinate: Option<(f64, f64)>,
    reset_btn_asset: Asset,
    back_btn_asset: Asset,
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
        let is_input_disabled = world.player().borrow().at_exit();
        if let Some((down_x, down_y)) = self.mouse_down_coordinate {
            if !is_input_disabled {
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
    fn on_mouse_move(&mut self, mouse_x: f64, mouse_y: f64, world: &mut World) {
        let is_input_disabled = world.player().borrow().at_exit();
        if !is_input_disabled {
            if self.is_back_btn_hovered(mouse_x, mouse_y) {
                self.back_btn_asset.set_x_offset(BACK_BUTTON_X_OFFSET + 2f64);
            } else {
                self.back_btn_asset.set_x_offset(BACK_BUTTON_X_OFFSET);
            }
            if self.is_reset_btn_hovered(mouse_x, mouse_y) {
                self.reset_btn_asset.set_x_offset(RESET_BUTTON_X_OFFSET + 2f64);
            } else {
                self.reset_btn_asset.set_x_offset(RESET_BUTTON_X_OFFSET);
            }
        }
    }
}

impl GameScene {
    pub fn new() -> GameScene {
        let reset_btn_asset = Asset::new(
            AssetType::RuzzleUI,
            RESET_BUTTON_X_OFFSET,
            RESET_BUTTON_Y_OFFSET,
            RESET_BUTTON_SIZE,
            RESET_BUTTON_SIZE,
            None,
            None,
        );
        let back_btn_asset = Asset::new(
            AssetType::RuzzleUI,
            BACK_BUTTON_X_OFFSET,
            BACK_BUTTON_Y_OFFSET,
            BACK_BUTTON_SIZE,
            BACK_BUTTON_SIZE,
            None,
            None,
        );
        GameScene {
            scene_type: SceneType::Game,
            next_scene_type: None,
            action_timestamp: 0f64,
            mouse_down_coordinate: None,
            reset_btn_asset,
            back_btn_asset,
        }
    }
    fn check_direction_event(&mut self, controller: &mut Controller, world: &mut World) {
        // Check for a keyboard event that would cause player movement first
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
            return;
        }

        // Otherwise check if the mouse is currently pressed in a way that would cause player movement
        if controller.is_mouse_down && self.is_mouse_on_map(controller.mouse_x, controller.mouse_y) {
            let mouse_position = self.get_mouse_position(controller.mouse_x, controller.mouse_y);
            let player_position = world.player().borrow().status_manager().position;
            let target_direction = self.get_relative_mouse_direction(&mouse_position, &player_position);
            world.handle_player_movement(target_direction);
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
        let x = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let y = 0f64;
        let menu_background_color = JsValue::from_str("#555555");
        renderer.draw_rectangle(x, y, TILE_SIZE, TILE_SIZE * WORLD_HEIGHT_IN_TILES as f64, &menu_background_color);
        renderer.draw_asset_by_coordinate(&self.reset_btn_asset, x, TILE_SIZE * (WORLD_HEIGHT_IN_TILES - 2) as f64, RESET_BUTTON_SIZE, RESET_BUTTON_SIZE);
        renderer.draw_asset_by_coordinate(&self.back_btn_asset, x, TILE_SIZE * (WORLD_HEIGHT_IN_TILES - 1) as f64, RESET_BUTTON_SIZE, RESET_BUTTON_SIZE);
    }
    fn is_back_btn_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        self.is_back_btn_hovered(down_x, down_y) && self.is_back_btn_hovered(up_x, up_y)
    }
    fn is_reset_btn_pressed (&self, down_x: f64, down_y: f64, up_x: f64, up_y: f64) -> bool {
        self.is_reset_btn_hovered(down_x, down_y) && self.is_reset_btn_hovered(up_x, up_y)
    }
    fn is_back_btn_hovered (&self, x: f64, y: f64) -> bool {
        let x0 = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let x1 = x + BACK_BUTTON_SIZE;
        let y1 = WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE;
        let y0 = y1 - BACK_BUTTON_SIZE;
        is_mouse_inside_box(x, y, x0, y0, x1, y1)
    }
    fn is_reset_btn_hovered (&self, x: f64, y: f64) -> bool {
        let x0 = WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let x1 = x0 + RESET_BUTTON_SIZE;
        let y1 = WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE - BACK_BUTTON_SIZE;
        let y0 = y1 - RESET_BUTTON_SIZE;
        is_mouse_inside_box(x, y, x0, y0, x1, y1)
    }
    fn is_mouse_on_map(&self, x: f64, y: f64) -> bool {
        let x0 = 0f64;
        let x1 = x0 + WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE;
        let y0 = 0f64;
        let y1 = y0 + WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE;
        is_mouse_inside_box(x, y, x0, y0, x1, y1)
    }
    fn get_mouse_position(&self, x: f64, y: f64) -> Position {
        let row = (y / TILE_SIZE).floor();
        let col = (x / TILE_SIZE).floor();
        Position(row,col)
    }
    fn get_relative_mouse_direction(&self, mouse_position: &Position, player_position: &Position) -> Option<Direction> {
        let m_row = mouse_position.row();
        let m_col = mouse_position.col();
        let p_row = player_position.row();
        let p_col = player_position.col();
        let vertical_diff = m_row - p_row;
        let horizontal_diff = m_col - p_col;
        if vertical_diff == 0f64 && horizontal_diff == 0f64 {
            return None;
        }
        if vertical_diff.abs() > horizontal_diff.abs() {
            if vertical_diff > 0f64 {
                Some(Direction::Down)
            } else {
                Some(Direction::Up)
            }
        } else {
            if horizontal_diff < 0f64 {
                Some(Direction::Left)
            } else {
                Some(Direction::Right)
            }
        }
    }
}