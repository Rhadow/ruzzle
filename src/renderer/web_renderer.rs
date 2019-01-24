// use web_sys::console::log_1;
use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use super::Renderer;
use crate::web_client::WebAssets;
use crate::game::{Asset, AssetType};
use crate::game::terrain::Terrain;
use crate::game::object::Object;
use crate::game::character::Character;
use crate::game::constants::{
    TILE_SIZE,
    ASSET_SIZE,
    WORLD_HEIGHT_IN_TILES,
    WORLD_WIDTH_IN_TILES,
};

pub struct WebRenderer {
    ctx: CanvasRenderingContext2d,
    asset_type_map: HashMap<AssetType, HtmlImageElement>,
    width: f64,
    height: f64
}

impl Renderer for WebRenderer {
    fn clear_screen(&self) {
        // log_1(&format!("{}, {}", self.width, self.height).into());
        self.ctx.clear_rect(0f64, 0f64, self.width, self.height);
    }

    fn draw_terrain(&self, terrain: &RefCell<Box<dyn Terrain>>) {
        let terrain = terrain.borrow();
        let asset = terrain.asset();
        let (x, y) = (terrain.status_manager().coordinate.x(), terrain.status_manager().coordinate.y());
        self.draw_asset_by_coordinate(asset, x, y, TILE_SIZE, TILE_SIZE);
    }

    fn draw_objects(&self, objects: &Vec<RefCell<Box<dyn Object>>>) {
        for object in objects {
            let mut object = object.borrow_mut();
            if object.attribute_manager().is_visible {
                let asset = object.asset();
                let (x, y) = (object.status_manager().coordinate.x(), object.status_manager().coordinate.y());
                let (w, h) = (object.status_manager().width, object.status_manager().height);
                self.draw_asset_by_coordinate(asset, x, y, w, h);
            }
        }
    }

    fn draw_characters(&self, characters: &Vec<RefCell<Box<dyn Character>>>) {
        for character in characters {
            let character = character.borrow();
            let asset = character.asset();
            let (x, y) = (character.status_manager().coordinate.x(), character.status_manager().coordinate.y());
            let (w, h) = (character.status_manager().width, character.status_manager().height);
            self.draw_asset_by_coordinate(asset, x, y, w, h);
        }
    }

    fn draw_asset_by_coordinate(&self, asset: &Asset, x: f64, y: f64, width: f64, height: f64) {
        let asset_by_type = self.asset_type_map.get(asset.get_type()).unwrap();
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            asset_by_type,
            asset.get_x_offset() * ASSET_SIZE,
            asset.get_y_offset() * ASSET_SIZE,
            asset.get_width(),
            asset.get_height(),
            x,
            y,
            width,
            height,
        ).unwrap();
    }

    fn draw_rectangle(&self, x: f64, y: f64, width: f64, height: f64, fill_color: &JsValue) {
        self.ctx.begin_path();
        self.ctx.set_fill_style(fill_color);
        self.ctx.rect(x, y, width, height);
        self.ctx.fill_rect(x, y, width, height);
        self.ctx.stroke();
        self.ctx.close_path();
    }
}

impl WebRenderer {
    pub fn new(canvas_id: &str, assets: &WebAssets) -> WebRenderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas: HtmlCanvasElement = document.get_element_by_id(canvas_id).unwrap().dyn_into().unwrap();
        let is_asset_type_map = WebRenderer::init_id_asset_type_map();
        let mut asset_type_map = HashMap::new();
        for sprite_id in &assets.sprite {
            let asset_type = is_asset_type_map.get(sprite_id).unwrap();
            let asset_element = document.get_element_by_id(sprite_id).unwrap().dyn_into().unwrap();
            asset_type_map.insert(*asset_type, asset_element);
        }
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        WebRenderer {
            ctx,
            asset_type_map,
            width: WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE,
            height: WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE,
        }
    }

    fn init_id_asset_type_map() -> HashMap<String, AssetType> {
        let mut asset_type_map: HashMap<String, AssetType> = HashMap::new();
        asset_type_map.insert(String::from("environment"), AssetType::Environment);
        asset_type_map.insert(String::from("object"), AssetType::Object);
        asset_type_map.insert(String::from("character"), AssetType::Character);
        asset_type_map
    }
}