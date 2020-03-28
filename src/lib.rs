mod screen;
use crate::screen::{Canvas, Screen};

mod keyboard;
use crate::keyboard::Keyboard;

mod chip8;
use crate::chip8::Chip8;

mod countdown;
use crate::countdown::Countdown;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use url::Url;

extern crate console_error_panic_hook;
use std::panic;

#[wasm_bindgen(raw_module = "../js/panic_handler.ts")]
extern "C" {
    #[wasm_bindgen(js_name = panicHandler)]
    fn panic_handler();
}

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

pub fn run_emulator() -> Result<(), JsValue> {
    let rom = Asset::get(&format!("{}.ch8", get_rom_name()))
        .expect("Could not get ROM")
        .into_owned();

    let timer = Box::new(Countdown::new());

    let keyboard = Box::new(Keyboard::new());

    let screen = Box::new(Screen::new_empty(Canvas::new(12, "canvas")));

    let mut chip8 = Chip8::new(screen, keyboard, timer);
    match chip8.init_memory(&rom[..]) {
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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Catch any panics that occur and report them to javascript
    panic::set_hook(Box::new(|info| {
        console::error_1(&JsValue::from(format!("{}", info)));
        panic_handler();
    }));

    run_emulator()
}
