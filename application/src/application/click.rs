use nalgebra::{Vector3, Vector4};
use objects::{getters::Getters, hitbox::ray::Ray, GameObject};

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) -> Option<(&GameObject, Vector3<f32>)> {
        let mut collision_point = None;
        let closest_z = f32::MIN;
        if self
            .input
            .get_mouse_button_down(winit::event::MouseButton::Left)
        {
            let ray_origin = (self.camera.get_transform().try_inverse().unwrap()
                * Vector4::new(
                    self.input.get_relative_mouse_position().x * (1920. / 1080.),
                    self.input.get_relative_mouse_position().y,
                    0.,
                    1.,
                ))
            .xyz();
            let ray_direction = self.camera.get_transform().z_axis();
            let mouse_ray = Ray::new(ray_origin, ray_direction);
            for object in &self.gameobjects {
                if object.flag_active(objects::GameObjectFlag::NotClickable) {
                    continue;
                }
                if let Some((intersection_point, intersection_distance)) =
                    object.ray_object_intersection_point(&mouse_ray)
                {
                    if intersection_distance > closest_z {
                        collision_point = Some((object, intersection_point));
                    }
                }
            }
        }
        collision_point
    }
}
