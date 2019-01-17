// use web_sys::console::log_1;
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

pub struct InputMap {
    pub key_map: HashMap<String, Option<f64>>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub is_mouse_down: bool
}

impl InputMap {
    pub fn new() -> InputMap {
        InputMap {
            key_map: InputMap::init_key_map(),
            mouse_x: 0f64,
            mouse_y: 0f64,
            is_mouse_down: false,
        }
    }
    fn init_key_map() -> HashMap<String, Option<f64>> {
        let mut result = HashMap::new();
        result.insert(String::from(ARROW_DOWN), None);
        result.insert(String::from(ARROW_UP), None);
        result.insert(String::from(ARROW_LEFT), None);
        result.insert(String::from(ARROW_RIGHT), None);
        return result;
    }
}

pub type CanvasInputMap = Rc<RefCell<InputMap>>;

pub struct Canvas {
    pub window: Window,
    pub input_map: CanvasInputMap,
    pub canvas_element: HtmlCanvasElement,
}

impl Canvas {
    pub fn new(canvas_id: &str) -> Canvas {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(&canvas_id).unwrap().dyn_into().unwrap();
        Canvas {
            window,
            canvas_element: canvas,
            input_map: Rc::new(RefCell::new(InputMap::new())),
        }
    }
    pub fn canvas_element(&self) -> &HtmlCanvasElement{
        &self.canvas_element
    }
}
