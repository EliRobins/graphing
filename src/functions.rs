use std::f64::consts;

pub fn weierstrass(x: f64) -> f64 {
    const ITERATIONS: u64 = 60;
    let mut sum = 0.0;
    let mut product = 1.0;
    for _ in 1..ITERATIONS {
        sum += f64::cos(product * x) / product;
        product *= 2.0;
    }
    sum
}

pub fn square_wave(x: f64) -> f64 {
    const ITERATIONS: u64 = 50;
    let mut sum = 0.0;
    let mut odd = 1.0;
    let mut sign = true;
    for _ in 1..ITERATIONS {
        if sign {
            sum += f64::cos(odd * x) / odd
        } else {
            sum -= f64::cos(odd * x) / odd
        }
        odd += 2.0;
        sign = !sign;
    }
    sum
}
