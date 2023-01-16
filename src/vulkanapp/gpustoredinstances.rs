use std::vec;
use bytemuck::{Pod, Zeroable};
use crate::{camera::Camera, map::Map, engine::object_vector::GameObject};

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredGameObject {
    pub coordinates: [f32; 3],
    pub texture_layer: u32,
}
vulkano::impl_vertex!(GpuStoredGameObject, coordinates, texture_layer);

impl GpuStoredGameObject {
    pub fn zero() -> Self {
        Self {
            coordinates: [0.0, 0.0, 0.0],
            texture_layer: 0,
        }
    }
}
impl Map {
    pub fn get_tile_instance_coordinates(&self) -> Vec<GpuStoredGameObject> {
        let mut coordinate_vec = vec::from_elem(
            GpuStoredGameObject::zero(),
            self.num_of_vulkan_instances as usize,
        );
        let mut vector_index = 0;
        for y in &self.tile_matr {
            for x in y {
                if let Some(tile) = x {
                    for z in tile.min_z..tile.max_z + 1 {
                        coordinate_vec[vector_index] = GpuStoredGameObject {
                            coordinates: [
                                tile.coordinates[0] as f32 - z as f32,
                                tile.coordinates[1] as f32 - z as f32,
                                (tile.coordinates[0] + tile.coordinates[1] + z as u16 + 1) as f32
                                    / (self.size * 2 + self.height as usize) as f32,
                            ],
                            texture_layer: (tile.flags >> 4) as u32,
                        };
                        vector_index += 1;
                    }
                }
            }
        }
        assert_eq!(coordinate_vec.len(), vector_index);
        coordinate_vec
    }

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

    pub fn get_troop_instance_coordinates(&self) -> Vec<GpuStoredGameObject> {
        let mut gpu_stored_troop_vector =
            vec::from_elem(GpuStoredGameObject::zero(), self.troop_vector.len());
        let mut vector_index = 0;
        for troop in &self.troop_vector {
            if !troop.is_none() {
                let z = self
                    .get_tile_from_matr(troop.coordinates.into())
                    .unwrap()
                    .max_z
                    + 1;
                gpu_stored_troop_vector[vector_index] = GpuStoredGameObject {
                    coordinates: [
                        troop.coordinates[0] as f32 - z as f32,
                        troop.coordinates[1] as f32 - z as f32,
                        (troop.coordinates[0] + troop.coordinates[1] + z as u16 + 1) as f32
                            / (self.size * 2 + self.height as usize) as f32,
                    ],
                    texture_layer: 0,
                };
                vector_index += 1;
            }
        }
        gpu_stored_troop_vector
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredHUDObject {
    pub screen_position: [f32; 3], //Represents top left corner
    pub object_size: [f32; 2],     //Bottom right corner relative to top left corner
    pub texture_layer: u32,
}
vulkano::impl_vertex!(
    GpuStoredHUDObject,
    screen_position,
    object_size,
    texture_layer
);
impl GpuStoredHUDObject {
    pub fn zero() -> Self {
        Self {
            screen_position: [0.0, 0.0, 0.0],
            object_size: [0.0, 0.0],
            texture_layer: 0,
        }
    }
}
impl Camera {
    pub fn get_hud_instance_coordinates(&self) -> Vec<GpuStoredHUDObject> {
        let mut gpu_stored_hud_objects =
            vec::from_elem(GpuStoredHUDObject::zero(), self.hud_objects.len());
        for (vector_index, hud_object) in self.hud_objects.iter().enumerate() {
            if hud_object.is_shown() {
                gpu_stored_hud_objects[vector_index] = GpuStoredHUDObject {
                    screen_position: [
                        hud_object.top_left.x,
                        hud_object.top_left.y,
                        hud_object.z_layer as f32,
                    ],
                    object_size: (hud_object.bottom_right - hud_object.top_left).into(),
                    texture_layer: hud_object.texture_layer as u32,
                }
            }
        }
        gpu_stored_hud_objects
    }
}
