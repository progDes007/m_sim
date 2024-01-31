use crate::prelude::*;
use crate::math_core::*;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const UNIT_X: Self = Self { x: 1.0, y: 0.0 };
    pub const UNIT_Y: Self = Self { x: 0.0, y: 1.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn length_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    pub fn approx_eq(&self, other: Self, epsilon: f64) -> bool {
        approx_eq(self.x, other.x, epsilon) && approx_eq(self.y, other.y, epsilon)
    }

    pub fn is_unit(&self) -> bool {
        approx_eq(self.length_sq(), 1.0, DISTANCE_EPS_SQ)
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Self) -> f64 {
        self.x * other.y - self.y * other.x
    }

    pub fn rotated_90_ccw(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    pub fn rotated_90_cw(&self) -> Self {
        Self::new(self.y, -self.x)
    }

}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}


impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(self * rhs.x, self * rhs.y)
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}


impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn test_length() {
        assert!(approx_eq(Vec2::new(3.0, 4.0).length(), 5.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(0.0, 0.0).length(), 0.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(-3.0, 4.0).length(), 5.0, DISTANCE_EPS));
    }
    #[test]
    fn test_length_sq() {
        assert!(approx_eq(Vec2::new(3.0, 4.0).length_sq(), 25.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(0.0, 0.0).length_sq(), 0.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(-3.0, 4.0).length_sq(), 25.0, DISTANCE_EPS));
    }
    #[test]
    fn test_mul_scalar() {
        assert!((Vec2::new(3.0, 4.0) * 2.0).approx_eq(Vec2::new(6.0, 8.0), DISTANCE_EPS));
        assert!((Vec2::new(3.0, 4.0) * 0.0).approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));
        assert!((Vec2::new(3.0, 4.0) * -1.0).approx_eq(Vec2::new(-3.0, -4.0), DISTANCE_EPS));

        assert!((2.0 * Vec2::new(3.0, 4.0)).approx_eq(Vec2::new(6.0, 8.0), DISTANCE_EPS));
        assert!((0.0 * Vec2::new(3.0, 4.0)).approx_eq(Vec2::new(0.0, 0.0), DISTANCE_EPS));
        assert!((-1.0 * Vec2::new(3.0, 4.0)).approx_eq(Vec2::new(-3.0, -4.0), DISTANCE_EPS));
    } 

    #[test]
    fn test_neg() {
        assert!((-Vec2::new(3.0, 4.0)).approx_eq(Vec2::new(-3.0, -4.0), DISTANCE_EPS));
    }

    #[test]
    fn test_div_scalar() {
        assert!((Vec2::new(3.0, 4.0) / 2.0).approx_eq(Vec2::new(1.5, 2.0), DISTANCE_EPS));
        assert!((Vec2::new(3.0, 4.0) / 1.0).approx_eq(Vec2::new(3.0, 4.0), DISTANCE_EPS));
        assert!((Vec2::new(3.0, 4.0) / -1.0).approx_eq(Vec2::new(-3.0, -4.0), DISTANCE_EPS));
    }

    #[test]
    fn test_dot() {
        assert!(approx_eq(Vec2::new(3.0, 4.0).dot(Vec2::new(5.0, 6.0)), 39.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(3.0, 4.0).dot(Vec2::new(0.0, 0.0)), 0.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(3.0, 4.0).dot(Vec2::new(-5.0, -6.0)), -39.0, DISTANCE_EPS));
    }

    #[test]
    fn test_cross() {
        assert!(approx_eq(Vec2::new(1.0, 0.0).cross(Vec2::new(1.0, 0.0)), 0.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(1.0, 0.0).cross(Vec2::new(0.0, 2.0)), 2.0, DISTANCE_EPS));
        assert!(approx_eq(Vec2::new(1.0, 0.0).cross(Vec2::new(0.0, -2.0)), -2.0, DISTANCE_EPS));
    }
}
