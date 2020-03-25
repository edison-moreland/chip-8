use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::CanvasRenderingContext2d;

// 32 rows of 64 bits, 1 bit = 1 pixel
pub type RawGrid = [u64; 32];

// Canvas will encapsulate all operations with the canvas on the webpage
// Originally this was implemented in JS, but I had trouble with numbers
// in JS and moving the raw screen data into JS. Ultimatly it was easier
// and only a little slower in Rust. Some hybrid approach where half of
// the class live in JS might work better in the future.
// The JS version still lives in code pen, if curious:
// https://codepen.io/edison-moreland/pen/PowKeLv
pub struct Canvas {
    scale: f64,
    ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(scale: u32, canvas_id: &str) -> Canvas {
        // Some hot garbage to get the canvas element
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        // Make sure canvas is the right size
        // Only supported video mode is 64x32 (for now)
        canvas.set_width(64 * scale);
        canvas.set_height(32 * scale);

        // More hot garbage to get a context
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_fill_style(&JsValue::from_str("rgb(0, 0, 0)"));

        return Canvas {
            scale: scale as f64,
            ctx: context,
        };
    }

    fn clear_screen(&self) {
        self.ctx
            .clear_rect(0.0, 0.0, 64.0 * self.scale, 32.0 * self.scale);
    }

    fn draw_pixel(&self, x: usize, y: usize) {
        self.ctx.fill_rect(
            (x as f64) * self.scale,
            (y as f64) * self.scale,
            self.scale,
            self.scale,
        )
    }

    pub fn draw_grid(&self, screen: &RawGrid) {
        self.clear_screen();

        for (y, scanline) in screen.iter().enumerate() {
            let mut pixel_data = *scanline;

            for x in 0..64 {
                let is_pixel_set = pixel_data & 0x01;
                pixel_data >>= 1;

                if is_pixel_set == 1 {
                    self.draw_pixel(x, y);
                }
            }
        }
    }
}
