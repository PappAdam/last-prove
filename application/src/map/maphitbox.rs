use nalgebra::{coordinates::X, Vector2, Vector3, Vector4};
use objects::{
    hitbox::ray::{IntersectableWithRay, Ray},
    mesh::Mesh,
};

use super::{tile, Map};

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
        if self.is_tile_solid_at(&grass_level_map_coordinates) {
            return Some((
                Vector3::new(
                    grass_level_map_coordinates.x as f32 + 0.5,
                    0.,
                    grass_level_map_coordinates.y as f32 + 0.5,
                ),
                grass_level_intersection_point.1,
            ));
        }
        let mut vertices = vec![];
        let mut quads = vec![];
        for x in -1..2 {
            for y in -1..2 {
                // if x == 0 && y == 0 {
                //     continue;
                // }
                let tile_coordinates_f32 = Vector3::new(
                    grass_level_map_coordinates.x as f32 + x as f32,
                    0.,
                    grass_level_map_coordinates.y as f32 + y as f32,
                );
                let tile_coordinates_usize = Vector2::new(
                    (grass_level_map_coordinates.x as i32 + x) as usize,
                    (grass_level_map_coordinates.y as i32 + y) as usize,
                );
                let tile = self.get_tile_at(&tile_coordinates_usize);
                if tile.is_none() || unsafe { !tile.unwrap_unchecked().is_solid() } {
                    continue;
                }
                let mut rounded_quad = Mesh::rounded_quad(
                    [
                        tile_coordinates_f32,
                        tile_coordinates_f32 + Vector3::z(),
                        tile_coordinates_f32 + Vector3::x() + Vector3::z(),
                        tile_coordinates_f32 + Vector3::x(),
                    ],
                    Vector3::zeros(),
                    vertices.len(),
                );
                vertices.append(&mut rounded_quad.0.iter().map(|v| v.pos).collect());
                quads.append(&mut rounded_quad.1);
            }
        }
        let mut closest_intersection_point = None;
        for quad in quads {
            let possible_new_intersection_point = ray.polygon_intersection_point(&quad, &vertices);
            if closest_intersection_point.is_none() {
                closest_intersection_point = possible_new_intersection_point;
                continue;
            }
            if let Some(new_intersection_point) = possible_new_intersection_point {
                if new_intersection_point.1
                    < unsafe { closest_intersection_point.unwrap_unchecked().1 }
                {
                    closest_intersection_point = possible_new_intersection_point;
                }
            }
        }

        closest_intersection_point
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
