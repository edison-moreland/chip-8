mod canvas_grid;
use crate::canvas_grid::{
    Canvas
};

mod screen;
use crate::screen::{
    Screen
};

mod keyboard;
use crate::keyboard::{
    Keyboard
};


mod timer;
use crate::timer::{
    Timer
};

use wasm_bindgen::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::console;

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "static/roms/"]
struct Asset;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let test_rom = Asset::get("test_opcode.ch8").expect("Could not get ROM");
    console::log_1(&JsValue::from_f64(test_rom[0] as f64));

    // Here we want to call `requestAnimationFrame` in a loop, but only a fixed
    // number of times. After it's done we want all our resources cleaned up. To
    // achieve this we're using an `Rc`. The `Rc` will eventually store the
    // closure we want to execute on each frame, but to start out it contains
    // `None`.
    //
    // After the `Rc` is made we'll actually create the closure, and the closure
    // will reference one of the `Rc` instances. The other `Rc` reference is
    // used to store the closure, request the first frame, and then is dropped
    // by this function.
    //
    // Inside the closure we've got a persistent `Rc` reference, which we use
    // for all future iterations of the loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    // Keyboard and sprites test
    let mut screen = Screen::new_empty();
    
    let mut grid = Canvas::new(12, "canvas");

    let keyboard = Keyboard::new();

    let timer = Timer::new(500.0);

    let mut previous_key: i8 = -1;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let pressed_key = keyboard.key_pressed();
        grid.debug_msg(format!("Key Pressed: {}", pressed_key));

        let ticks = timer.ticks_passed();
        grid.debug_msg(format!("Ticks passed: {}", ticks));

        if pressed_key == previous_key {
            // Schedule another frame and bounce
            grid.draw_grid(&screen.as_raw());
            request_animation_frame(f.borrow().as_ref().unwrap());
            return;
        }

        // Step 1, erase previous sprite
        if previous_key != -1 {
            // Construct sprite slice
            let sprite_offset = screen::character_offset(previous_key as u16) as usize;
            let sprite = &screen::CHIP8_FONT[sprite_offset..sprite_offset+5];
            screen.write_sprite(0, 0, &sprite);
        }

        if pressed_key != -1 {
            // Step 2, write new sprite
            // Construct sprite slice
            let sprite_offset = screen::character_offset(pressed_key as u16) as usize;
            let sprite = &screen::CHIP8_FONT[sprite_offset..sprite_offset+5];
            screen.write_sprite(0, 0, &sprite);
        }
        previous_key = pressed_key;

        // Schedule ourself for another requestAnimationFrame callback.
        grid.draw_grid(&screen.as_raw());
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}