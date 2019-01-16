// use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use web_sys::Performance;
use crate::canvas::Canvas;
use crate::renderer::WebRenderer;
use crate::audio::{BGM, AudioPlayer, WebAudioPlayer};
use crate::game::World;
use crate::game::level::LevelManager;
use crate::game::character::{Character, Player};
use crate::game::status_manager::Direction;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

#[derive(Debug, Deserialize)]
pub struct WebAssets {
    pub sprite: Vec<String>,
    pub bgm: Vec<String>,
    pub sfx: Vec<String>,
}

#[wasm_bindgen]
pub struct WebClient {
    canvas: Canvas,
    world: World,
    timer: Performance,
    renderer: WebRenderer,
    audio: WebAudioPlayer,
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, assets: JsValue) -> WebClient {
        set_panic_hook();
        let assets: WebAssets = assets.into_serde().unwrap();
        let current_level: usize = 0;
        let window = web_sys::window().unwrap();
        let timer = window.performance().unwrap();
        let now = timer.now();
        let mut audio = WebAudioPlayer::new(&assets);
        let world = init_world(current_level, now);
        let canvas = Canvas::new(&canvas_id);
        let renderer = WebRenderer::new(canvas.canvas_element(), &assets);
        canvas.bind_events();
        audio.play_bgm(BGM::World1);

        WebClient {
            canvas,
            world,
            timer,
            renderer,
            audio,
        }
    }

    pub fn update(&mut self) {
        self.check_direction_event();
        let now = self.timer.now();
        self.world.update(now, &mut self.audio);
    }

    pub fn render(&self) {
        self.renderer.render(&self.world);
    }

    fn check_direction_event(&mut self) {
        let key_map = (*(self.canvas.key_map)).borrow();
        let mut direction_key = None;
        let mut most_recent_timestamp = 0f64;
        for (key, &value) in &(*key_map) {
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
            self.world.handle_direction_event(direction_key);
        }
    }
}

fn init_world<> (current_level: usize, current_time: f64) -> World {
    let mut level_manager = LevelManager::new();
    let (level_cells, objects) = level_manager.construct_level(current_level);
    let player_position = level_manager.get_player_position().unwrap();
    let player = Box::new(Player::new(player_position, Direction::Down, current_time)) as Box<dyn Character>;
    World::new(level_cells, objects, vec![player],)
}
