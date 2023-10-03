use nalgebra::{Matrix4, OPoint, Point3, Rotation3, Translation3, Vector3};

use crate::getters::Getters;

pub trait Transformations<'a> {
    fn global_to_local_coordinate(&self, global_coordinate: Vector3<f32>) -> Vector3<f32>;
    fn translate(
        &'a mut self,
        translation_x: f32,
        translation_y: f32,
        translation_z: f32,
    ) -> &'a mut Self;
    fn translate_local(
        &'a mut self,
        translation_x: f32,
        translation_y: f32,
        translation_z: f32,
    ) -> &'a mut Self;
    fn traslate_vec3(&'a mut self, translation: Vector3<f32>) -> &'a mut Self;
    fn set_position(&'a mut self, position: Vector3<f32>) -> &'a mut Self;
    fn scale_object(&'a mut self, scale: f32) -> &'a mut Self;
    fn rotate(&'a mut self, rotation_x: f32, rotation_y: f32, rotation_z: f32) -> &'a mut Self;
    fn rotate_local(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
    ) -> &'a mut Self;
    fn orbit(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        orbit_center: Vector3<f32>,
    ) -> &'a mut Self;
    fn orbit_local(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        orbit_center: Vector3<f32>,
    ) -> &'a mut Self;
    fn look_at(&'a mut self, target: Vector3<f32>) -> &'a mut Self;
    fn set_transform(&'a mut self, matrix: &Self) -> &'a mut Self;
}
impl<'a> Transformations<'a> for Matrix4<f32> {
    fn global_to_local_coordinate(&self, global_coordinate: Vector3<f32>) -> Vector3<f32> {
        let inverse_transform = self.try_inverse().unwrap_or_else(Matrix4::identity);
        let local_coord = inverse_transform.transform_point(&OPoint::from(global_coordinate));
        local_coord.coords
    }
    fn translate(
        &'a mut self,
        translation_x: f32,
        translation_y: f32,
        translation_z: f32,
    ) -> &'a mut Self {
        let translation_matrix = Matrix4::from(Translation3::from(Vector3::new(
            translation_x,
            translation_y,
            translation_z,
        )));
        *self *= translation_matrix;
        self
    }
    fn translate_local(
        &'a mut self,
        translation_x: f32,
        translation_y: f32,
        translation_z: f32,
    ) -> &'a mut Self {
        let translation_vector = Vector3::new(translation_x, translation_y, translation_z);

        // Convert translation vector from local coordinates to global coordinates
        let global_translation = self
            .try_inverse()
            .unwrap()
            .transform_vector(&translation_vector);

        // Apply the translation in global coordinates
        let translation_matrix = Matrix4::from(Translation3::from(global_translation));
        *self *= translation_matrix;
        self
    }
    fn traslate_vec3(&'a mut self, translation: Vector3<f32>) -> &'a mut Self {
        let translation_matrix = Matrix4::from(Translation3::from(translation));
        *self *= translation_matrix;
        self
    }
    fn set_position(&'a mut self, position: Vector3<f32>) -> &'a mut Self {
        self[12] = position.x;
        self[13] = position.y;
        self[14] = position.z;
        self
    }
    fn scale_object(&'a mut self, scale: f32) -> &'a mut Self {
        //Uniform scaling only, scaling each basis vector.

        //X Axis
        self[0] *= scale;
        self[1] *= scale;
        self[2] *= scale;
        //Y Axis
        self[4] *= scale;
        self[5] *= scale;
        self[6] *= scale;
        //Z Axis
        self[8] *= scale;
        self[9] *= scale;
        self[10] *= scale;

        self
    }
    fn rotate(&'a mut self, rotation_x: f32, rotation_y: f32, rotation_z: f32) -> &'a mut Self {
        //Amount is in radians
        let rotation_matrix = Matrix4::from_euler_angles(rotation_x, rotation_y, rotation_z);
        *self = *self * rotation_matrix;
        self
    }
    fn rotate_local(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
    ) -> &'a mut Self {
        //Amount is in radians
        let rotation_matrix = Matrix4::from(Rotation3::new(Vector3::new(
            rotation_x, rotation_y, rotation_z,
        )));
        *self = rotation_matrix * *self;
        self
    }
    fn orbit(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        orbit_center: Vector3<f32>,
    ) -> &'a mut Self {
        let relative_orbit_center = self.global_to_local_coordinate(orbit_center);
        //Translating object to 0,0,0
        *self *= Matrix4::from(Translation3::from(relative_orbit_center));
        //Rotating by amount
        self.rotate(rotation_x, rotation_y, rotation_z);
        //Translating object back. Because we rotated the object, tranforming backwards
        *self *= Matrix4::from(Translation3::from(-relative_orbit_center));
        self
    }
    fn orbit_local(
        &'a mut self,
        rotation_x: f32,
        rotation_y: f32,
        rotation_z: f32,
        orbit_center: Vector3<f32>,
    ) -> &'a mut Self {
        let relative_orbit_center = self.global_to_local_coordinate(orbit_center);
        //Translating object to 0,0,0
        *self *= Matrix4::from(Translation3::from(relative_orbit_center));
        //Rotating by amount
        self.rotate_local(rotation_x, rotation_y, rotation_z);
        //Translating object back. Because we rotated the object, tranforming backwards
        *self *= Matrix4::from(Translation3::from(-relative_orbit_center));
        self
    }
    fn look_at(&'a mut self, target: Vector3<f32>) -> &'a mut Self {
        dbg!(self.get_position());
        *self = nalgebra::Matrix::look_at_rh(
            &Point3::from(self.get_position()),
            &Point3::from(target),
            &Vector3::y_axis(),
        );
        // dbg!(self.get_position());
        self
    }
    fn set_transform(&'a mut self, matrix: &Self) -> &'a mut Self {
        *self = *matrix;
        self[12] = matrix[12];
        self[13] = matrix[13];
        self[14] = matrix[14];
        self
    }
}
