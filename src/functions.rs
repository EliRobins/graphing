pub fn weierstrass(x: f64) -> f64 {
    const ITERATIONS: u64 = 100;
    let mut sum = 0.0;
    let mut product = 1.0;
    for _ in 1..ITERATIONS {
        sum += f64::cos(product * x) / product;
        product *= 2.0;
    }
    sum
}
