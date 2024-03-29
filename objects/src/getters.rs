use nalgebra::{Matrix4, Vector3, Vector4};

pub trait Getters {
    fn get_position(&self) -> Vector3<f32>;
    fn x_axis(&self) -> Vector3<f32>;
    fn y_axis(&self) -> Vector3<f32>;
    fn z_axis(&self) -> Vector3<f32>;
    fn get_scale(&self) -> f32;
}
impl Getters for Matrix4<f32> {
    fn get_position(&self) -> Vector3<f32> {
        self.column(3).xyz()
    }
    fn x_axis(&self) -> Vector3<f32> {
        (self.try_inverse().unwrap() * Vector4::x()).xyz()
    }
    fn y_axis(&self) -> Vector3<f32> {
        (self.try_inverse().unwrap() * Vector4::y()).xyz()
    }
    fn z_axis(&self) -> Vector3<f32> {
        (self.try_inverse().unwrap() * Vector4::z()).xyz()
    }
    fn get_scale(&self) -> f32 {
        self.column(0).xyz().magnitude()
        //Returns the length of the X axis! (scale of X)
        //Because we use uniform scales only, other axis have the same scale.
    }
}
