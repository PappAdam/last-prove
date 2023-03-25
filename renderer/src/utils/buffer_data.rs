use std::ffi::c_void;

use nalgebra_glm::TMat4;

use crate::engine::lin_alg::{Convert, Vector2, Vector3};
#[derive(Debug)]
pub struct Transform {
    pub rotation: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: Vector3::default(),
        }
    }

    #[inline]
    pub fn as_void_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
}

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
