// use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use web_sys::Performance;
use crate::game::scenes::{Scene, SceneType, EntryScene, GameScene, LevelSelectionScene};
use crate::game::World;
use crate::controller::Controller;
use crate::renderer::WebRenderer;
use crate::audio::{BGM, WebAudioPlayer, AudioPlayer};
use crate::game::level::LEVELS;

#[derive(Debug, Deserialize)]
pub struct WebAssets {
    pub sprite: Vec<String>,
    pub bgm: Vec<String>,
    pub sfx: Vec<String>,
}

#[wasm_bindgen]
pub struct WebClient {
    controller: Controller,
    renderer: WebRenderer,
    audio: Box<dyn AudioPlayer>,
    timer: Performance,
    world: World,
    current_scene: Box<dyn Scene>,
    completed_levels: Vec<bool>,
    current_level_page: usize,
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, assets: JsValue) -> WebClient {
        set_panic_hook();
        let assets: WebAssets = assets.into_serde().unwrap();
        let window = web_sys::window().unwrap();
        let timer = window.performance().unwrap();
        let audio = Box::new(WebAudioPlayer::new(&assets));
        let world = World::new(vec![], vec![], vec![]);
        let renderer = WebRenderer::new(&canvas_id, &assets);
        let controller = Controller::new();
        let completed_levels = vec![false; LEVELS.len()];

        WebClient {
            controller,
            world: world,
            timer,
            renderer,
            audio,
            current_scene: Box::new(EntryScene::new()),
            completed_levels,
            current_level_page: 0,
        }
    }

    pub fn update(&mut self) {
        let mut next_scene: Option<Box<dyn Scene>> = None;
        {
            let next_scene_type = self.current_scene.next_scene_type();
            if let Some(next_scene_type) = next_scene_type {
                match next_scene_type {
                    SceneType::Entry => {
                        next_scene = Some(Box::new(EntryScene::new()));
                    },
                    SceneType::Game => {
                        next_scene = Some(Box::new(GameScene::new()));
                        self.audio.play_bgm(BGM::World1);
                    },
                    SceneType::LevelSelection => {
                        next_scene = Some(Box::new(LevelSelectionScene::new(self.current_level_page)));
                        self.audio.play_bgm(BGM::LevelSelection);
                    },
                }
            }
        }
        if let Some(next_scene) = next_scene {
            self.current_scene = next_scene;
        }
        let now = self.timer.now();
        self.current_scene.update(&mut self.world, &mut self.controller, &mut self.audio, &mut self.completed_levels, now);
    }

    pub fn render(&self) {
        self.current_scene.render(&self.renderer, &self.world, &self.completed_levels);
    }

    pub fn handle_mousedown(&mut self, x: usize, y: usize) {
        self.controller.is_mouse_down = true;
        self.controller.mouse_x = x as f64;
        self.controller.mouse_y = y as f64;
        self.current_scene.on_mouse_down(x as f64, y as f64, &mut self.world);
    }

    pub fn handle_mouseup(&mut self, x: usize, y: usize) {
        self.controller.is_mouse_down = false;
        self.controller.mouse_x = x as f64;
        self.controller.mouse_y = y as f64;
        self.current_scene.on_mouse_up(x as f64, y as f64, &mut self.world, &mut self.current_level_page);
    }

    pub fn handle_keydown(&mut self, key: String, time: usize) {
        let value = self.controller.key_map.get(&key).cloned();
        if let Some(value) = value {
            if value == None {
                self.controller.key_map.insert(key, Some(time as f64));
            }
        }
    }

    pub fn handle_keyup(&mut self, key: String) {
        if let Some(_) = self.controller.key_map.get(&key) {
            self.controller.key_map.insert(key, None);
        }
    }
}
