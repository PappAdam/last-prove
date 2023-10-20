use nalgebra::{coordinates::X, ComplexField, Vector2, Vector3, Vector4};
use objects::{
    hitbox::ray::{IntersectableWithRay, Ray},
    mesh::Mesh,
};

use super::{tile, Map};

impl IntersectableWithRay for Map {
    ///Returns the intersection point of a ray and the map. Always returns an intersection point.
    ///This function has special functionality for Map only, to speed up mouse-world intersection point search.
    fn intersection_point(&self, ray: &Ray) -> Option<(Vector3<f32>, f32)> {
        let grass_level_plane_equation = Vector4::new(0., -1., 0., 0.);

        //Getting the intersection points with the grass plane.
        //Because of the camera tilt restrictions, the plane cannot be paralell to the camera direction
        //  -> there will be always an intersection point, we can use unwrap_unchecked()
        let grass_level_intersection_point = unsafe {
            ray.plane_intersection_point(grass_level_plane_equation)
                .unwrap_unchecked()
        };
        let grass_level_map_coordinates = Vector2::new(
            grass_level_intersection_point.0.x as usize,
            grass_level_intersection_point.0.z as usize,
        );

        //If there is a solid tile at the intersection point at grass level, we clicked on the top of a tile, we can return it instantly.
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

        //We recreate the surrounding quads to test intersection with them
        let mut vertices = vec![];
        let mut quads = vec![];
        //We check a 3x3 area with the mouse at the center
        for x in -1..2 {
            for y in -1..2 {
                //We don't have to check the one that the mouse is on, because we checked that earlier.
                if x == 0 && y == 0 {
                    continue;
                }
                //'Tile' is the current tile that we are iterating over
                //These are the rounded forms of the grass_level_map_coordinates in different types.
                let tile_coordinates_f32 = Vector3::new(
                    grass_level_map_coordinates.x as f32 + x as f32,
                    0.,
                    grass_level_map_coordinates.y as f32 + y as f32,
                );
                let tile_coordinates_usize = Vector2::new(
                    (grass_level_map_coordinates.x as i32 + x) as usize,
                    (grass_level_map_coordinates.y as i32 + y) as usize,
                );
                //If the tile isn't solid, we don't have to check for intersections
                if !self.is_tile_solid_at(&tile_coordinates_usize) {
                    continue;
                }
                //The tile is solid, we recreate it's hitbox
                let (rounded_quad_vertices, mut rounded_quad_indidicies) = Mesh::rounded_quad(
                    [
                        tile_coordinates_f32,
                        tile_coordinates_f32 + Vector3::z(),
                        tile_coordinates_f32 + Vector3::x() + Vector3::z(),
                        tile_coordinates_f32 + Vector3::x(),
                    ],
                    Vector3::zeros(),
                    vertices.len(),
                );
                //We push the recreated hitbox into our Vecs.
                vertices.append(&mut rounded_quad_vertices.iter().map(|v| v.pos).collect());
                quads.append(&mut rounded_quad_indidicies);
            }
        }
        //We check each quad for an intersection
        let mut closest_intersection_point = None;
        for (i, quad) in quads.iter().enumerate() {
            //If the quad's normal is upwards, we can skip the quad, we checked it before.
            if quad.normal == -Vector3::y() {
                continue;
            }
            //We get the intersection point.
            let possible_new_intersection_point = ray.polygon_intersection_point(&quad, &vertices);
            //If there were no intersections before, we can simply overwrite the old.
            if closest_intersection_point.is_none() {
                closest_intersection_point = possible_new_intersection_point;
                continue;
            }
            //If a closer intersection was found, we overwrite the old.
            if let Some(new_intersection_point) = possible_new_intersection_point {
                if new_intersection_point.1
                    < unsafe { closest_intersection_point.unwrap_unchecked().1 }
                {
                    closest_intersection_point = possible_new_intersection_point;
                }
            }
        }
        //End of 3x3 check

        //If an intersection was found, we return it converted to map coordinates.
        if let Some(closest_intersection_point) = closest_intersection_point {
            return Some((
                self.world_coordinate_to_tile_center(&closest_intersection_point.0),
                closest_intersection_point.1,
            ));
        }

        //If no intersection was found we return the water level intersection point converted to map coordinates.
        let water_plane = Vector4::new(0., -1., 0., -0.2);
        let water_intersection_point =
            unsafe { ray.plane_intersection_point(water_plane).unwrap_unchecked() };
        return Some((
            self.world_coordinate_to_tile_center(&water_intersection_point.0),
            water_intersection_point.1,
        ));
    }
}

