use crate::screen::{Canvas, Screen};

use crate::keyboard::Keyboard;

use crate::chip8::Chip8;

use crate::countdown::Countdown;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use std::cell::RefCell;
use std::rc::Rc;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

pub fn run_emulator(rom: &[u8]) -> Result<(), String> {
    // Initialize emulator
    let timer = Box::new(Countdown::new());

    let keyboard = Box::new(Keyboard::new());

    let screen = Box::new(Screen::new_empty(Canvas::new(12, "canvas")));

    let mut chip8 = Chip8::new(screen, keyboard, timer);
    match chip8.init_memory(rom) {
        Ok(_) => {}
        Err(e) => return Err(e.to_string()),
    }

    // Step execution on animation frame
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
