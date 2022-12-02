use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
};

use winit::dpi::PhysicalPosition;

#[derive(Clone, Copy, Default, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
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

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
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
impl From<PhysicalPosition<f64>> for Vector2 
{
    fn from(position: PhysicalPosition<f64>) -> Self {
        Vector2 { x: position.x as f32, y: position.y as f32 }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "x: {}, y: {}", self.x, self.y);
        res
    }
}

#[allow(dead_code)]
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
    pub fn from<T: Into<f32> + Copy>(coordinates: [T; 2]) -> Self {
        Self {
            x: coordinates[0].into(),
            y: coordinates[1].into(),
        }
    }
    pub fn from_usize(coordinates: [usize; 2]) -> Self {
        Self {
            x: coordinates[0] as f32,
            y: coordinates[1] as f32,
        }
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
        let x = a.x + (b.x - a.x) * t;
        let y = a.y + (b.y - a.y) * t;
        Self::new(x, y)
    }
    pub fn abs(&self) -> Self {
        Vector2::new(self.x.abs(), self.y.abs())
    }
}
