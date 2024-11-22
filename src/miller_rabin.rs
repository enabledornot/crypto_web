use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen(getter_with_clone)]
pub struct MillerRabinResult {
    pub d: JsValue,
    pub result: bool,
    pub result_a: Array,
    pub number_result: Array,
}

#[wasm_bindgen]
pub fn miller_rabin_test(t_prime: i32, aes: js_sys::Array) -> MillerRabinResult {
    let (s,d) = find_s_d(t_prime);
    let tested_bools = Array::new();
    let eq_rslts = Array::new();
    let mut is_valid = true;
    for a in aes {
        if let Some(real_val) = a.as_f64() {
            let (tested_bool,eq_rslt) = check_prime(real_val as i32, s, d, t_prime);
            let eq_rslt_js = Array::new();
            for item in eq_rslt {eq_rslt_js.push(&JsValue::from(item));}
            tested_bools.push(&JsValue::from(tested_bool));
            eq_rslts.push(&JsValue::from(eq_rslt_js));
            if !tested_bool {is_valid = false;}
        }
    }
    return MillerRabinResult{
        d: JsValue::from(s),
        result: is_valid,
        result_a: tested_bools,
        number_result: eq_rslts,
    }
}
fn check_prime_test(vec: &Vec<i32>, modulus: i32) -> bool {
    if vec.is_empty() {
        return false;
    }
    if vec[0] != 1 {
        return true;
    }
    for &item in &vec[1..] {
        if item != modulus - 1 {
            return true;
        }
    }
    return false;
}
fn check_prime(a: i32, s: i32, d: i32, modulus: i32) -> (bool, Vec<i32>) {
    let mut rpower = d;
    let mut results: Vec<i32> = Vec::new();
    for _ in 0..s {
        results.push(mod_pow(a,rpower,modulus));
        rpower = (rpower * 2) % modulus;
    }
    return (check_prime_test(&results, modulus),results);
}

fn find_s_d(n: i32) -> (i32, i32) {
    let mut s = 0;
    let mut current_n = n-1;
    while current_n % 2 == 0 {
        s += 1;
        current_n = current_n / 2;
    }
    return (s,current_n);
}

fn mod_pow(number: i32, power: i32, modulus: i32) -> i32 {
    let mut result: i32 = 1;
    let mut current: i32 = number;
    let mut rem: i32 = power % modulus;
    while rem != 0 {
        if rem % 2 == 1 {
            result = (result * current) % modulus;
        }
        rem = rem / 2;
        current = (current * current) % modulus;
    }
    return result;
}