// use web_sys::console::log_1;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use crate::game::{World, AssetType};
use crate::game::terrain::Terrain;
use crate::game::object::Object;
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
        let width = world.width();
        let height = world.height();
        let cells = world.cells();
        self.ctx.clear_rect(0f64, 0f64, width as f64 * CELL_SIZE, height as f64 * CELL_SIZE);

        for row in 0..height {
            for col in 0..width {
                let idx = world.get_index(row, col);
                let cell = &cells[idx];
                if let Some(terrain) = cell.get_terrain() {
                    self.draw_terrain(terrain, row, col);
                }
                if let Some(object) = cell.get_object() {
                    self.draw_object(object, row, col);
                }
            }
        }
    }

    fn draw_terrain(&self, terrain: &Box<dyn Terrain>, row: usize, col: usize) {
        let asset = match terrain.get_asset_type() {
            AssetType::Environment => &self.env_assets,
            AssetType::Object => &self.obj_assets,
            AssetType::Character => &self.char_assets,
        };
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            asset,
            terrain.get_asset_x_offset() * ASSET_SIZE,
            terrain.get_asset_y_offset() * ASSET_SIZE,
            terrain.get_asset_width(),
            terrain.get_asset_height(),
            col as f64 * CELL_SIZE,
            row as f64 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        ).unwrap();
    }

    fn draw_object(&self, object: &Box<dyn Object>, row: usize, col: usize) {
        let asset = match object.get_asset_type() {
            AssetType::Environment => &self.env_assets,
            AssetType::Object => &self.obj_assets,
            AssetType::Character => &self.char_assets,
        };
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            asset,
            object.get_asset_x_offset() * ASSET_SIZE,
            object.get_asset_y_offset() * ASSET_SIZE,
            object.get_asset_width(),
            object.get_asset_height(),
            col as f64 * CELL_SIZE,
            row as f64 * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
        ).unwrap();
    }
}