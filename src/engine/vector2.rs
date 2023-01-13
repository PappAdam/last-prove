use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}
impl Add<Vector2> for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<Vector2> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: Into<f32> + Copy> Mul<T> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs.into(), self.y * rhs.into())
    }
}

impl<T: Into<f32> + Copy> MulAssign<T> for Vector2 {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs.into();
        self.y *= rhs.into();
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Self;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: Into<f32> + Copy> Div<T> for Vector2 {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs.into(), self.y / rhs.into())
    }
}

impl<T: Into<f32> + Copy> DivAssign<T> for Vector2 {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs.into();
        self.y /= rhs.into();
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Into<[f32; 2]> for Vector2 {
    fn into(self) -> [f32; 2] {
        [self.x as f32, self.y as f32]
    }
}
impl Into<[u16; 2]> for Vector2 {
    fn into(self) -> [u16; 2] {
        [self.x as u16, self.y as u16]
    }
}
impl Into<[usize; 2]> for Vector2 {
    fn into(self) -> [usize; 2] {
        [self.x as usize, self.y as usize]
    }
}
impl From<PhysicalPosition<f64>> for Vector2 {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}
impl From<PhysicalSize<u32>> for Vector2 {
    fn from(position: PhysicalSize<u32>) -> Self {
        Self {
            x: position.width as f32,
            y: position.height as f32,
        }
    }
}
impl<T> From<[T; 2]> for Vector2
where
    T: Into<f32> + Copy,
{
    fn from(coordinates: [T; 2]) -> Self {
        Self {
            x: coordinates[0].into(),
            y: coordinates[1].into(),
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "x: {}, y: {}", self.x, self.y);
        res
    }
}

impl Vector2 {
    pub fn new<T: Into<f32>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn new_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
        }
    }
    pub fn uniform<T: Into<f32> + Copy>(x: T) -> Self {
        Vector2 {
            x: x.into(),
            y: x.into(),
        }
    }
    pub fn uniform_usize(x: usize) -> Self {
        Vector2 {
            x: x as f32,
            y: x as f32,
        }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance(a: Self, b: Self) -> f32 {
        let a_to_b = (a - b).abs();
        (a_to_b.x * a_to_b.x + a_to_b.y * a_to_b.y).sqrt()
    }
    pub fn distance_squared(a: Self, b: Self) -> f32 {
        let a_to_b = (a - b).abs();
        a_to_b.x * a_to_b.x + a_to_b.y * a_to_b.y
    }
    pub fn lerp(a: Self, b: Self, t: f32) -> Self {
        if Vector2::distance_squared(a, b) < 0.0000001 {
            return b;
        }

        let x = a.x + (b.x - a.x) * t;
        let y = a.y + (b.y - a.y) * t;
        Self::new(x, y)
    }
    pub fn abs(&self) -> Self {
        Vector2::new(self.x.abs(), self.y.abs())
    }
    pub fn round(&self) -> Self {
        Vector2::new(self.x.round(), self.y.round())
    }
}

//float implementations
impl Div<Vector2> for f32 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self / rhs.x, self / rhs.y)
    }
}
