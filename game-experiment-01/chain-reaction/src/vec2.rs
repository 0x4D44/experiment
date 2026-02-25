use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// 2D vector for positions, velocities, and forces
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub const ONE: Vec2 = Vec2 { x: 1.0, y: 1.0 };
    pub const UP: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: 1.0 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };

    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            Vec2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            Vec2::ZERO
        }
    }

    #[inline]
    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline]
    pub fn cross(&self, other: &Vec2) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline]
    pub fn perpendicular(&self) -> Self {
        Vec2 {
            x: -self.y,
            y: self.x,
        }
    }

    #[inline]
    pub fn rotate(&self, angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec2 {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }

    #[inline]
    pub fn distance(&self, other: &Vec2) -> f64 {
        (*self - *other).length()
    }

    #[inline]
    pub fn distance_squared(&self, other: &Vec2) -> f64 {
        (*self - *other).length_squared()
    }

    #[inline]
    pub fn lerp(&self, other: &Vec2, t: f64) -> Self {
        Vec2 {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Vec2) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    #[inline]
    fn mul(self, vec: Vec2) -> Vec2 {
        vec * self
    }
}

impl MulAssign<f64> for Vec2 {
    #[inline]
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;

    #[inline]
    fn div(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    #[inline]
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    #[inline]
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_creation() {
        let v = Vec2::new(3.0, 4.0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
    }

    #[test]
    fn test_vec2_length() {
        let v = Vec2::new(3.0, 4.0);
        assert!((v.length() - 5.0).abs() < 1e-10);
        assert!((v.length_squared() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_normalized() {
        let v = Vec2::new(3.0, 4.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < 1e-10);
        assert!((n.x - 0.6).abs() < 1e-10);
        assert!((n.y - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_dot_product() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1.dot(&v2), 11.0);
    }

    #[test]
    fn test_vec2_cross_product() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1.cross(&v2), -2.0);
    }

    #[test]
    fn test_vec2_perpendicular() {
        let v = Vec2::new(3.0, 4.0);
        let p = v.perpendicular();
        assert_eq!(p.x, -4.0);
        assert_eq!(p.y, 3.0);
        assert!((v.dot(&p)).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_rotation() {
        let v = Vec2::new(1.0, 0.0);
        let r = v.rotate(std::f64::consts::PI / 2.0);
        assert!((r.x - 0.0).abs() < 1e-10);
        assert!((r.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_operators() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        let sum = v1 + v2;
        assert_eq!(sum.x, 4.0);
        assert_eq!(sum.y, 6.0);

        let diff = v2 - v1;
        assert_eq!(diff.x, 2.0);
        assert_eq!(diff.y, 2.0);

        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);

        let divided = v2 / 2.0;
        assert_eq!(divided.x, 1.5);
        assert_eq!(divided.y, 2.0);

        let negated = -v1;
        assert_eq!(negated.x, -1.0);
        assert_eq!(negated.y, -2.0);
    }

    #[test]
    fn test_vec2_distance() {
        let v1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert!((v1.distance(&v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_lerp() {
        let v1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(10.0, 20.0);
        let mid = v1.lerp(&v2, 0.5);
        assert_eq!(mid.x, 5.0);
        assert_eq!(mid.y, 10.0);
    }
}
