use nalgebra::{Matrix4, Vector3};

pub trait Getters {
    fn get_position(&self) -> Vector3<f32>;
    fn x_axis(&self) -> Vector3<f32>;
    fn y_axis(&self) -> Vector3<f32>;
    fn z_axis(&self) -> Vector3<f32>;
}
impl Getters for Matrix4<f32> {
    fn get_position(&self) -> Vector3<f32> {
        self.column(3).xyz()
    }
    fn x_axis(&self) -> Vector3<f32> {
        self.transform_vector(&Vector3::x_axis())
    }
    fn y_axis(&self) -> Vector3<f32> {
        self.transform_vector(&Vector3::y_axis())
    }
    fn z_axis(&self) -> Vector3<f32> {
        self.transform_vector(&Vector3::z_axis())
    }
}
