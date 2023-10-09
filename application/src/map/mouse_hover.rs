use nalgebra::{Vector2, Vector3};

use super::Map;

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

        if self.matrix[new_y][new_x]
            .is_solid()
        {
            return Vector3::new(new_x as f32 + 0.5 , 0., new_y as f32 + 0.5);
        }
        Vector3::new(
            rounded_coordinates.x as f32 + 0.5,
            0.2,
            rounded_coordinates.y as f32 + 0.5,
        )
    }
}