impl Map {
    ///Converts a world coordinate into a tile coordinate, returns the map coordainates.
    pub fn world_coordinate_to_tile_center(&self, world_coodinate: &Vector3<f32>) -> Vector3<f32> {
        //If the coordinates are below zero, we for sure clicked water.
        //Usize conversion won't work, so we return early (usize is unsigned cannot be negative)
        if world_coodinate.x < 0. || world_coodinate.z < 0. {
            return Vector3::new(
                world_coodinate.x.floor() + 0.5,
                0.2,
                world_coodinate.z.floor() + 0.5,
            );
        }
        //These are the tile incdicies for the click position
        let rounded_coordinates =
            Vector2::new(world_coodinate.x as usize, world_coodinate.z as usize);
        //If the y is 0.2 (+-float imprecision), we clicked water, we can return immediately.
        if (world_coodinate.y - 0.2).abs() < 1e-3 {
            return Vector3::new(
                rounded_coordinates.x as f32 + 0.5,
                0.2,
                rounded_coordinates.y as f32 + 0.5,
            );
        }

        if self.is_tile_solid_at(&rounded_coordinates) {
            return Vector3::new(
                rounded_coordinates.x as f32 + 0.5,
                0.,
                rounded_coordinates.y as f32 + 0.5,
            );
        }
        //Clicked tile is not solid

        // //Distance from the previous whole, can only be values from -0.5 to 0.5
        let x_from_whole = world_coodinate.x - world_coodinate.x.round();
        let y_from_whole = world_coodinate.z - world_coodinate.z.round();

        let x_offsetted_coordinates = Vector2::new(
            (rounded_coordinates.x as i32 - x_from_whole.signum() as i32) as usize,
            rounded_coordinates.y,
        );
        let y_offsetted_coordinates = Vector2::new(
            rounded_coordinates.x,
            (rounded_coordinates.y as i32 - y_from_whole.signum() as i32) as usize,
        );

        if self.is_tile_solid_at(&x_offsetted_coordinates)
            && self.is_tile_solid_at(&y_offsetted_coordinates)
        {
            if x_from_whole.abs() < y_from_whole.abs() {
                return Vector3::new(
                    x_offsetted_coordinates.x as f32 + 0.5,
                    0.,
                    rounded_coordinates.y as f32 + 0.5,
                );
            }
            return Vector3::new(
                rounded_coordinates.x as f32 + 0.5,
                0.,
                y_offsetted_coordinates.y as f32 + 0.5,
            );
        }
        if self.is_tile_solid_at(&x_offsetted_coordinates) {
            return Vector3::new(
                x_offsetted_coordinates.x as f32 + 0.5,
                0.,
                rounded_coordinates.y as f32 + 0.5,
            );
        }
        if self.is_tile_solid_at(&y_offsetted_coordinates) {
            return Vector3::new(
                rounded_coordinates.x as f32 + 0.5,
                0.,
                y_offsetted_coordinates.y as f32 + 0.5,
            );
        }
        if x_offsetted_coordinates.x != rounded_coordinates.x
            && y_offsetted_coordinates.y != rounded_coordinates.y
        {
            return Vector3::new(
                x_offsetted_coordinates.x as f32 + 0.5,
                0.,
                y_offsetted_coordinates.y as f32 + 0.5,
            );
        }
        panic!("This case is impossible");
    }
}
