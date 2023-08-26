use std::vec;

use nalgebra::Vector3;
use objects::mesh::Mesh;
use renderer::{utils::vertex::Vertex, Renderer};

use self::{heightmap::HeightMap, tile::Tile};

pub mod heightmap;
pub mod tile;

pub struct Map {
    matrix: Vec<Vec<Tile>>,
    size: usize,
}

impl Map {
    pub fn convert_to_mesh(&self, renderer: &mut Renderer) -> Mesh {
        // let vertex_color = Vector3::new(33. / 255., 120. / 255., 0.);
        // let vertex_color = Vector3::new(255. / 255., 255. / 255., 255. / 255.);

        let mut vertices = vec![];
        let mut indicies = vec![];
        let mut tile_index = 0;
        //Iterating over rows
        for (y, _) in self.matrix.iter().enumerate() {
            //Iterating over columns, using while so I can modify x.
            let mut x = 0;
            while x < self.size {
                //If a tile is solid, we search for the next water tile in that column.
                if self.matrix[y][x].is_solid() {
                    for offset in x..self.size {
                        if self.matrix[y][offset].is_solid() {
                            //Searching for the next water tile on the column, increasing offset.
                            continue;
                        }
                        let vertex_color =
                            Vector3::new(rand::random(), rand::random(), rand::random());
                        //Found the next water tile, so make a square from the first to the last solid tile
                        let (mut square_vertices, mut square_indicies) = Mesh::quad(
                            [
                                Vector3::new(x as f32, 0., y as f32),
                                Vector3::new(offset as f32, 0., y as f32),
                                Vector3::new(x as f32, 0., y as f32 + 1.),
                                Vector3::new(offset as f32, 0., y as f32 + 1.),
                            ],
                            vertex_color,
                            tile_index * 4,
                        );
                        vertices.append(&mut square_vertices);
                        indicies.append(&mut square_indicies);
                        tile_index += 1;

                        let (mut square_vertices, mut square_indicies) = Mesh::quad(
                            [
                                Vector3::new(x as f32 - 0.1, 0.1, y as f32),
                                Vector3::new(x as f32, 0., y as f32),
                                Vector3::new(x as f32 - 0.1, 0.1, y as f32 + 1.),
                                Vector3::new(x as f32, 0., y as f32 + 1.),
                            ],
                            vertex_color,
                            tile_index * 4,
                        );
                        vertices.append(&mut square_vertices);
                        indicies.append(&mut square_indicies);
                        tile_index += 1;

                        let (mut square_vertices, mut square_indicies) = Mesh::quad(
                            [
                                Vector3::new(offset as f32, 0., y as f32),
                                Vector3::new(offset as f32 + 0.1, 0.1, y as f32),
                                Vector3::new(offset as f32, 0., y as f32 + 1.),
                                Vector3::new(offset as f32 + 0.1, 0.1, y as f32 + 1.),
                            ],
                            vertex_color,
                            tile_index * 4,
                        );
                        vertices.append(&mut square_vertices);
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
        let heightmap = HeightMap::perlin_noise(100, 30., 0.65, 4);
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
            size,
        }
    }
}
