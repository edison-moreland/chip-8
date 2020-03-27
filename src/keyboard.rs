use crate::chip8::traits::HexKeyboard;

use wasm_bindgen::prelude::*;
// Re-exports a class defined in javascript

#[wasm_bindgen(raw_module = "../js/keyboard.ts")]
extern "C" {
    type KeyboardListener;

    #[wasm_bindgen(constructor)]
    fn new() -> KeyboardListener;

    #[wasm_bindgen(method, getter = keyPressed)]
    fn pressed_key(this: &KeyboardListener) -> i8;
}

pub struct Keyboard {
    listener: KeyboardListener,
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            listener: KeyboardListener::new(),
        }
    }
}

impl HexKeyboard for Keyboard {
    fn pressed_key(&self) -> Option<u8> {
        let key = self.listener.pressed_key();
        return match key {
            0x0..=0xF => Some(key as u8),
            _ => None,
        };
    }
}
