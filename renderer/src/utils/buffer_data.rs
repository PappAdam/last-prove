use std::{
    ffi::c_void,
    ops::{BitAnd, BitOr},
};

use nalgebra_glm::{Mat4, TVec3, Vec3};

pub trait BufferObject {
    fn as_void_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
}

#[derive(Debug)]
pub struct Transform {
    pub view: Mat4,
    pub rotation: Mat4,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            view: Mat4::identity(),
            rotation: Mat4::identity(),
        }
    }
}

impl BufferObject for Transform {}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub color: Vec3,
    pub normal: Vec3,
    pub wave_multip: f32,
}

impl Vertex {
    pub fn from_pos(pos: TVec3<f32>) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn new(pos: TVec3<f32>, color: TVec3<f32>, normal: Vec3, wave_multip: f32) -> Self {
        Self {
            pos,
            color,
            normal,
            wave_multip,
        }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vec3::default(),
            color: Vec3::new(1., 1., 0.3),
            normal: Vec3::new(1., 1., 1.),
            wave_multip: 0f32,
        }
    }
}
