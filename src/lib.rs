mod canvas_grid;
use crate::canvas_grid::{
    Canvas
};

mod screen;
use crate::screen::{
    Screen
};

use wasm_bindgen::prelude::*;
//use web_sys::console;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js(){
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Draw a checkerboard
    let mut screen = Screen::new_empty();
    let grid = Canvas::new(4, "canvas");

    // Sprite test
    for i in 0..16 {
        // Construct sprite slice
        let sprite_offset = screen::character_offset(i);
        let sprite = &screen::CHIP8_FONT[sprite_offset..sprite_offset+5];

        // If character leaves the screen, knock it down a line
        let mut x = i*8;
        let y;
        if x > 56 {
            x -= 64;
            y = 5;
        } else {
            y = 0;
        }

        screen.write_sprite(x as usize, y, sprite);
    }

    grid.draw_grid(&screen.as_raw());

}
