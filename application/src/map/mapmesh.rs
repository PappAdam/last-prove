use std::ops::Range;

use nalgebra::Vector3;
use objects::{mesh::Mesh, hitbox::Hitbox};
use renderer::Renderer;
use crate::MAP_SIZE;

use super::Map;

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
        vertices.append(&mut water_vertices);
        indicies.append(&mut water_quad.triangulated_indicies());
        Mesh::new(
            renderer,
            vertices,
            indicies,
            // Hitbox::new(vertex_positions, vec![], hitbox_quads),
            //We currently use a specialized function for intersections with the map, so no hitbox is needed.
            Hitbox::new(vec![], vec![], vec![]),
        )
    }
}
