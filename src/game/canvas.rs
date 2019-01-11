use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::game::World;
use web_sys::{HtmlCanvasElement, Window};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Canvas {
    pub window: Window,
    canvas_element: HtmlCanvasElement,
    world: Rc<RefCell<World>>,
}

impl Canvas {

    pub fn new(canvas_id: &str, world: Rc<RefCell<World>>) -> Canvas {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(&canvas_id).unwrap().dyn_into().unwrap();
        Canvas {
            window,
            canvas_element: canvas,
            world,
        }
    }

    pub fn bind_events(&self) {
        bind_key_down_event(&self.window, Rc::clone(&self.world));
        bind_mouse_down_event(&self.canvas_element, Rc::clone(&self.world));
    }

    pub fn canvas_element(&self) -> &HtmlCanvasElement{
        &self.canvas_element
    }
}



fn bind_key_down_event(window: &Window, world: Rc<RefCell<World>>) {
    let handler = move |event: web_sys::KeyboardEvent| {
        let mut world = (*(world)).borrow_mut();
        world.handle_key_down_event(&event.key());
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_mouse_down_event(canvas: &HtmlCanvasElement, _world: Rc<RefCell<World>>) {
    let handler = move |event: web_sys::MouseEvent| {
        log_1(&format!("{}, {}", event.offset_x(), event.offset_y()).into());
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}