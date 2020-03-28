mod chip8;
mod countdown;
mod keyboard;
mod screen;

use wasm_bindgen::prelude::*;
use web_sys::console;

use std::collections::HashMap;

use url::Url;

extern crate console_error_panic_hook;
use std::panic;

mod start;
use start::run_emulator;

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

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Catch any panics that occur and report them to javascript
    panic::set_hook(Box::new(|info| {
        console::error_1(&JsValue::from(format!("{}", info)));
        panic_handler();
    }));

    let rom = Asset::get(&format!("{}.ch8", get_rom_name()))
        .expect("Could not get ROM")
        .into_owned();

    match run_emulator(&rom[..]) {
        Ok(_) => Ok(()),
        Err(e) => Err(JsValue::from(e)),
    }
}

fn get_rom_name() -> String {
    // get rom name from url query defaulting to "test_opcode"
    let href = web_sys::window().unwrap().location().href().unwrap();
    let parsed_url = Url::parse(&href).expect("could not parse url");

    let query: HashMap<String, String> = parsed_url.query_pairs().into_owned().collect();

    let rom_name = query
        .get("rom")
        .unwrap_or(&String::from("test_opcode/test_opcode"))
        .to_owned();

    return rom_name;
}
