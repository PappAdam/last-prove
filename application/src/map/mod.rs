use std::{ops::Range, vec};

use nalgebra::Vector3;
use objects::{hitbox::Hitbox, mesh::Mesh};
use renderer::{engine::object_vector::ObjVec, Renderer};

use crate::MAP_SIZE;

use self::{heightmap::HeightMap, structure::Structure, tile::Tile};

pub mod heightmap;
pub mod mouse_hover;
pub mod structure;
pub mod tile;

pub struct Map {
    matrix: Vec<Vec<Tile>>,
}

impl Map {
    pub fn convert_to_mesh(&self, renderer: &mut Renderer) -> Mesh {
        let grass_color = Vector3::new(148. / 255., 186. / 255., 101. / 255.);
        let mut tile_quads: Vec<Vec<Range<usize>>> = vec![];

        let mut vertices = vec![];
        let mut indicies = vec![];
        let mut hitbox_quads = vec![];
        let mut tile_index = 0;
        //Iterating over rows
        for (y, _) in self.matrix.iter().enumerate() {
            //Iterating over columns, using while so I can modify x.
            tile_quads.push(vec![]);
            let mut x = 0;
            while x < MAP_SIZE {
                //If a tile is solid, we search for the next water tile in that column.
                if self.matrix[y][x].is_solid() {
                    for offset in x..MAP_SIZE {
                        if self.matrix[y][offset].is_solid() {
                            //Searching for the next water tile on the column, increasing offset.
                            continue;
                        }
                        tile_quads[y].push(x..offset);
                        x = offset;
                        break;
                    }
                }
                x += 1;
            }
        }

        //We can skip all previously checked tiles.
        let mut y = 0;
        while y < MAP_SIZE {
            let row = tile_quads[y].clone();
            for section in row {
                let mut y_offset = 1;
                let mut index = 0;
                while tile_quads[y + y_offset]
                    .iter()
                    .enumerate()
                    .find(|(i, foundsection)| {
                        index = *i;
                        &&section == foundsection
                    })
                    .is_some()
                {
                    tile_quads[y + y_offset].remove(index);
                    y_offset += 1;
                }
                let (mut square_vertices, mut tile_rounded_quad) = Mesh::rounded_quad(
                    [
                        Vector3::new(section.start as f32, 0., y as f32),
                        Vector3::new(section.start as f32, 0., y as f32 + y_offset as f32),
                        Vector3::new(section.end as f32, 0., y as f32 + y_offset as f32),
                        Vector3::new(section.end as f32, 0., y as f32),
                    ],
                    grass_color,
                    tile_index * 20,
                );
                vertices.append(&mut square_vertices);
                indicies.append(&mut tile_rounded_quad[0].triangulated_indicies());
                indicies.append(&mut tile_rounded_quad[1].triangulated_indicies());
                indicies.append(&mut tile_rounded_quad[2].triangulated_indicies());
                indicies.append(&mut tile_rounded_quad[3].triangulated_indicies());
                indicies.append(&mut tile_rounded_quad[4].triangulated_indicies());
                hitbox_quads.append(&mut tile_rounded_quad);
                tile_index += 1;
            }
            y += 1;
        }
        let (mut water_vertices, water_quad) = Mesh::quad(
            [
                Vector3::new(0., 0.2, 0.),
                Vector3::new(0., 0.2, MAP_SIZE as f32),
                Vector3::new(MAP_SIZE as f32, 0.2, MAP_SIZE as f32),
                Vector3::new(MAP_SIZE as f32, 0.2, 0.),
            ],
            Vector3::new(39. / 255., 144. / 255., 176. / 255.),
            tile_index * 20,
        );
        dbg!(&water_vertices);
        dbg!(&water_quad.triangulated_indicies());
        vertices.append(&mut water_vertices);
        indicies.append(&mut water_quad.triangulated_indicies());
        hitbox_quads.push(water_quad);
        let vertex_positions: Vec<Vector3<f32>> = vertices.iter().map(|v| v.pos).collect();
        Mesh::new(
            renderer,
            vertices,
            indicies,
            Hitbox::new(vertex_positions, vec![], hitbox_quads),
        )
    }

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
