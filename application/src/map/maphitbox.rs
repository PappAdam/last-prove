use nalgebra::{Vector2, Vector3, Vector4};
use objects::hitbox::ray::{IntersectableWithRay, Ray};

use super::Map;

impl IntersectableWithRay for Map {
    ///Returns the intersection point of a ray and the map.
    ///This function has special functionality for Map only, to speed up mouse-world intersection point search.
    fn intersection_point(&self, ray: &Ray) -> Option<(Vector3<f32>, f32)> {
        //Creating the two plane equations
        let grass_level_plane_equation = Vector4::new(0., -1., 0., 0.);

        //Getting the intersection points with the two planes.
        //Because of the camera tilt restrictions, the two planes cannot be paralell to the camera direction
        //  -> there will be always an intersection point, we can use unwrap_unchecked()
        let grass_level_intersection_point = unsafe {
            ray.plane_intersection_point(grass_level_plane_equation)
                .unwrap_unchecked()
        };
        let grass_level_map_coordinates = Vector2::new(
            grass_level_intersection_point.0.x as usize,
            grass_level_intersection_point.0.z as usize,
        );
        //If there is a solid tile at the intersection point at grass level, we clicked on the top of a tile, we can reutrn it instantly.
        if self.matrix[grass_level_map_coordinates.y][grass_level_map_coordinates.x].is_solid() {
            return Some((
                Vector3::new(
                    grass_level_map_coordinates.x as f32 + 0.5,
                    0.,
                    grass_level_map_coordinates.y as f32 + 0.5,
                ),
                grass_level_intersection_point.1,
            ));
        }

        let water_level_plane_equation = Vector4::new(0., -1., 0., -0.2);
        let water_level_intersection_point = unsafe {
            ray.plane_intersection_point(water_level_plane_equation)
                .unwrap_unchecked()
        };
        let water_level_map_coordinates = Vector2::new(
            water_level_intersection_point.0.x as usize,
            water_level_intersection_point.0.z as usize,
        );

        None
    }
}

impl Map {
    ///Converts a world coordinate into a tile coordinate, returns the map coordainates.
    pub fn world_coordinate_to_tile_center(&self, world_coodinate: &Vector3<f32>) -> Vector3<f32> {
        let rounded_coordinates =
            Vector2::new(world_coodinate.x as usize, world_coodinate.z as usize);
        if self.matrix[rounded_coordinates.y][rounded_coordinates.x].is_solid() {
            return Vector3::new(
                rounded_coordinates.x as f32 + 0.5,
                0.,
                rounded_coordinates.y as f32 + 0.5,
            );
        }
        // //Clicked tile is not solid

        // //Distance from the previous whole, can only be values from 0 to 1
        let x_from_whole = world_coodinate.x - rounded_coordinates.x as f32;
        let y_from_whole = world_coodinate.z - rounded_coordinates.y as f32;
        let x_offset = {
            if x_from_whole > 0.9 {
                1
            } else if x_from_whole < 0.1 {
                -1
            } else {
                0
            }
        };
        let y_offset = {
            if y_from_whole > 0.9 {
                1
            } else if y_from_whole < 0.1 {
                -1
            } else {
                0
            }
        };
        let new_x = (rounded_coordinates.x as i32 + x_offset) as usize;
        let new_y = (rounded_coordinates.y as i32 + y_offset) as usize;

        if self.matrix[new_y][new_x].is_solid() {
            return Vector3::new(new_x as f32 + 0.5, 0., new_y as f32 + 0.5);
        }
        Vector3::new(
            rounded_coordinates.x as f32 + 0.5,
            0.2,
            rounded_coordinates.y as f32 + 0.5,
        )
    }
}
