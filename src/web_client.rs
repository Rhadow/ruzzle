use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use utils::set_panic_hook;
use web_sys::HtmlCanvasElement;
use crate::game::World;
use crate::renderer::WebRenderer;
use crate::game::level::LevelManager;
use crate::game::character::{Player, Direction};

#[wasm_bindgen]
pub struct WebClient {
    canvas: HtmlCanvasElement,
    world: World,
    renderer: WebRenderer
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, env_assets_id: String, obj_assets_id: String, char_assets_id: String) -> WebClient {
        set_panic_hook();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(&canvas_id).unwrap().dyn_into().unwrap();
        let renderer: WebRenderer = WebRenderer::new(&canvas, &env_assets_id, &obj_assets_id, &char_assets_id);
        let current_level: usize = 0;
        let world = init_world(current_level);

        WebClient {
            canvas,
            world,
            renderer
        }
    }

    pub fn render(&mut self) {
        self.world.update();
        self.renderer.render(&self.world);
    }
}

fn init_world (current_level: usize) -> World {
    let mut world = World::new();
    let mut level_manager = LevelManager::new();
    let level_cells = level_manager.construct_level(current_level);
    let player_coordinate = level_manager.get_player_coordinate().unwrap();
    let player = Box::new(Player::new(player_coordinate, Direction::Right));
    world.set_cells(level_cells);
    world.set_characters(vec![player]);
    world
}