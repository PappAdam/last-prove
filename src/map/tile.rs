use bytemuck::{Pod, Zeroable};
use std::fmt::{Display, Result};
use vulkano::impl_vertex;

use crate::engine::vector2::Vector2;

pub enum NeighborLocation {
    // Dir from Top -> counter clockwise
    Top = 0b1000,    //8
    Left = 0b0100,   //4
    Bottom = 0b0010, //2
    Right = 0b0001,  //1
}
pub enum TileFlag {
    NotNone = 0b10000000,
    BuildingOnTop = 0b01000000,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub struct Tile {
    pub coordinates: [u16; 2],
    texture_layer: u16,
    pub max_z: u8, //Max Z also means height.
    pub min_z: u8, //Not range, because range is not copiable
    pub neighbors: u8,
    pub flags: u8,
    //0 NOT NONE (0 If None.)
    //1 BUILDING ON TOP
    //2 NOT USED
    //3 NOT USED
    //4 NOT USED
    //5 NOT USED
    //6 NOT USED
    //7 NOT USED
    pub building_on_top_index_in_vector: u16,
    //status: TileStatus for clicked events and stuff like that maybe
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredTile {
    pub coordinates: [f32; 3],
    pub texture_layer: u32,
}
impl_vertex!(GpuStoredTile, coordinates, texture_layer);

impl GpuStoredTile {
    pub fn zero() -> Self {
        Self {
            coordinates: [0.0, 0.0, 0.0],
            texture_layer: 0,
        }
    }
}

impl Tile {
    pub fn new(coordinates: [u16; 2], max_z: u8) -> Self {
        Self {
            coordinates,
            max_z,
            min_z: 0,
            texture_layer: 0,
            flags: 0b00000000,
            neighbors: 0b0000,
            building_on_top_index_in_vector: 0,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(
            f,
            "Tile:\n\tX: {}\n\tY: {}\n\tMax Z: {}\n\tMin Z: {}",
            self.coordinates[0], self.coordinates[1], self.max_z, self.min_z
        )?;
        Ok(())
    }
}
