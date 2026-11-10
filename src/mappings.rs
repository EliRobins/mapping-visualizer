use crate::complex::Complex;

#[allow(dead_code)]
pub fn zeta(z: Complex) -> Complex {
    if z.x <= 1f64 { return Complex::new(f64::NAN, f64::NAN) }

    const ITERATIONS: usize = 100;
    let mut sum = Complex::from(0f64);

    for k in 1..=ITERATIONS {
        sum += Complex::pow(Complex::from(1f64 / k as f64), z);
    }

    sum
}
