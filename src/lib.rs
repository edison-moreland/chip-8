mod screen;
use crate::screen::{Canvas, Screen};

mod keyboard;
use crate::keyboard::Keyboard;

mod chip8;
use crate::chip8::Chip8;
use crate::chip8::traits::Drawable;

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
    let test_rom = Asset::get("test_opcode.ch8")
        .expect("Could not get ROM")
        .into_owned();

    let screen = Box::new(Screen::new_empty(Canvas::new(12, "canvas")));

    let mut chip8 = Chip8::new(screen);
    match chip8.init_memory(&test_rom[..]) {
        Ok(_) => {}
        Err(e) => {
            console::warn_1(&JsValue::from(e.to_string()));
            return Err(JsValue::from(e.to_string()));
        }
    }

    

    // https://rustwasm.github.io/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        match chip8.step_execution() {
            Ok(_) => {}
            Err(e) => {
                console::warn_1(&JsValue::from(e.to_string()));
                return;
            }
        };

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
