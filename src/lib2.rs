use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sum_numbers_2(a: i32, b: i32) -> i32 {
    return a + b;
}