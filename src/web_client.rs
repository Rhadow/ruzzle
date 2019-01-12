// use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use web_sys::Performance;
use crate::renderer::WebRenderer;
use crate::game::World;
use crate::game::canvas::Canvas;
use crate::game::level::LevelManager;
use crate::game::character::{Character, Player};
use crate::game::movement_manager::Direction;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

#[wasm_bindgen]
pub struct WebClient {
    canvas: Canvas,
    world: World,
    timer: Performance,
    renderer: WebRenderer,
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, env_assets_id: String, obj_assets_id: String, char_assets_id: String) -> WebClient {
        set_panic_hook();
        let current_level: usize = 0;
        let window = web_sys::window().unwrap();
        let timer = window.performance().unwrap();
        let now = timer.now();
        let world = init_world(current_level, now);
        let canvas = Canvas::new(&canvas_id);
        let renderer: WebRenderer = WebRenderer::new(canvas.canvas_element(), &env_assets_id, &obj_assets_id, &char_assets_id);
        canvas.bind_events();

        WebClient {
            canvas,
            world,
            timer,
            renderer,
        }
    }

    pub fn update(&mut self) {
        self.check_direction_event();
        let now = self.timer.now();
        self.world.update(now);
    }

    pub fn render(&self) {
        self.renderer.render(&self.world);
    }

    fn check_direction_event(&mut self) {
        let key_map = (*(self.canvas.key_map)).borrow();
        let mut direction_key = None;
        for (key, &value) in &(*key_map) {
            if value && (key == ARROW_DOWN || key == ARROW_UP || key == ARROW_RIGHT || key == ARROW_LEFT) {
                direction_key = Some(key);
            }
        }
        if let Some(direction_key) = direction_key {
            self.world.handle_direction_event(direction_key);
        }
    }
}

fn init_world<> (current_level: usize, current_time: f64) -> World {
    let mut level_manager = LevelManager::new();
    let level_cells = level_manager.construct_level(current_level);
    let player_position = level_manager.get_player_position().unwrap();
    let player = Box::new(Player::new(player_position, Direction::Down, current_time)) as Box<dyn Character>;
    World::new(level_cells, vec![player])
}
