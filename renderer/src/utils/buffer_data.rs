use std::ffi::c_void;

use nalgebra::{Matrix4, Vector3};

pub trait BufferObject {
    fn as_void_ptr(&self) -> *const c_void {
        self as *const _ as *const c_void
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PushConst {
    pub wh_ratio: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub ghost_value: f32,
    pub sun_direction: Vector3<f32>,
    pub ghost_value_2: f32,
    pub sun_color: Vector3<f32>,
}

#[derive(Debug)]
pub struct WorldView {
    pub view: Matrix4<f32>,
    pub rotation: Matrix4<f32>,
}

impl WorldView {
    pub fn new() -> Self {
        Self {
            view: Matrix4::identity(),
            rotation: Matrix4::identity(),
        }
    }
}

impl BufferObject for WorldView {}
