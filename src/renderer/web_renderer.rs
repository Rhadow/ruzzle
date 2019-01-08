use wasm_bindgen::JsCast;
use web_sys::console::log_1;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};
use crate::game::World;
use crate::game::Cell;

pub struct WebRenderer {
    ctx: CanvasRenderingContext2d,
    assets: HtmlImageElement
}

impl WebRenderer {
    pub fn new(canvas: &HtmlCanvasElement, assets: HtmlImageElement) -> WebRenderer {
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        WebRenderer {
            ctx,
            assets
        }
    }

    pub fn render(&self, world: &World) {
        let width = world.width();
        let height = world.height();
        let cells = world.cells();

        for row in 0..width {
            for col in 0..height {
                let idx = world.get_index(row, col);
                let cell = &cells[idx];
                self.draw_cell(cell, row, col);
            }
        }
    }

    fn draw_cell(&self, cell: &Cell, row: usize, col: usize) {
        let size: f64 = 64f64;
        let step: f64 = 16f64;
        self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &self.assets,
            5f64 * step,
            9f64 * step,
            step,
            step,
            row as f64 * size,
            col as f64 * size,
            size,
            size,
        );
    }
}