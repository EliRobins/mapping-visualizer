use macroquad::prelude::*;
use std::{fmt, ops};

#[derive(Default, PartialEq, Debug, Copy, Clone)]
pub struct Complex {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Complex {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn exp(z: Self) -> Self {
        Self::new(f64::cos(z.y), f64::sin(z.y)) * f64::exp(z.x)
    }

    pub fn log(z: Self) -> Self {
        Self::new(f64::ln(z.magnitude()), f64::atan2(z.y, z.x))
    }

    pub fn pow(base: Self, exponent: Self) -> Self {
        Self::exp(exponent * Self::log(base))
    }

    pub fn cos(z: Self) -> Self {
        (Self::exp(z * Self::new(0f64, 1f64)) + Self::exp(z * Self::new(0f64, -1f64))) / Complex::new(2f64, 0f64)
    }

    pub fn sin(z: Self) -> Self {
        (Self::exp(z * Self::new(0f64, 1f64)) - Self::exp(z * Self::new(0f64, -1f64))) / Complex::new(0f64, 2f64)
    }

    pub fn tan(z: Self) -> Self {
        Self::sin(z) / Self::cos(z)
    }

    pub fn sec(z: Self) -> Self {
        Self::from(1f64) / Self::cos(z)
    }

    pub fn csc(z: Self) -> Self {
        Self::from(1f64) / Self::sin(z)
    }

    pub fn cot(z: Self) -> Self {
        Self::cos(z) / Self::sin(z)
    }

    pub fn cosh(z: Self) -> Self {
        (Self::exp(z) + Self::exp(-z)) / Complex::from(2f64)
    }

    pub fn sinh(z: Self) -> Self {
        (Self::exp(z) - Self::exp(-z)) / Complex::from(2f64)
    }

    pub fn tanh(z: Self) -> Self {
        Self::sinh(z) / Self::cosh(z)
    }

    pub fn sech(z: Self) -> Self {
        Self::from(1f64) / Self::cosh(z)
    }

    pub fn csch(z: Self) -> Self {
        Self::from(1f64) / Self::sinh(z)
    }

    pub fn coth(z: Self) -> Self {
        Self::cosh(z) / Self::sinh(z)
    }

    pub const fn square_distance(z: Self, w: Self) -> f64 {
        (z.x - w.x) * (z.x - w.x) + (z.y - w.y) * (z.y - w.y)
    }

    pub fn distance(z: Self, w: Self) -> f64 {
        f64::sqrt(Self::square_distance(z, w))
    }

    pub const fn conjugate(&self) -> Self {
        Self::new(self.x, -self.y)
    }
    
    pub const fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.square_magnitude())
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3} + {:.3}i", self.x, self.y)
    }
}

impl ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl ops::MulAssign<Complex> for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl ops::Div<Complex> for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        let denominator = other.square_magnitude();
        Self {
            x: (self.x * other.x + self.y * other.y) / denominator,
            y: (self.x * other.y - self.y * other.x) / denominator,
        }
    }
}

impl ops::DivAssign<Complex> for Complex {
    fn div_assign(&mut self, other: Self) {
        let denominator = other.square_magnitude();
        *self = Self {
            x: (self.x * other.x + self.y * other.y) / denominator,
            y: (self.x * other.y - self.y * other.x) / denominator,
        }
    }
}

impl ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::MulAssign<f64> for Complex {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::Div<f64> for Complex {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl ops::DivAssign<f64> for Complex {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl From<Vec2> for Complex {
    fn from(vec: Vec2) -> Self {
        Self {
            x: f64::from(vec.x),
            y: f64::from(vec.y),
        }
    }
}

impl Into<Vec2> for Complex {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}

impl From<f64> for Complex {
    fn from(x: f64) -> Self {
        Self {
            x: x,
            y: 0f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjugate() {
        let z = Complex::new(4f64, 2f64);
        assert_eq!(z.conjugate(), Complex::new(4f64, -2f64));
    }
}
