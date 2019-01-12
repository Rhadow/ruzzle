use web_sys::console::log_1;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, Window};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct Canvas {
    pub window: Window,
    pub key_map: Rc<RefCell<HashMap<String, Option<f64>>>>,
    canvas_element: HtmlCanvasElement,
}

impl Canvas {

    pub fn new(canvas_id: &str) -> Canvas {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(&canvas_id).unwrap().dyn_into().unwrap();
        Canvas {
            window,
            canvas_element: canvas,
            key_map: Rc::new(RefCell::new(init_key_map())),
        }
    }

    pub fn bind_events(&self) {
        bind_key_down_event(&self.window, Rc::clone(&self.key_map));
        bind_key_up_event(&self.window, Rc::clone(&self.key_map));
        bind_mouse_down_event(&self.canvas_element);
    }

    pub fn canvas_element(&self) -> &HtmlCanvasElement{
        &self.canvas_element
    }
}

fn bind_key_down_event(window: &Window, key_map: Rc<RefCell<HashMap<String, Option<f64>>>>) {
    let performance = window.performance().unwrap();
    let handler = move |event: web_sys::KeyboardEvent| {
        let mut key_map = (*(key_map)).borrow_mut();
        if let Some(_) = key_map.get(&event.key()) {
            let time = Some(performance.now());
            key_map.insert(event.key(), time);
        }
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_key_up_event(window: &Window, key_map: Rc<RefCell<HashMap<String, Option<f64>>>>) {
    let handler = move |event: web_sys::KeyboardEvent| {
        let mut key_map = (*(key_map)).borrow_mut();
        if let Some(_) = key_map.get(&event.key()) {
            key_map.insert(event.key(), None);
        }
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    window.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn bind_mouse_down_event(canvas: &HtmlCanvasElement) {
    let handler = move |event: web_sys::MouseEvent| {
        log_1(&format!("{}, {}", event.offset_x(), event.offset_y()).into());
    };
    let closure = Closure::wrap(Box::new(handler) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

fn init_key_map() -> HashMap<String, Option<f64>> {
    let mut result = HashMap::new();
    result.insert(String::from(ARROW_DOWN), None);
    result.insert(String::from(ARROW_UP), None);
    result.insert(String::from(ARROW_LEFT), None);
    result.insert(String::from(ARROW_RIGHT), None);
    return result;
}