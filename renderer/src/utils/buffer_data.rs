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
