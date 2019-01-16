// use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use web_sys::Performance;
use crate::game::scenes::{Scene, GameScene};
use crate::canvas::Canvas;
use crate::renderer::WebRenderer;
use crate::audio::WebAudioPlayer;
use crate::game::World;
use crate::game::level::LevelManager;
use crate::game::character::{Character, Player};
use crate::game::status_manager::Direction;

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
    current_scene: Box<dyn Scene>,
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
        let audio = WebAudioPlayer::new(&assets);
        let world = init_world(current_level, now);
        let canvas = Canvas::new(&canvas_id);
        let renderer = WebRenderer::new(canvas.canvas_element(), &assets);
        canvas.bind_events();

        WebClient {
            canvas,
            world,
            timer,
            renderer,
            audio,
            current_scene: Box::new(GameScene::new()),
        }
    }

    pub fn update(&mut self) {
        let now = self.timer.now();
        self.current_scene.update(&mut self.world, &mut self.canvas.key_map, &mut self.audio, now);
    }

    pub fn render(&self) {
        self.current_scene.render(&self.renderer, &self.world);
    }
}

fn init_world<> (current_level: usize, current_time: f64) -> World {
    let mut level_manager = LevelManager::new();
    let (level_cells, objects) = level_manager.construct_level(current_level);
    let player_position = level_manager.get_player_position().unwrap();
    let player = Box::new(Player::new(player_position, Direction::Down, current_time)) as Box<dyn Character>;
    World::new(level_cells, objects, vec![player],)
}
