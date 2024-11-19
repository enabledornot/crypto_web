use wasm_bindgen::prelude::*;

pub mod fermat;

#[wasm_bindgen]
pub fn sum_numbers(a: i32, b: i32) -> i32 {
    return a + b;
}