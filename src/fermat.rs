use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn fermat_test(t_prime: i32, aes: js_sys::Array) -> i32 {
    let mut sum = 0;
    for a in aes {
        if let Some(real_val) = a.as_f64() {
            sum += real_val as i32;
        }
    }
    return sum;
}