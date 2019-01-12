use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
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
    env_assets: HtmlImageElement,
    obj_assets: HtmlImageElement,
    char_assets: HtmlImageElement,
}

impl WebRenderer {
    pub fn new(canvas: &HtmlCanvasElement, env_assets_id: &str, obj_assets_id: &str, char_assets_id: &str) -> WebRenderer {
        let document = web_sys::window().unwrap().document().unwrap();
        let env_assets: HtmlImageElement = document.get_element_by_id(env_assets_id).unwrap().dyn_into().unwrap();
        let obj_assets: HtmlImageElement = document.get_element_by_id(obj_assets_id).unwrap().dyn_into().unwrap();
        let char_assets: HtmlImageElement = document.get_element_by_id(char_assets_id).unwrap().dyn_into().unwrap();
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        WebRenderer {
            ctx,
            env_assets,
            obj_assets,
            char_assets
        }
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
                if let Some(terrain) = tile.get_terrain() {
                    self.draw_terrain(terrain, row as f64, col as f64);
                }
            }
        }
        self.draw_objects(objects);
        self.draw_characters(characters);
    }

    fn draw_terrain(&self, terrain: &Box<dyn Terrain>, row: f64, col: f64) {
        let asset = terrain.get_asset();
        self.draw_asset_by_position(asset, row, col);
    }

    fn draw_objects(&self, objects: &Vec<RefCell<Box<dyn Object>>>) {
        for object in objects {
            let object = object.borrow();
            let asset = object.asset();
            let (x, y) = (object.movement_manager().coordinate.x(), object.movement_manager().coordinate.y());
            self.draw_asset_by_coordinate(asset, x, y);
        }
    }

    fn draw_characters(&self, characters: &Vec<RefCell<Box<dyn Character>>>) {
        for character in characters {
            let character = character.borrow();
            let asset = character.asset();
            let (x, y) = (character.movement_manager().coordinate.x(), character.movement_manager().coordinate.y());
            self.draw_asset_by_coordinate(asset, x, y);
        }
    }

    fn draw_asset_by_coordinate(&self, asset: &Asset, x: f64, y: f64) {
        let asset_by_type = match asset.get_type() {
            AssetType::Environment => &self.env_assets,
            AssetType::Object => &self.obj_assets,
            AssetType::Character => &self.char_assets,
        };
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
        let asset_by_type = match asset.get_type() {
            AssetType::Environment => &self.env_assets,
            AssetType::Object => &self.obj_assets,
            AssetType::Character => &self.char_assets,
        };
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