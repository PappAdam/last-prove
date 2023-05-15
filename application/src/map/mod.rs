use std::vec;

use nalgebra::Vector3;
use objects::mesh::Mesh;
use renderer::{utils::vertex::Vertex, Renderer};

use self::tile::Tile;

pub mod tile;

pub struct Map {
    matrix: Vec<Vec<Tile>>,
    size: usize,
}

impl Map {
    pub fn full(size: usize) -> Self {
        Self {
            matrix: vec::from_elem(vec::from_elem(Tile::new(), size), size),
            size,
        }
    }
    pub fn convert_to_mesh(&self, renderer: &Renderer) -> Mesh {
        let mut vertices = vec::from_elem(Vertex::default(), self.size * self.size * 4);
        let mut indicies = vec::from_elem(0, self.size * self.size * 6);
        for (y, row) in self.matrix.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.is_solid() {
                    let tile_index = y * self.size + x;
                    vertices[tile_index * 4] =
                        Vertex::from_pos(Vector3::new(x as f32, 0., y as f32));
                    vertices[tile_index * 4 + 1] =
                        Vertex::from_pos(Vector3::new(x as f32 + 1., 0., y as f32));
                    vertices[tile_index * 4 + 2] =
                        Vertex::from_pos(Vector3::new(x as f32, 0., y as f32 + 1.));
                    vertices[tile_index * 4 + 3] =
                        Vertex::from_pos(Vector3::new(x as f32 + 1., 0., y as f32 + 1.));

                    indicies[tile_index * 6 + 0] = (tile_index * 4 + 0) as u16;
                    indicies[tile_index * 6 + 1] = (tile_index * 4 + 1) as u16;
                    indicies[tile_index * 6 + 2] = (tile_index * 4 + 2) as u16;
                    indicies[tile_index * 6 + 3] = (tile_index * 4 + 1) as u16;
                    indicies[tile_index * 6 + 4] = (tile_index * 4 + 2) as u16;
                    indicies[tile_index * 6 + 5] = (tile_index * 4 + 3) as u16;
                }
            }
        }
        Mesh::new(renderer, vertices, indicies)
    }
}
