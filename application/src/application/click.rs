use nalgebra::{Vector3, Vector4};
use objects::{
    getters::Getters,
    hitbox::ray::{IntersectableWithRay, Ray},
    GameObject,
};

use crate::input::EventState;

use super::App;

impl<'a> App<'a> {
    #[inline]
    ///Returns with an intersection point of the mouse with the world, if there was one.
    ///Also returns the clicked object if one was found.
    pub fn world_mouse_intersection_point(&self) -> Option<(&GameObject, Vector3<f32>)> {
        //Starting with no intersection point, we modify this later once we find closer and closer intersection points.
        let mut final_intersection_point = None;
        let mut final_intersection_distance = f32::MAX;
        //Creating a ray going from the camera into the direction of the view.
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

        let grass_level_tile = Vector4::new(0., -1., 0., 0.);
        let (grass_level_intersection, _) = unsafe {
            mouse_ray
                .plane_intersection_point(grass_level_tile)
                .unwrap_unchecked()
        };
        let mouse_ray = Ray::new(grass_level_intersection, ray_direction);

        //Iterating over each object, checking intersections with each object
        for object in &self.gameobjects {
            //If object is not clickable, we ignore it, and continue with the next object.
            if object.has_flag(objects::GameObjectFlag::NotClickable) {
                continue;
            }
            //We check if there's an intersection point with the object
            if let Some((intersection_point, intersection_distance)) =
                object.intersection_point(&mouse_ray)
            {
                //If there is an intersection point, we check for the distance of the intersection.
                if intersection_distance < final_intersection_distance {
                    //If the intersection point was closer than the previous intersection point
                    //We declare this as the new final intersection point (Min search by distance over all objects)
                    final_intersection_point = Some((object, intersection_point));
                    final_intersection_distance = intersection_distance;
                }
            }
        }

        //We check for an intersection point with the map. This is a specialized function
        //It is considerably faster on the map than the generalized algorithm.
        let (map_intersection_point, map_intersection_distance) =
            unsafe { self.map.intersection_point(&mouse_ray).unwrap_unchecked() };
        if map_intersection_distance < final_intersection_distance {
            //If the intersection point was closer than the previous intersection point
            //We declare this as the new final intersection point (Min search by distance over all objects)
            final_intersection_point = Some((&self.gameobjects[0], map_intersection_point));
        }

        //We return the possibly modified value at the end.
        final_intersection_point
    }
}
