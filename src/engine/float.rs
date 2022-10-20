use std::ops::{Add, Sub, Mul};

impl EngineFloat for f32 { }
impl EngineFloat for f64 { }

pub trait EngineFloat {
    fn lerp<T>(a: T, b: T, t: T) -> T
    where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy
    {
        a + (b - a) * t
    }
}