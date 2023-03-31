use nalgebra_glm::Vec3;

#[derive(Clone, Copy, Default)]
pub struct Cube {
    pub location: Vec3,
}

impl Cube {
    pub fn new(location: Vec3) -> Self {
        Self { location }
    }
}
