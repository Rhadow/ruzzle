use std::collections::HashMap;
use crate::game::constants::{
    ARROW_DOWN,
    ARROW_UP,
    ARROW_RIGHT,
    ARROW_LEFT,
};

pub struct Controller {
    pub key_map: HashMap<String, Option<f64>>,
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub is_mouse_down: bool
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            key_map: Controller::init_key_map(),
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