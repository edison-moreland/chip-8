mod screen;
use crate::screen::canvas::Canvas;
use crate::screen::Screen;

mod keyboard;
use crate::keyboard::Keyboard;

// mod timer;
// use crate::timer::Timer;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

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
    let test_rom = Asset::get("test_opcode.ch8").expect("Could not get ROM");
    console::log_1(&JsValue::from_f64(test_rom[0] as f64));

    //let timer = Timer::new(500.0);

    let keyboard = Keyboard::new();
    let mut previous_key: i8 = -1;

    let mut screen = Screen::new_empty(Canvas::new(12, "canvas"));

    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let pressed_key = keyboard.key_pressed();

        if pressed_key != previous_key {
            // Step 1, erase previous sprite
            if previous_key != -1 {
                // Construct sprite slice
                let sprite_offset = screen::character_offset(previous_key as u16) as usize;
                let sprite = &screen::CHIP8_FONT[sprite_offset..sprite_offset + 5];
                screen.write_sprite(0, 0, &sprite);
            }

            // Step 2, write new sprite
            if pressed_key != -1 {
                // Construct sprite slice
                let sprite_offset = screen::character_offset(pressed_key as u16) as usize;
                let sprite = &screen::CHIP8_FONT[sprite_offset..sprite_offset + 5];
                screen.write_sprite(0, 0, &sprite);
            }
            previous_key = pressed_key;
        }

        screen.flush();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
