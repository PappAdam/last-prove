use nalgebra::{Matrix4, Vector3};

use crate::GameObject;

impl GameObject {
    pub fn get_transform(&self) -> Matrix4<f32> {
        self.transform
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.transform.column(3).xyz()
    }
    pub fn x_axis(&self) -> Vector3<f32> {
        self.transform.transform_vector(&Vector3::x_axis())
    }
    pub fn y_axis(&self) -> Vector3<f32> {
        self.transform.transform_vector(&Vector3::y_axis())
    }
    pub fn z_axis(&self) -> Vector3<f32> {
        self.transform.transform_vector(&Vector3::z_axis())
    }
}