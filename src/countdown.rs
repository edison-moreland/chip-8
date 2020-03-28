use crate::chip8::traits::Timer;
use std::convert::TryFrom;
use std::u8;
use wasm_bindgen::prelude::*;
// Re-exports a class defined in javascript

#[wasm_bindgen(raw_module = "../js/countdown.ts")]
extern "C" {
    type JSCountdown;

    #[wasm_bindgen(constructor)]
    fn new(period: f64) -> JSCountdown;

    #[wasm_bindgen(method, js_name = cyclesPassed)]
    fn cycles_passed(this: &JSCountdown) -> u32;
}

pub struct Countdown {
    js_countdown: JSCountdown,
}

impl Countdown {
    pub fn new() -> Self {
        Self {
            // 1000 milliseconds / 60 times per second
            js_countdown: JSCountdown::new(1000.0 / 60.0),
        }
    }
}

impl Timer for Countdown {
    fn cycles_passed(&self) -> u8 {
        match u8::try_from(self.js_countdown.cycles_passed()) {
            Ok(cycles) => cycles,
            Err(_) => u8::MAX,
        }
    }
}
