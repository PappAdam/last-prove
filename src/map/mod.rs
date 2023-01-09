mod automata;
pub mod building;
pub mod generate;
pub mod perlin;
pub mod tile;

use std::fmt::{self, Display};
use std::vec;

use crate::engine::object_vector::ObjVec;
use crate::engine::vector2::Vector2;
use crate::vulkanapp::gpustoredinstances::GpuStoredGameObject;

use self::building::Building;
use self::tile::Tile;

pub struct Map {
    pub size: usize,
    pub height: u8,
    pub tile_matr: Vec<Vec<Option<tile::Tile>>>,
    pub building_vector: ObjVec<Building>,
    pub num_of_vulkan_instances: u32,
    pub num_of_tile_columns: u32,
}

#[allow(unused_comparisons)]
impl Map {
    pub fn new(size: usize, height: u8) -> Self {
        Self {
            size,
            height,
            tile_matr: vec::from_elem(vec::from_elem(None, size as usize), size as usize),
            building_vector: ObjVec::with_capacity(10),
            num_of_vulkan_instances: 0,
            num_of_tile_columns: 0,
        }
    }

    pub fn get_tile_instance_coordinates(&self) -> Vec<GpuStoredGameObject> {
        let mut coordinate_vec = vec::from_elem(
            GpuStoredGameObject::zero(),
            self.num_of_vulkan_instances as usize,
        );
        let mut vector_index = 0;
        for y in &self.tile_matr {
            for x in y {
                if let Some(tile) = x {
                    for z in tile.min_z..tile.max_z + 1 {
                        coordinate_vec[vector_index] = GpuStoredGameObject {
                            coordinates: [
                                tile.coordinates[0] as f32 - z as f32,
                                tile.coordinates[1] as f32 - z as f32,
                                (tile.coordinates[0] + tile.coordinates[1] + z as u16 + 1) as f32
                                    / (self.size * 2 + self.height as usize) as f32,
                            ],
                            texture_layer: (tile.flags >> 4) as u32,
                        };
                        vector_index += 1;
                    }
                }
            }
        }
        assert_eq!(coordinate_vec.len(), vector_index);
        coordinate_vec
    }

    pub fn get_shown_tile_at_coordinates(&self, mouse_tile_coordinates: Vector2) -> Option<&Tile> {
        let rounded_mouse_coordinates = mouse_tile_coordinates.round();

        //Checking tiles in front of the click position
        let (mut final_clicked_tile, mut height_of_click) =
            self.get_tile_on_top_at_coordinates(rounded_mouse_coordinates);
        let side = (mouse_tile_coordinates.x - mouse_tile_coordinates.x.round())
        //Side is < 0 if on the left, > 0 if on the right.
            - (mouse_tile_coordinates.y - mouse_tile_coordinates.y.round());

        //Checking tiles to the sides from the click position
        let side_offset = {
            if side < 0.0 {
                Vector2::new(-1.0, 0.0)
            } else {
                Vector2::new(0.0, -1.0)
            }
        };
        let (clicked_tile_on_side, height_of_side_of_click) =
            self.get_tile_on_top_at_coordinates(rounded_mouse_coordinates + side_offset);

        if let Some(mut clicked_tile) = final_clicked_tile {
            if let Some(clicked_tile_on_side) = clicked_tile_on_side {
                height_of_click += 1;
                if height_of_side_of_click >= height_of_click {
                    clicked_tile = clicked_tile_on_side;
                }
            }
            final_clicked_tile = Some(clicked_tile)
        } else {
            final_clicked_tile = self.get_tile_from_matr(rounded_mouse_coordinates + side_offset);
            if let None = final_clicked_tile {
                final_clicked_tile =
                    self.get_tile_from_matr(rounded_mouse_coordinates + Vector2::uniform(-1.0))
            }
        }

        final_clicked_tile
    }

    fn get_tile_on_top_at_coordinates(&self, rounded_coordinates: Vector2) -> (Option<&Tile>, u8) {
        //Returns the tile that is drawn on top of the original. (The tile that is shown on the screen)
        let mut tile_at_coordinate = self.get_tile_from_matr(rounded_coordinates);
        let mut tile_at_coordinate_z = if let Some(tile_at_coordinate) = tile_at_coordinate {
            tile_at_coordinate.max_z
        } else {
            0
        };
        for z_up in 1..self.height + 1 {
            if let Some(other_tile) =
                self.get_tile_from_matr(Vector2::uniform(z_up) + rounded_coordinates)
            {
                if other_tile.max_z >= z_up {
                    tile_at_coordinate = Some(other_tile);
                    tile_at_coordinate_z = z_up;
                }
            }
        }

        (tile_at_coordinate, tile_at_coordinate_z)
    }

    pub fn get_mut_tile_from_matr(&mut self, coordinates: Vector2) -> Option<&mut Tile> {
        if coordinates.x >= 0f32
            && coordinates.x < self.size as f32
            && coordinates.y >= 0f32
            && coordinates.y < self.size as f32
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].as_mut();
        }
        None
    }
    pub fn get_tile_from_matr(&self, coordinates: Vector2) -> Option<&Tile> {
        if coordinates.x >= 0f32
            && coordinates.x < self.size as f32
            && coordinates.y >= 0f32
            && coordinates.y < self.size as f32
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].as_ref();
        }
        None
    }
    pub fn copy_tile_from_matr(&self, coordinates: Vector2) -> Option<Tile> {
        if coordinates.x >= 0f32
            && coordinates.x < self.size as f32
            && coordinates.y >= 0f32
            && coordinates.y < self.size as f32
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].clone();
        }
        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                match self.tile_matr[y][x] {
                    None => res = write!(f, "_ "),
                    Some(tile) => res = write!(f, "{} ", tile.max_z),
                }

                if let Err(_) = res {
                    return res;
                }
            }
            res = write!(f, "\n");
        }

        return res;
    }
}
