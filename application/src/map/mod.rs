use std::{ops::Range, vec};

use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh};
use renderer::{engine::object_vector::ObjVec, Renderer};

use crate::MAP_SIZE;

use self::{heightmap::HeightMap, structure::Structure, tile::Tile};

pub mod heightmap;
pub mod maphitbox;
pub mod mapmesh;
pub mod structure;
pub mod tile;

pub struct Map {
    matrix: Vec<Vec<Tile>>,
}

impl Map {

    pub fn generate(size: usize) -> Self {
        let heightmap = HeightMap::perlin_noise(size, 30., 0.65, 4);
        let mut tile_matrix = vec::from_elem(vec::from_elem(Tile::none(), size), size);
        for y in 0..size {
            for x in 0..size {
                if heightmap[y][x] > 0.7 {
                    tile_matrix[y][x] = Tile::new();
                }
            }
        }
        Self {
            matrix: tile_matrix,
        }
    }
}
