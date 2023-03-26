use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use num::{traits::{real::Real, AsPrimitive}, Num};
use winit::dpi::{PhysicalPosition, PhysicalSize};

#[derive(Clone, Copy, Default, Debug)]
#[repr(C)]
pub struct Vector2<T>
where
    T: Num + Copy,
{
    pub x: T,
    pub y: T,
}
// impl Index<u8> for Vector2 {
//     type Output = f32;

//     #[inline]
//     fn index(&self, index: u8) -> &Self::Output {
//         unsafe { &(&*(self as *const _ as *const [f32; 2]))[index as usize] }
//     }
// }
impl<T: Num + Copy + Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2::new(-self.x, -self.y)
    }
}
impl<T: Num + Copy> Add<Vector2<T>> for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<T: Num + Copy + AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Num + Copy> Sub<Vector2<T>> for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Num + Copy + SubAssign> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Num + Copy> Mul<Vector2<T>> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: Num + Copy> Mul<T> for Vector2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Real + Copy> Div<Vector2<T>> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T: Real + Copy> Div<T> for Vector2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Vector2::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Num + Copy + DivAssign> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Num + Copy> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: Num + Copy + AsPrimitive<U>, U: Num + Copy + 'static> Into<[U; 2]> for Vector2<T> {
    fn into(self) -> [U; 2] {
        [self.x.as_(), self.y.as_()]
    }
}
impl<T: Num + Copy + AsPrimitive<U>, U: Num + Copy + 'static> From<[T; 2]> for Vector2<U> {
    fn from(array: [T; 2]) -> Self {
        Vector2::new(array[0].as_(), array[1].as_())
    }
}
impl From<PhysicalPosition<f64>> for Vector2<f32> {
    fn from(position: PhysicalPosition<f64>) -> Self {
        Self {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}
impl From<PhysicalSize<u32>> for Vector2<u16> {
    fn from(position: PhysicalSize<u32>) -> Self {
        Self {
            x: position.width as u16,
            y: position.height as u16,
        }
    }
}

impl<T: Num + Copy + Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = write!(f, "x: {}, y: {}", self.x, self.y);
        res
    }
}
impl Into<Vector2<f32>> for Vector2<usize> {
    fn into(self) -> Vector2<f32> {
        Vector2::new(self.x as f32, self.y as f32)
    }
}
impl Into<Vector2<u16>> for Vector2<usize> {
    fn into(self) -> Vector2<u16> {
        Vector2::new(self.x as u16, self.y as u16)
    }
}

impl<T> Vector2<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn uniform(x: T) -> Self {
        Vector2 { x, y: x }
    }
}

impl<T: Real + Copy> Vector2<T> {
    pub fn zero() -> Self {
        Self {
            x: num::zero(),
            y: num::zero(),
        }
    }

    pub fn distance(a: Self, b: Self) -> T {
        Self::distance_squared(a, b).sqrt()
    }
    pub fn distance_squared(a: Self, b: Self) -> T {
        let a_to_b = (a - b).abs();
        a_to_b.x * a_to_b.x + a_to_b.y * a_to_b.y
    }
    pub fn lerp(a: Self, b: Self, t: T) -> Self {
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

pub trait Convert<T: Num + Copy + AsPrimitive<U>, U: Num + Copy + 'static> {
    fn convert(&self) -> Vector2<U>;
}

impl<T: Num + Copy + AsPrimitive<U>, U: Num + Copy + 'static> Convert<T, U> for Vector2<T> {
    fn convert(&self) -> Vector2<U> {
        Vector2::new(self.x.as_(), self.y.as_())
    }
}