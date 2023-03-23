use std::f64::consts;

use crate::engine::lin_alg::{Convert, Vector2, Vector3};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Vertex {
    pub fn from_pos(pos: Vector3<f32>) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn new(pos: Vector3<f32>, color: Vector3<f32>) -> Self {
        Self { pos, color }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vector3::default(),
            color: Vector3 {
                x: 1.,
                y: 1.,
                z: 0.3,
            },
        }
    }
}
