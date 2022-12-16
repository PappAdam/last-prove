mod automata;
pub mod building;
pub mod generate;
pub mod perlin;
pub mod tile;

use std::fmt::{self, Display};
use std::vec;

use crate::engine::vector2::Vector2;
use crate::map::tile::GpuStoredTile;

use self::building::{Building, GpuStoredBuilding};
use self::tile::Tile;

pub struct Map {
    pub size: usize,
    pub height: u8,
    pub tile_matr: Vec<Vec<Option<tile::Tile>>>,
    pub building_vector: Vec<Building>,
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
            building_vector: vec![],
            num_of_vulkan_instances: 0,
            num_of_tile_columns: 0,
        }
    }

    pub fn get_tile_instance_coordinates(&self) -> Vec<GpuStoredTile> {
        let mut coordinate_vec =
            vec::from_elem(GpuStoredTile::zero(), self.num_of_vulkan_instances as usize);
        let mut vector_index = 0;
        for y in &self.tile_matr {
            for x in y {
                if let Some(tile) = x {
                    for z in tile.min_z..tile.max_z + 1 {
                        coordinate_vec[vector_index] = GpuStoredTile {
                            coordinates: [
                                tile.coordinates[0] as f32 - z as f32,
                                tile.coordinates[1] as f32 - z as f32,
                                (tile.coordinates[0] + tile.coordinates[1] + z as u16 + 1) as f32
                                    / (self.size * 2 + self.height as usize) as f32,
                            ],
                            texture_layer: tile.neighbors as u32,
                        };
                        vector_index += 1;
                    }
                }
            }
        }
        assert_eq!(coordinate_vec.len(), vector_index);
        dbg!(coordinate_vec.clone());
        coordinate_vec
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
    pub fn is_tile_at(&self, coordinates: Vector2) -> bool {
        if coordinates.x >= 0f32
            && coordinates.x < self.size as f32
            && coordinates.y >= 0f32
            && coordinates.y < self.size as f32
        {
            if let Some(_) = self.tile_matr[coordinates.y as usize][coordinates.x as usize].clone()
            {
                return true;
            }
        }
        return false;
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
