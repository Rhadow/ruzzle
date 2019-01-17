// use web_sys::console::log_1;
use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use utils::set_panic_hook;
use web_sys::{HtmlCanvasElement, Performance, Window};
use crate::game::scenes::{Scene, SceneType, EntryScene, GameScene};
use crate::game::World;
use crate::canvas::{Canvas, CanvasInputMap};
use crate::renderer::WebRenderer;
use crate::audio::WebAudioPlayer;

#[derive(Debug, Deserialize)]
pub struct WebAssets {
    pub sprite: Vec<String>,
    pub bgm: Vec<String>,
    pub sfx: Vec<String>,
}

#[wasm_bindgen]
pub struct WebClient {
    canvas: Canvas,
    world: Rc<RefCell<World>>,
    timer: Performance,
    renderer: WebRenderer,
    audio: WebAudioPlayer,
    current_scene: Rc<RefCell<Box<dyn Scene>>>,
}

#[wasm_bindgen]
impl WebClient {
    pub fn new(canvas_id: String, assets: JsValue) -> WebClient {
        set_panic_hook();
        let assets: WebAssets = assets.into_serde().unwrap();
        let window = web_sys::window().unwrap();
        let timer = window.performance().unwrap();
        let audio = WebAudioPlayer::new(&assets);
        let world = World::new(vec![], vec![], vec![]);
        let canvas = Canvas::new(&canvas_id);
        let renderer = WebRenderer::new(canvas.canvas_element(), &assets);

        WebClient {
            canvas,
            world: Rc::new(RefCell::new(world)),
            timer,
            renderer,
            audio,
            current_scene: Rc::new(RefCell::new(Box::new(EntryScene::new()))),
        }
    }

    pub fn update(&mut self) {
        let mut next_scene: Option<Rc<RefCell<Box<dyn Scene>>>> = None;
        {
            let borrowed = self.current_scene.borrow();
            let next_scene_type = borrowed.next_scene_type();
            if let Some(next_scene_type) = next_scene_type {
                match next_scene_type {
                    SceneType::Entry => {
                        next_scene = Some(Rc::new(RefCell::new(Box::new(EntryScene::new()))));
                    },
                    SceneType::Game => {
                        next_scene = Some(Rc::new(RefCell::new(Box::new(GameScene::new()))));
                    },
                    _ => ()
                }
            }
        }
        if let Some(next_scene) = next_scene {
            self.current_scene = next_scene;
        }
        let now = self.timer.now();
        self.current_scene.borrow_mut().update(&mut self.world.borrow_mut(), &mut self.canvas.input_map, &mut self.audio, now);
    }

    pub fn render(&self) {
        self.current_scene.borrow().render(&self.renderer, &self.world.borrow());
    }

    pub fn bind_events(&mut self) {
        bind_key_down_event(&self.canvas.window, Rc::clone(&self.canvas.input_map));
        bind_key_up_event(&self.canvas.window, Rc::clone(&self.canvas.input_map));
        bind_mouse_down_event(&self.canvas.canvas_element, Rc::clone(&mut self.canvas.input_map));
        bind_mouse_up_event(&self.canvas.canvas_element, Rc::clone(&mut self.canvas.input_map), Rc::clone(&mut self.world), Rc::clone(&mut self.current_scene));
    }
}

fn bind_key_down_event(window: &Window, input_map: CanvasInputMap) {
    let performance = window.performance().unwrap();
    let handler = move |event: web_sys::KeyboardEvent| {
        let mut input_map = (*(input_map)).borrow_mut();
        if let Some(_) = input_map.key_map.get(&event.key()) {
            let time = Some(performance.now());
            input_map.key_map.insert(event.key(), time);
        }
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_key_up_event(window: &Window, input_map: CanvasInputMap) {
    let handler = move |event: web_sys::KeyboardEvent| {
        let mut input_map = (*(input_map)).borrow_mut();
        if let Some(_) = input_map.key_map.get(&event.key()) {
            input_map.key_map.insert(event.key(), None);
        }
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_mouse_down_event(canvas: &HtmlCanvasElement, input_map: CanvasInputMap) {
    let handler = move |event: web_sys::MouseEvent| {
        let mut input_map = input_map.borrow_mut();
        input_map.is_mouse_down = true;
        input_map.mouse_x = event.offset_x() as f64;
        input_map.mouse_y = event.offset_y() as f64;
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_mouse_up_event(canvas: &HtmlCanvasElement, input_map: CanvasInputMap, world: Rc<RefCell<World>>, current_scene: Rc<RefCell<Box<dyn Scene>>>) {
    let handler = move |event: web_sys::MouseEvent| {
        let mut world = world.borrow_mut();
        let mut input_map = input_map.borrow_mut();
        let mut current_scene = current_scene.borrow_mut();
        input_map.is_mouse_down = false;
        input_map.mouse_x = event.offset_x() as f64;
        input_map.mouse_y = event.offset_y() as f64;
        current_scene.on_mouse_up(input_map.mouse_x, input_map.mouse_y, &mut world);
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
