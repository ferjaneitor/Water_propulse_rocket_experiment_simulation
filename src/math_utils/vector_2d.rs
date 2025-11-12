// math.rs
use core::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub const ZERO: Self   = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }


    #[inline]
    pub fn magnitude(self) -> f64 { self.length() }

    #[inline]
    pub fn length(self) -> f64 { self.x.hypot(self.y) } // more stable than sqrt(x^2+y^2)

}

// Operator overloads (ergonomic and fast)
impl Add for Vector2D {
    type Output = Self;
    #[inline] fn add(self, rhs: Self) -> Self { Self::new(self.x + rhs.x, self.y + rhs.y) }
}
impl AddAssign for Vector2D {
    #[inline] fn add_assign(&mut self, rhs: Self) { self.x += rhs.x; self.y += rhs.y; }
}
impl Sub for Vector2D {
    type Output = Self;
    #[inline] fn sub(self, rhs: Self) -> Self { Self::new(self.x - rhs.x, self.y - rhs.y) }
}
impl SubAssign for Vector2D {
    #[inline] fn sub_assign(&mut self, rhs: Self) { self.x -= rhs.x; self.y -= rhs.y; }
}
impl Mul<f64> for Vector2D {
    type Output = Self;
    #[inline] fn mul(self, s: f64) -> Self { Self::new(self.x * s, self.y * s) }
}
impl MulAssign<f64> for Vector2D {
    #[inline] fn mul_assign(&mut self, s: f64) { self.x *= s; self.y *= s; }
}
impl Div<f64> for Vector2D {
    type Output = Self;
    #[inline] fn div(self, s: f64) -> Self { Self::new(self.x / s, self.y / s) }
}
impl DivAssign<f64> for Vector2D {
    #[inline] fn div_assign(&mut self, s: f64) { self.x /= s; self.y /= s; }
}
impl Neg for Vector2D {
    type Output = Self;
    #[inline] fn neg(self) -> Self { Self::new(-self.x, -self.y) }
}

impl From<(f64, f64)> for Vector2D {
    fn from(t: (f64, f64)) -> Self { Self::new(t.0, t.1) }
}
