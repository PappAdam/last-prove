use nalgebra::{Matrix4, OPoint, Rotation3, Scale3, Translation3, Vector3};

use crate::GameObject;

impl GameObject {
    pub fn global_to_local_coordinate(&self, global_coordinate: Vector3<f32>) -> Vector3<f32> {
        let inverse_transform = self
            .transform
            .try_inverse()
            .unwrap_or_else(Matrix4::identity);
        let local_coord = inverse_transform.transform_point(&OPoint::from(global_coordinate));
        local_coord.coords
    }
    pub fn traslate(&mut self, translation_x: f32, translation_y: f32, translation_z: f32) {
        let translation_matrix = Matrix4::from(Translation3::from(Vector3::new(
            translation_x,
            translation_y,
            translation_z,
        )));
        self.transform *= translation_matrix
    }
    pub fn scale(&mut self, scale_x: f32, scale_y: f32, scale_z: f32) {
        let scale_matrix = Matrix4::from(Scale3::from(Vector3::new(scale_x, scale_y, scale_z)));
        self.transform *= scale_matrix
    }
    pub fn rotate(&mut self, rotation_x: f32, rotation_y: f32, rotation_z: f32) {
        //Amount is in radians
        let rotation_matrix = Matrix4::from_euler_angles(rotation_x, rotation_y, rotation_z);
        self.transform = self.transform * rotation_matrix;
    }
    pub fn orbit(
        &mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        orbit_center: Vector3<f32>,
    ) {
        let relative_orbit_center = self.global_to_local_coordinate(orbit_center);
        //Translating object to 0,0,0
        self.transform *= Matrix4::from(Translation3::from(relative_orbit_center));
        //Rotating by amount
        self.rotate(rotation_x, rotation_y, rotation_z);
        //Translating object back. Because we rotated the object, tranforming backwards
        self.transform *= Matrix4::from(Translation3::from(-relative_orbit_center));
    }

    pub fn get_transform(&self) -> Matrix4<f32> {
        self.transform
    }
    pub fn get_position(&self) -> Vector3<f32> {
        self.transform.column(3).xyz()
    }
}
