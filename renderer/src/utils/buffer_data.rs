use std::{
    ffi::c_void,
    ops::{BitAnd, BitOr},
};

use nalgebra::{Matrix4, Vector3};

pub trait BufferObject {
    fn as_void_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
}

#[derive(Debug)]
pub struct Transform {
    pub view: Matrix4<f32>,
    pub rotation: Matrix4<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            view: Matrix4::identity(),
            rotation: Matrix4::identity(),
        }
    }
}

impl BufferObject for Transform {}

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
