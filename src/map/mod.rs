pub mod generate;
pub mod perlin;
pub mod tile;
mod automata;

use std::fmt::{self, Display};
use std::vec;

pub struct Map {
    pub size: u16,
    pub height: u8,
    pub matr: Vec<Vec<Option<tile::Tile>>>,
    pub num_of_vulkan_instances: u32,
}
impl Map {
    pub fn new(size: u16, height:u8) -> Self {
        Self {
            size,
            height,
            matr: vec::from_elem(vec::from_elem(None, size as usize), size as usize),
            num_of_vulkan_instances: 0
        }
    }

    pub fn get_tile_coordinates(&self) -> Vec<[f32; 2]> {
        let mut coordinate_vec = vec::from_elem([0.0, 0.0], self.num_of_vulkan_instances as usize);
        let mut vector_index = 0; 
        for y in &self.matr {
            for x in y {
                if let Some(tile) = x {
                    coordinate_vec[vector_index] = [tile.coordinates[0] as f32, tile.coordinates[1] as f32];
                    vector_index += 1;
                }
            }
        }
        coordinate_vec
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                match self.matr[y][x] {
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
