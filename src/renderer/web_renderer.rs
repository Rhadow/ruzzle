use std::cell::RefCell;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use crate::web_client::WebAssets;
use crate::game::{World, Asset, AssetType};
use crate::game::terrain::Terrain;
use crate::game::object::Object;
use crate::game::character::Character;
use crate::game::constants::{
    TILE_SIZE,
    ASSET_SIZE,
    WORLD_WIDTH_IN_TILES,
    WORLD_HEIGHT_IN_TILES,
};

pub struct WebRenderer {
    ctx: CanvasRenderingContext2d,
    asset_type_map: HashMap<AssetType, HtmlImageElement>,
}

impl WebRenderer {
    pub fn new(canvas: &HtmlCanvasElement, assets: &WebAssets) -> WebRenderer {
        let document = web_sys::window().unwrap().document().unwrap();
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
        }
    }

    fn init_id_asset_type_map() -> HashMap<String, AssetType> {
        let mut asset_type_map: HashMap<String, AssetType> = HashMap::new();
        asset_type_map.insert(String::from("environment"), AssetType::Environment);
        asset_type_map.insert(String::from("object"), AssetType::Object);
        asset_type_map.insert(String::from("character"), AssetType::Character);
        asset_type_map
    }

    pub fn render(&self, world: &World) {
        let tile_map = world.tile_map();
        let characters = world.get_characters();
        let objects = world.get_objects();
        self.ctx.clear_rect(0f64, 0f64, WORLD_WIDTH_IN_TILES as f64 * TILE_SIZE, WORLD_HEIGHT_IN_TILES as f64 * TILE_SIZE);

        for row in 0..WORLD_HEIGHT_IN_TILES {
            for col in 0..WORLD_WIDTH_IN_TILES {
                let idx = world.get_index(row, col);
                let tile = &tile_map[idx];
                if let Some(terrain) = tile.borrow().terrain() {
                    self.draw_terrain(terrain, row as f64, col as f64);
                }
            }
        }
        self.draw_objects(objects);
        self.draw_characters(characters);
    }

    fn draw_terrain(&self, terrain: &RefCell<Box<dyn Terrain>>, row: f64, col: f64) {
        let terrain = terrain.borrow();
        let asset = terrain.get_asset();
        self.draw_asset_by_position(asset, row, col);
    }

    fn draw_objects(&self, objects: &Vec<RefCell<Box<dyn Object>>>) {
        for object in objects {
            let object = object.borrow();
            if object.is_visible() {
                let asset = object.asset();
                let (x, y) = (object.status_manager().coordinate.x(), object.status_manager().coordinate.y());
                self.draw_asset_by_coordinate(asset, x, y);
            }
        }
    }

    fn draw_characters(&self, characters: &Vec<RefCell<Box<dyn Character>>>) {
        for character in characters {
            let character = character.borrow();
            let asset = character.asset();
            let (x, y) = (character.status_manager().coordinate.x(), character.status_manager().coordinate.y());
            self.draw_asset_by_coordinate(asset, x, y);
        }
    }

    fn draw_asset_by_coordinate(&self, asset: &Asset, x: f64, y: f64) {
        let asset_by_type = self.asset_type_map.get(asset.get_type()).unwrap();
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            asset_by_type,
            asset.get_x_offset() * ASSET_SIZE,
            asset.get_y_offset() * ASSET_SIZE,
            asset.get_width(),
            asset.get_height(),
            x,
            y,
            TILE_SIZE,
            TILE_SIZE,
        ).unwrap();
    }

    fn draw_asset_by_position(&self, asset: &Asset, row: f64, col: f64) {
        let asset_by_type = self.asset_type_map.get(asset.get_type()).unwrap();
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            asset_by_type,
            asset.get_x_offset() * ASSET_SIZE,
            asset.get_y_offset() * ASSET_SIZE,
            asset.get_width(),
            asset.get_height(),
            col as f64 * TILE_SIZE,
            row as f64 * TILE_SIZE,
            TILE_SIZE,
            TILE_SIZE,
        ).unwrap();
    }
}