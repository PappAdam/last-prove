use nalgebra::{Matrix4, OPoint, Point3, Rotation3, Scale3, Translation3, Vector3};

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
    fn scale(&'a mut self, scale_x: f32, scale_y: f32, scale_z: f32) -> &'a mut Self;
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
    fn scale(&'a mut self, scale_x: f32, scale_y: f32, scale_z: f32) -> &'a mut Self {
        let scale_matrix = Matrix4::from(Scale3::from(Vector3::new(scale_x, scale_y, scale_z)));
        *self *= scale_matrix;

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
}
