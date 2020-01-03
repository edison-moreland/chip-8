use wasm_bindgen::prelude::*;
// Re-exports a class defined in javascript

#[wasm_bindgen(raw_module = "../js/keyboard.ts")]
extern "C" {
    pub type Keyboard;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Keyboard;

    #[wasm_bindgen(method, getter = keyPressed)]
    pub fn key_pressed(this: &Keyboard) -> i8;
} 