use std::vec;

use nalgebra::Vector3;
use objects::mesh::Mesh;
use renderer::{utils::vertex::Vertex, Renderer};

use self::{heightmap::HeightMap, tile::Tile};

mod heightmap;
mod perlin;
pub mod tile;

pub struct Map {
    matrix: Vec<Vec<Tile>>,
    size: usize,
}

impl Map {
    pub fn convert_to_mesh(&self, renderer: &mut Renderer) -> Mesh {
        let mut vertices = vec![];
        let mut indicies = vec![];
        let mut tile_index = 0;
        //Iterating over rows
        for (y, _) in self.matrix.iter().enumerate() {
            //Iterating over columns, using while so I can modify x.
            let mut x = 0;
            while x < self.size {
                let tile = &self.matrix[y][x];
                //If a tile is solid, we search for the next water tile.
                if tile.is_solid() {
                    for offset in x..self.size {
                        if self.matrix[y][offset].is_solid() && offset != self.size - 1 {
                            continue;
                        }
                        //Here we found the next water tile, so we make a square from the first to the last solid tile
                        let mut square = vec![
                            Vertex::from_pos(Vector3::new(x as f32, 0., y as f32)),
                            Vertex::from_pos(Vector3::new(offset as f32, 0., y as f32)),
                            Vertex::from_pos(Vector3::new(x as f32, 0., y as f32 + 1.)),
                            Vertex::from_pos(Vector3::new(offset as f32, 0., y as f32 + 1.)),
                        ];
                        vertices.append(&mut square);
                        let mut square_indicies = vec![
                            (tile_index * 4 + 0) as u32,
                            (tile_index * 4 + 1) as u32,
                            (tile_index * 4 + 2) as u32,
                            (tile_index * 4 + 1) as u32,
                            (tile_index * 4 + 2) as u32,
                            (tile_index * 4 + 3) as u32,
                        ];
                        indicies.append(&mut square_indicies);

                        tile_index += 1;
                        //We can skip all previously checked tiles.
                        x = offset;
                        break;
                    }
                }
                x += 1;
            }
        }
        Mesh::new(renderer, vertices, indicies)
    }

    pub fn generate(size: usize) -> Self {
        let heightmap = HeightMap::perlin_noise(None, size);
        let mut tile_matrix = vec::from_elem(vec::from_elem(Tile::none(), size), size);
        for y in 0..size {
            for x in 0..size {
                if heightmap[y][x] > 0.5 {
                    tile_matrix[y][x] = Tile::new();
                }
            }
        }
        Self {
            matrix: tile_matrix,
            size,
        }
    }
}
