use std::cell::RefCell;
use crate::game::Asset;
use crate::game::terrain::Terrain;
use crate::game::object::Object;
use crate::game::character::Character;

pub mod web_renderer;

pub use self::web_renderer::WebRenderer;

pub trait Renderer {
    fn clear_screen(&self);
    fn draw_terrain(&self, terrain: &RefCell<Box<dyn Terrain>>, row: f64, col: f64);
    fn draw_objects(&self, objects: &Vec<RefCell<Box<dyn Object>>>);
    fn draw_characters(&self, characters: &Vec<RefCell<Box<dyn Character>>>);
    fn draw_asset_by_coordinate(&self, asset: &Asset, x: f64, y: f64);
    fn draw_asset_by_position(&self, asset: &Asset, row: f64, col: f64);
}