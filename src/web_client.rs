use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use utils::set_panic_hook;
use crate::game::World;
use crate::renderer::WebRenderer;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

#[wasm_bindgen]
pub struct WebClient {
    canvas: HtmlCanvasElement,
    world: World,
    renderer: WebRenderer
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, assets_id: String) -> WebClient {
        set_panic_hook();
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(&canvas_id).unwrap().dyn_into().unwrap();
        let assets: HtmlImageElement = document.get_element_by_id(&assets_id).unwrap().dyn_into().unwrap();
        let renderer: WebRenderer = WebRenderer::new(&canvas, assets);
        let world = World::new();
        WebClient {
            canvas,
            world,
            renderer
        }
    }

    pub fn render(&self) {
        self.renderer.render(&self.world);
    }
}