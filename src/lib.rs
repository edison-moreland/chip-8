mod screen;
use crate::screen::{Canvas, Screen};

mod keyboard;
use crate::keyboard::Keyboard;

mod chip8;
use crate::chip8::traits::Drawable;
use crate::chip8::Chip8;

// mod timer;
// use crate::timer::Timer;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::cell::RefCell;
use std::rc::Rc;

use web_sys::console;

use std::collections::HashMap;

use url::Url;

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

fn get_rom_name() -> String {
    // get rom name from url query defaulting to "test_opcode"
    let parsed_url = Url::parse(&window().location().href().unwrap()).expect("could not parse url");

    let query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let rom_name = query
        .get("rom")
        .unwrap_or(&String::from("test_opcode/test_opcode"))
        .to_owned();

    return rom_name;
}

// This function is automatically invoked after the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let test_rom = Asset::get(&format!("{}.ch8", get_rom_name()))
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
