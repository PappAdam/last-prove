use std::vec;

use crate::{
    engine::{object_vector::GameObject, vector2::Vector2},
    vulkanapp::gpustoredinstances::GpuStoredGameObject,
};

use super::{tile::TileFlag, Map};

pub enum BuildingFlag {
    NotNone = 0b10000000,
}

#[derive(Debug)]
pub struct Building {
    pub coordinates: [u16; 2],
    pub texture_layer: u16,
    pub flags: u8,
    //0: NOT  NONE (0 If None.)
    //1: NOT  SET
    //2: NOT  SET
    //3: NOT  SET
    //4: NOT  SET
    //5: NOT  SET
    //6: NOT  SET
    //7: NOT  SET
}

impl GameObject for Building {
    fn is_none(&self) -> bool {
        self.flags & BuildingFlag::NotNone as u8 != BuildingFlag::NotNone as u8
    }

    fn set_to_none(&mut self) {
        self.flags &= !(BuildingFlag::NotNone as u8);
    }
}

impl Map {
    pub fn get_building_instance_coordinates(&self) -> Vec<GpuStoredGameObject> {
        let mut gpu_stored_building_vector =
            vec::from_elem(GpuStoredGameObject::zero(), self.building_vector.len());
        let mut vector_index = 0;
        for building in &self.building_vector {
            if !building.is_none() {
                let z = self
                    .get_tile_from_matr(building.coordinates.into())
                    .unwrap()
                    .max_z
                    + 1;
                gpu_stored_building_vector[vector_index] = GpuStoredGameObject {
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
        }
        gpu_stored_building_vector
    }

    pub fn build_building(&mut self, coordinates: Vector2, building_texture_layer: u16) {
        let building_coordinates = coordinates;
        let building = Building {
            coordinates: building_coordinates.into(),
            texture_layer: building_texture_layer,
            flags: BuildingFlag::NotNone as u8,
        };

        let building_index = self.building_vector.push(building);

        if let Some(tile_under_building) = self.get_mut_tile_from_matr(building_coordinates) {
            if tile_under_building.flags & TileFlag::BuildingOnTop as u8 == 0 {
                //No building on top of tile
                tile_under_building.flags |= TileFlag::BuildingOnTop as u8;
                tile_under_building.building_on_top_index_in_vector = building_index;
            } else {
                panic!(
                    "Building already exists at build position: {}",
                    building_coordinates
                );
            }
        } else {
            panic!("No tile found at build position: {}", building_coordinates);
        }
    }

    pub fn destroy_building(&mut self, tile_coordinates_below_building: Vector2) {
        let tile_below_building = self
            .get_mut_tile_from_matr(tile_coordinates_below_building)
            .unwrap();
        tile_below_building.flags &= !(TileFlag::BuildingOnTop as u8);
        let building_index = tile_below_building.building_on_top_index_in_vector as usize;
        self.building_vector.remove(building_index);
    }
}
