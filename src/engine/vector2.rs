use std::{ops::{Add, Sub, Mul, AddAssign}, fmt::Display};

#[derive(Clone, Copy, Default)]
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

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
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
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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