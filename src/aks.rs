use wasm_bindgen::prelude::*;
use js_sys::Array;


#[wasm_bindgen]
pub fn aks_test(n: i32) -> bool {
    // step 1
    if perfect_power(n) {
        return false;
    }
    // step 2
    let r = smallestR(n);
    if gcd(r,n) != 1 {
        return false;
    }
    // step 3
    if aDivN(n,r) {
        return false;
    }
    // step 4
    if n <= r {
        return true;
    }
    // step 5
    return step_5(n,r);
}

// this method computes whether a given number is a perfect power
// step 1
pub fn perfect_power(n: i32) -> bool {
    let max_a = (f64::sqrt(number as f64)) as i32;
    for i in 2..=max_a {
        let mut b = 1;
        while b < n {
            b = b * n;
        }
        if b == n {
            return true;
        }
    }
    return false;
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
fn m_order(a: i32, n: i32) -> Option<i32> {
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
    let mut r = 2;
    let mut ord_side = 0;
    while ord_side <= log_side {
        if let Some(new_ord) = m_order(r,n) {
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
    for (p, k) in &prime_factors {
        prod *= (p-1).pow(k)
    }
    return prod;
}

fn add_or_inc(map: &mut HashMap<i32, i32>, key: i32) {
    let counter = map.entry(key).or_insert(0);
    *counter += 1;
}
// this is probably not the fastest
fn primeFac(n: i32) -> HashMap<i32,i32> {
    let mut pfacs = HashMap::new();
    let mut num = n;
    let mut cp = 2;
    while num != 1 {
        if num % cp == 0 {
            num = num / cp;
            add_or_inc(&mut pfacs, cp);
        }
        else {
            let mut keepLooking = true;
            cp += 1;
            while keepLooking {
                keepLooking = false;
                for p in pfacs.keys() {
                    if cp % p == 0 {
                        keepLooking = true;
                        break
                    }
                }
            }
        }
    }
    return pfacs;
}

fn apply_modxr(vec: &mut Vec<i32>, n: i32, r: i32) {
    while vec.len() > r {
        let a = vec.pop();
        let indx = (vec.len() + 1) % r;
        vec[indx] = (vec[indx] + a) % n;
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
    for 1..n {
        offset_add(&mut vec);
    }
    return vec;
}

// step 5
fn step_5(n: i32, r: i32) -> bool {
    let phi_r = phi(r) as f64;
    let n_log = (n as f64).log(2);
    let a_bound: i32 = ((phi_r).sqrt() * n_log).floor();
    let nth_pascal = generate_pascal(n,r,n);
    for a in 1..=a_bound {
        let mut my_pascal = nth_pascal.copy();
        let mut a_powd = 1;
        for (i, x) in my_pascal.iter_mut().enumerate() {
            *x = (*x * a_powd) % n;
            a_powd = (a_powd * a) % n;
        }
        my_pascal.pop(0);
        my_pascal.pop();
        for x in my_pascal.iter() {
            if x != 0 {
                return false;
            }
        }
    }
    return true;
}