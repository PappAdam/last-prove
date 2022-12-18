use std::vec;

use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;

use crate::engine::vector2::Vector2;

use super::{tile::Tile, Map};
pub struct Building {
    pub coordinates: [u16; 2],
    pub texture_layer: u16,
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredBuilding {
    pub coordinates: [f32; 3],
    pub texture_layer: u32,
}
impl_vertex!(GpuStoredBuilding, coordinates, texture_layer);

impl GpuStoredBuilding {
    pub fn zero() -> Self {
        Self {
            coordinates: [0.0, 0.0, 0.0],
            texture_layer: 0,
        }
    }
}

impl Map {
    pub fn get_building_instance_coordinates(&self) -> Vec<GpuStoredBuilding> {
        let mut gpu_stored_building_vector =
            vec::from_elem(GpuStoredBuilding::zero(), self.building_vector.len());
        let mut vector_index = 0;
        for building in &self.building_vector {
            let z = self
                .get_tile_from_matr(Vector2::new(
                    building.coordinates[0],
                    building.coordinates[1],
                ))
                .unwrap()
                .max_z
                + 1;
            gpu_stored_building_vector[vector_index] = GpuStoredBuilding {
                coordinates: [
                    building.coordinates[0] as f32 - z as f32,
                    building.coordinates[1] as f32 - z as f32,
                    (building.coordinates[0] + building.coordinates[1] + z as u16 + 1) as f32
                        / (self.size * 2 + self.height as usize) as f32,
                ],
                texture_layer: 0,
            };
            vector_index += 1;
        }
        gpu_stored_building_vector
    }

    pub fn build_building(&mut self, coordinates: Vector2, building_texture_layer: u16) {
        let building_index = self.building_vector.len() as u16;
        let building = Building {
            coordinates: coordinates.into(),
            texture_layer: building_texture_layer,
        };
        if let Some(tile_under_building) = self.get_mut_tile_from_matr(building.coordinates.into())
        {
            if tile_under_building.flags & 0b10000000 == 0 {
                tile_under_building.building_on_top_index_in_vector = building_index;
                tile_under_building.flags |= 0b10000000;
                self.building_vector.push(building);
            }
        }
    }
}
