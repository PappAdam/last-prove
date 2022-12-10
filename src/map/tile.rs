use std::fmt::{Display, Result};
use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;

use crate::engine::vector2::Vector2;

pub enum NeighborLocation { // Dir from Top -> counter clockwise
    Top = 0b1000,       //8
    Left = 0b0100,      //4
    Bottom = 0b0010,    //2
    Right = 0b0001,     //1
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable)]
pub struct Tile {
    pub coordinates: [u16; 2],
    sampler_and_layer: u32, //First 16 bits represent sampler index, last 16 represent the texture layer
    pub max_z: u8, //Max Z also means height.
    pub min_z: u8, //Not range, because range is not copiable
    pub neighbors: u8,
    filler: u8,
    //status: TileStatus for clicked events and stuff like that maybe
}

#[repr(C)]
#[derive(Default, Clone, Copy, Pod, Zeroable, Debug)]
pub struct GpuStoredTile {
    pub coordinates: [f32; 2],
    pub sampler_and_layer: u32,
}
impl_vertex!(GpuStoredTile, coordinates, sampler_and_layer);
impl GpuStoredTile {
    pub fn zero() -> Self {
        Self { coordinates: [0.0, 0.0], sampler_and_layer: 0 }
    }
}

impl Tile {
    pub fn new(coordinates: [u16; 2], max_z: u8) -> Self {
        Self { coordinates, max_z, min_z: 0, sampler_and_layer: 0, filler: 0, neighbors: 0b0000}
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Tile:\n\tX: {}\n\tY: {}\n\tMax Z: {}\n\tMin Z: {}", self.coordinates[0], self.coordinates[1], self.max_z, self.min_z)?;
        Ok(())
    }
}