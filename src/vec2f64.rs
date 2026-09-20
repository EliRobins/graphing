use macroquad::prelude::*;
use std::{fmt, ops};

#[derive(Default, Debug, Copy, Clone)]
pub struct Vec2f64 {
    pub x: f64,
    pub y: f64,
}

impl Vec2f64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Vec2f64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Neg for Vec2f64 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl ops::Add for Vec2f64 {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::AddAssign for Vec2f64 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2f64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::SubAssign for Vec2f64 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<Vec2f64> for Vec2f64 {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::MulAssign<Vec2f64> for Vec2f64 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl ops::Div<Vec2f64> for Vec2f64 {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl ops::DivAssign<Vec2f64> for Vec2f64 {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl ops::Mul<f64> for Vec2f64 {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::MulAssign<f64> for Vec2f64 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl ops::Div<f64> for Vec2f64 {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl ops::DivAssign<f64> for Vec2f64 {
    fn div_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl From<Vec2> for Vec2f64 {
    fn from(vec: Vec2) -> Self {
        Self {
            x: f64::from(vec.x),
            y: f64::from(vec.y),
        }
    }
}

impl Into<Vec2> for Vec2f64 {
    fn into(self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }
}
