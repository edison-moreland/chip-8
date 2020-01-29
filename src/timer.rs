use wasm_bindgen::prelude::*;
// Re-exports a class defined in javascript

#[wasm_bindgen(raw_module = "../js/timer.ts")]
extern "C" {
    pub type Timer;

    #[wasm_bindgen(constructor)]
    pub fn new(period: f64) -> Timer;

    #[wasm_bindgen(method, js_name = ticksPassed)]
    pub fn ticks_passed(this: &Timer) -> u32;
}