use nalgebra::Vector3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub color: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl Vertex {
    pub fn from_pos(pos: Vector3<f32>) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn new(pos: Vector3<f32>, color: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, color, normal }
    }
    pub const fn new_const(pos: Vector3<f32>, color: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self { pos, color, normal }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vector3::default(),
            color: Vector3::new(1., 1., 0.3),
            normal: Vector3::new(1., 1., 1.),
        }
    }
}
