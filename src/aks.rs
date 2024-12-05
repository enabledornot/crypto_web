use wasm_bindgen::prelude::*;
use js_sys::Array;
use std::collections::HashMap;

#[wasm_bindgen(getter_with_clone)]
pub struct AksResult {
    pub result: bool,
    pub r: i32,
    pub step: i32
}

#[wasm_bindgen]
pub fn aks_test(n: i32) -> AksResult {
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
            r: r,
            step: 2
        };
    }
    // // step 3
    if aDivN(n,r) {
        return AksResult {
            result: false,
            r: r,
            step: 3,
        };
    }
    // step 4
    if n <= r {
        return AksResult {
            result: true,
            r: r,
            step: 4,
        };
    }
    // step 5
    if step_5(n,r) {
        return AksResult {
            result: true,
            r: r,
            step: 5,
        };
    }
    else {
        return AksResult {
            result: false,
            r: r,
            step: 5,
        };
    }
    // return r;
}

// this method computes whether a given number is a perfect power
// step 1
pub fn perfect_power(n: i32) -> bool {
    let mut e = 2;
    loop {
        if 2_i32.pow(e) > n {
            return false;
        }
        let mut low = 0_i32;
        let mut high = 1_i32;
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
// euclidian algortihm
fn gcd(a_in: i32, b_in: i32) -> i32 {
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
pub fn m_order(a: i32, n: i32) -> Option<i32> {
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

// part of step 2
fn smallestR(n: i32) -> i32 {
    let log_side = ((n as f64).log(2.0)).powi(2) as i32;
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
fn aDivN(n: i32, r: i32) -> bool {
    for a in 2..=r.min(n-1) {
        if n % a == 0 {
            return true;
        }
    }
    return false;
}

fn phi(n: i32) -> i32 {
    let prime_factors = primeFac(n);
    let mut prod = 1;
    for (p, &k) in &prime_factors {
        prod *= (p-1).pow(k as u32)
    }
    return prod;
}

fn add_or_inc(map: &mut HashMap<i32, i32>, key: i32) {
    let counter = map.entry(key).or_insert(0);
    *counter += 1;
}

fn primeFac(n: i32) -> HashMap<i32,i32> {
    let mut pfacs = HashMap::new();
    let mut num = n;
    while num % 2 == 0 {
        add_or_inc(&mut pfacs, 2);
        num = num / 2;
    }

    for i in (3..=((num as f64).sqrt() as i32)).step_by(2) {
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

fn apply_modxr(vec: &mut Vec<i32>, n: i32, r: i32) {
    while vec.len() > r as usize {
        let a = vec.pop();
        let indx = (vec.len() + 1) % r as usize;
        if let Some(a_real) = a {
            vec[indx] = (vec[indx] + a_real) % n;
        }
    }
}

fn offset_add(vec: &mut Vec<i32>, n: i32) {
    vec.insert(0,vec[0]);
    for i in 2..vec.len() {
        vec[i-1] = (vec[i-1] + vec[i]) % n;
    }
}

fn generate_pascal(level: i32, r: i32, n: i32) -> Vec<i32> {
    let mut vec = vec![1,1];
    for _ in 1..n {
        offset_add(&mut vec, n);
    }
    return vec;
}

// step 5
fn step_5(n: i32, r: i32) -> bool {
    let phi_r = phi(r) as f64;
    let n_log = (n as f64).log(2.0);
    let a_bound: i32 = ((phi_r).sqrt() * n_log).floor() as i32;
    let nth_pascal = generate_pascal(n,r,n);
    for a in 1..=a_bound {
        let mut my_pascal = nth_pascal.clone();
        let mut a_powd = 1;
        for (i, x) in my_pascal.iter_mut().enumerate() {
            *x = (*x * a_powd) % n;
            a_powd = (a_powd * a) % n;
        }
        my_pascal.remove(0);
        my_pascal.pop();
        for x in my_pascal.iter() {
            if *x != 0 {
                return false;
            }
        }
    }
    return true;
}