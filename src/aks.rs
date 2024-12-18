use wasm_bindgen::prelude::*;
use js_sys::Array;
use std::collections::HashMap;
use num_bigint::{BigInt, BigUint};
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
#[wasm_bindgen(getter_with_clone)]
pub struct AksResult {
    pub result: bool,
    pub r: u32,
    pub step: u32
}
// This is the main function which is called from javascript
// It receives the question prime and returns the AksResult
// Struct which communicates the data to be displayed on the website
#[wasm_bindgen]
pub fn aks_test(n_in: u32) -> AksResult {
    let n = n_in as u128;
    // step 0
    if n < 2 {
        return AksResult {
            result: false,
            r: 0,
            step: 0,
        };
    }
    // step 1
    if perfect_power(n) {
        return AksResult {
            result: false,
            r: 0,
            step: 1,
        };
    }
    // step 2
    let r = smallestR(n);
    if gcd(r,n) != 1 {
        return AksResult {
            result: false,
            r: r as u32,
            step: 2
        };
    }
    // // step 3
    if aDivN(n,r) {
        return AksResult {
            result: false,
            r: r as u32,
            step: 3,
        };
    }
    // step 4
    if n <= r {
        return AksResult {
            result: true,
            r: r as u32,
            step: 4,
        };
    }
    // step 5
    if step_5(n,r) {
        return AksResult {
            result: true,
            r: r as u32,
            step: 5,
        };
    }
    else {
        return AksResult {
            result: false,
            r: r as u32,
            step: 5,
        };
    }
    // return r;
}

// this method computes whether a given number is a perfect power
// using a binary search approach
// step 1
pub fn perfect_power(n: u128) -> bool {
    let mut e = 2;
    loop {
        if 2_u128.pow(e) > n {
            return false;
        }
        let mut low = 0_u128;
        let mut high = 1_u128;
        while high.pow(e) <= n {
            high = high * 2;
        }
        while high - low > 1 {
            let middle = (low+high) / 2;
            if middle.pow(e) <= n {
                low = middle;
            }
            else {
                high = middle;
            }
        }
        if low.pow(e) == n {
            return true;
        }
        e = e + 1;
    }
    return true;
}
// euclidean algorithm
fn gcd(a_in: u128, b_in: u128) -> u128 {
    let mut a = a_in;
    let mut b = b_in;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

// Multiplicative order
// a^k == 1 mod n
// Uses the brute force method (I dont think there is a quicker way to do this)
pub fn m_order(a: u128, n: u128) -> Option<u128> {
    if gcd(a,n) != 1 {
        return None;
    }
    let mut r = 1;
    for k in 1..n {
        r = (r*a) % n;
        if r == 1 {
            return Some(k);
        }
    }
    return None;
}

// finds the smallest R that satisfies the given conditions
fn smallestR(n: u128) -> u128 {
    let log_side = ((n as f64).log(2.0)).powi(2) as u128;
    let mut r = 1;
    let mut ord_side = 0;
    while ord_side <= log_side {
        r = r + 1;
        if let Some(new_ord) = m_order(n,r) {
            ord_side = new_ord;
        }
    }
    return r;
}

// step 3
// This is simply step 3
fn aDivN(n: u128, r: u128) -> bool {
    for a in 2..=r.min(n-1) {
        if n % a == 0 {
            return true;
        }
    }
    return false;
}
// This calculates phi of n using the provided formula
fn phi(n: u128) -> u128 {
    let prime_factors = primeFac(n);
    let mut prod = 1;
    for (p, &k) in &prime_factors {
        prod *= (p-1).pow(k as u32)
    }
    return prod;
}
// Helper method for prime factorization
fn add_or_inc(map: &mut HashMap<u128, u128>, key: u128) {
    let counter = map.entry(key).or_insert(0);
    *counter += 1;
}
// Finds the prime factorization for a number
// This is needed in order to find phi r
fn primeFac(n: u128) -> HashMap<u128,u128> {
    let mut pfacs = HashMap::new();
    let mut num = n;
    while num % 2 == 0 {
        add_or_inc(&mut pfacs, 2);
        num = num / 2;
    }

    for i in (3..=((num as f64).sqrt() as u128)).step_by(2) {
        while num % i == 0 {
            add_or_inc(&mut pfacs, i);
            num = num / i;
        }
    }
    if n > 2 {
        add_or_inc(&mut pfacs, n);
    }
    return pfacs;
}
// Below are helper functions for generate pascal and step 5
pub fn apply_modxr(vec: &mut Vec<u128>, n: u128, r: u128) {
    while vec.len() > r as usize {
        let a = vec.pop();
        let indx = (vec.len() + 0) % r as usize;
        if let Some(a_real) = a {
            vec[indx] = (vec[indx] + a_real) % n;
        }
    }
}

fn offset_add(vec: &mut Vec<u128>, n: u128) {
    vec.insert(0,vec[0]);
    for i in 2..vec.len() {
        vec[i-1] = (vec[i-1] + vec[i]) % n;
    }
}
// This is the original implementation of generate pascal
// It is still relatively slow since it performs an iteration for each level
pub fn generate_pascal(level: u128, r: u128, n: u128) -> Vec<u128> {
    let mut vec = vec![1,1];
    for i in 1..level {
        offset_add(&mut vec, n);
        apply_modxr(&mut vec, n, r);
    }
    return vec;
}
// This is an alternative version of generate pascal
// It uses an optimised algorithm although needs to make
// Use of bigint which can improve performance but still not fast enough
pub fn generate_pascal_fast(level: u128, r: u128, n: u128) -> Vec<u128> {
    let mut vec = vec![0];
    let mut last: BigInt = 1.into();
    for i in 1..r {
        vec.push(0);
    }
    for k in 0..=level {
        let newVec: BigInt = (vec[(k % r) as usize] + last.clone()) % n;
        match newVec.to_u128() {
            Some(val) => vec[(k % r) as usize] = val,
            None => todo!(),
        }
        last = last * (level-k) / (k+1);
    }
    return vec;
}

// step 5
// This program generates pascals triangle and confirms functionality
// There is a performance issue present in the generate pascal function meaning
// This means it can take a very long time to complete step 5
fn step_5(n: u128, r: u128) -> bool {
    let phi_r = phi(r) as f64;
    let n_log = (n as f64).log(2.0);
    let a_bound: u128 = ((phi_r).sqrt() * n_log).floor() as u128;
    let nth_pascal = generate_pascal_fast(n,r,n);
    for a in 1..=a_bound {
        let mut my_pascal = nth_pascal.clone();
        let mut a_powd = 1;
        for (i, x) in my_pascal.iter_mut().enumerate() {
            *x = (*x * a_powd) % n;
            a_powd = (a_powd * a) % n;
        }
        my_pascal.remove((n % r) as usize);
        my_pascal.remove(0);
        // my_pascal.pop();
        for x in my_pascal.iter() {
            if *x != 0 {
                return false;
            }
        }
    }
    return true;
}