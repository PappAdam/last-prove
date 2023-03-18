use std::ops::{Add, Sub};

use num::{traits::AsPrimitive, Num};

pub trait Convert<U> {
    fn conv(&self) -> U;
}

/// Vector2
///
/// Implemented functionality:
///     - Add
///     - Subtract
///     - convert between types

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector2<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vector2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Num + Copy + 'static> Convert<Vector2<T>> for [T; 2] {
    #[inline]
    fn conv(&self) -> Vector2<T> {
        unsafe { *(self.as_ptr() as *const Vector2<T>) }
    }
}

impl<T: Num + Copy + 'static> Convert<[T; 2]> for Vector2<T> {
    #[inline]
    fn conv(&self) -> [T; 2] {
        unsafe { *(&self as *const _ as *const [T; 2]) }
    }
}

impl<T: Num + AsPrimitive<U>, U: Num + Copy + 'static> Convert<Vector2<U>> for Vector2<T> {
    #[inline]
    fn conv(&self) -> Vector2<U> {
        Vector2::<U> {
            x: self.x.as_(),
            y: self.y.as_(),
        }
    }
}

impl<T: Num> Add for Vector2<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
        }
    }
}

impl<T: Num> Sub for Vector2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num> std::ops::Mul for Vector2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Num + Copy> std::ops::Mul<T> for Vector2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Vector3
///
/// Implemented functionality:
///     - Add
///     - Subtract
///     - convert between types

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Num> Vector3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Num + AsPrimitive<U>, U: Num + Copy + 'static> Convert<Vector3<U>> for Vector3<T> {
    #[inline]
    fn conv(&self) -> Vector3<U> {
        Vector3::<U> {
            x: self.x.as_(),
            y: self.y.as_(),
            z: self.z.as_(),
        }
    }
}

impl<T: Num> Add for Vector3<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x + self.x,
            y: rhs.y + self.y,
            z: rhs.z + self.z,
        }
    }
}

impl<T: Num> Sub for Vector3<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
