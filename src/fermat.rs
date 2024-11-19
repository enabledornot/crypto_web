use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen(getter_with_clone)]
pub struct FermantResult {
    pub result: bool,
    pub pow_result: Array,
}

#[wasm_bindgen]
pub fn fermat_test(t_prime: i32, aes: js_sys::Array) -> FermantResult {
    let mut all_a: Vec<i32> = Vec::new();
    for a in aes {
        if let Some(real_val) = a.as_f64() {
            all_a.push(real_val as i32);
        }
    }
    let new_a = Array::new();
    // let mut new_a: Vec<i32> = Vec<i32>::new();
    for a in all_a {
        let result = mod_pow(a,t_prime-1,t_prime);
        new_a.push(&JsValue::from(result));
    }
    let result = new_a.iter().all(|x| x == 1);
    return FermantResult{
        result: result,
        pow_result: new_a,
    };
}

fn mod_pow(number: i32, power: i32, modulus: i32) -> i32 {
    let mut result: i32 = 1;
    let mut current: i32 = number;
    let mut rem: i32 = power;
    while rem != 0 {
        if rem % 2 == 1 {
            result = (result * current) % modulus;
        }
        rem = rem / 2;
        current = (current * current) % modulus;
    }
    return result;
}