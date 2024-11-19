use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;
use std::panic;

pub mod fermat;

#[wasm_bindgen]
pub fn sum_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn init_rust() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}