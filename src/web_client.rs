use wasm_bindgen::prelude::*;
use utils::set_panic_hook;
use std::rc::Rc;
use std::cell::RefCell;
use crate::game::World;
use crate::renderer::WebRenderer;
use crate::game::canvas::Canvas;
use crate::game::level::LevelManager;
use crate::game::character::{Player, Direction};

#[wasm_bindgen]
pub struct WebClient {
    canvas: Canvas,
    world: Rc<RefCell<World>>,
    renderer: WebRenderer
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, env_assets_id: String, obj_assets_id: String, char_assets_id: String) -> WebClient {
        set_panic_hook();
        let current_level: usize = 0;
        let world = Rc::new(RefCell::new(init_world(current_level)));
        let canvas = Canvas::new(&canvas_id, Rc::clone(&world));
        let renderer: WebRenderer = WebRenderer::new(canvas.canvas_element(), &env_assets_id, &obj_assets_id, &char_assets_id);
        canvas.bind_events();

        WebClient {
            canvas,
            world: Rc::clone(&world),
            renderer
        }
    }

    pub fn update(&mut self) {
        (*(self.world)).borrow_mut().update();
    }

    pub fn render(&self) {
        let world = (*(self.world)).borrow();
        self.renderer.render(&world);
    }
}

fn init_world (current_level: usize) -> World {
    let mut world = World::new();
    let mut level_manager = LevelManager::new();
    let level_cells = level_manager.construct_level(current_level);
    let player_coordinate = level_manager.get_player_coordinate().unwrap();
    let player = Box::new(Player::new(player_coordinate, Direction::Down));
    world.set_cells(level_cells);
    world.set_characters(vec![player]);
    world
}
