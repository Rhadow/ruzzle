// use web_sys::console::log_1;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use crate::game::{World, Asset, AssetType};
use crate::game::terrain::Terrain;
use crate::game::object::Object;
use crate::game::character::Character;
use crate::game::constants::{
    CELL_SIZE,
    ASSET_SIZE
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
        let width_in_cells = world.width_in_cells();
        let height_in_cells = world.height_in_cells();
        let cells = world.cells();
        let characters = world.get_characters();
        self.ctx.clear_rect(0f64, 0f64, width_in_cells as f64 * CELL_SIZE, height_in_cells as f64 * CELL_SIZE);

        for row in 0..height_in_cells {
            for col in 0..width_in_cells {
                let idx = world.get_index(row, col);
                let cell = &cells[idx];
                if let Some(terrain) = cell.get_terrain() {
                    self.draw_terrain(terrain, row as f64, col as f64);
                }
                if let Some(object) = cell.get_object() {
                    self.draw_object(object, row as f64, col as f64);
                }
            }
        }

        self.draw_characters(characters);
    }

    fn draw_terrain(&self, terrain: &Box<dyn Terrain>, row: f64, col: f64) {
        let asset = terrain.get_asset();
        self.draw_asset(asset, row, col);
    }

    fn draw_object(&self, object: &Box<dyn Object>, row: f64, col: f64) {
        let asset = object.get_asset();
        self.draw_asset(asset, row, col);
    }

    fn draw_characters(&self, characters: &Vec<Box<dyn Character>>) {
        for character in characters {
            let asset = character.get_asset();
            let (row, col) = (character.get_coordinate().row(), character.get_coordinate().col());
            self.draw_asset(asset, row as f64, col as f64);
        }
    }

    fn draw_asset(&self, asset: &Asset, row: f64, col: f64) {
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
            col as f64 * CELL_SIZE,
            row as f64 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        ).unwrap();
    }
}