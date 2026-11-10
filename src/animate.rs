use crate::complex::Complex;
use std::f32::consts;

#[allow(dead_code)]
pub fn transition(f: impl Fn(Complex) -> Complex, z: Complex, p: f64) -> Complex {
    z * (1f64 - p) + f(z) * p
}

#[allow(dead_code)]
pub fn lerp(p: f32) -> f32 {
    p
}

#[allow(dead_code)]
pub fn ease_in_out(p: f32) -> f32 {
    3.0 * f32::powi(p, 2) - 2.0 * f32::powi(p, 3)
}

#[allow(dead_code)]
pub fn wind_in_out(p: f32) -> f32 {
    -2.0 * p + 9.0 * f32::powi(p, 2) - 6.0 * f32::powi(p, 3)
}

#[allow(dead_code)]
pub fn sin_ease(p: f32) -> f32 {
    p - (1.0 / consts::TAU) * f32::sin(consts::TAU * p)
}

#[allow(dead_code)]
pub fn exp_flat(p: f32) -> f32 {
    if p == 0.0 { return 0.0 }
    f32::exp(p - 1.0 / p)
}

#[allow(dead_code)]
pub fn dexp_flat(p: f32) -> f32 {
    if p == 1.0 { return 1.0 }
    1.0 - f32::exp(1.0 - p - 1.0 / (1.0 - p))
}

#[allow(dead_code)]
pub fn exp_sloped(p: f32) -> f32 {
    (f32::exp(3.0 * p) - 1.0) / (f32::exp(3.0) - 1.0)
}

#[allow(dead_code)]
pub fn dexp_sloped(p: f32) -> f32 {
    (f32::exp(-3.0 * p) - 1.0) / (f32::exp(-3.0) - 1.0)
}

#[allow(dead_code)]
pub fn bounce(p: f32) -> f32 {
    p + 0.1 * f32::sin(2.0 * consts::TAU * p)
}

#[allow(dead_code)]
pub fn jump(p: f32) -> f32 {
    if p < 0.2 { return 0.0 }
    if p < 0.4 { return 0.2 }
    if p < 0.6 { return 0.4 }
    if p < 0.8 { return 0.6 }
    if p < 1.0 { return 0.8 }
    1.0
}

#[allow(dead_code)]
pub fn ease_in(p: f32) -> f32 {
    f32::powi(p, 2)
}

#[allow(dead_code)]
pub fn slow_middle(p: f32) -> f32 {
    (f32::sin(consts::TAU * p) + consts::TAU * p) / consts::TAU
}

#[allow(dead_code)]
pub fn smooth(p: f32) -> f32 {
    let a = f32::exp(-1.0 / p);
    let b = f32::exp(-1.0 / (1.0 - p));
    a / (a + b)
}
