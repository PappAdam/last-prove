#[inline]
pub fn lerp_dt(time: f32, speed: f32) -> f32 {
    1. - speed.powf(time)
}

#[inline]
pub fn lerp(source: f32, target: f32, t: f32) -> f32 {
    if t > 1. {
        return target
    }
    source + (target - source) * t
}
