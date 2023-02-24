use bytemuck::{Pod, Zeroable};
use std::fmt::{Display, Result};

pub enum TileFlag {
    NeighborOnTop = 0b10000000,
    NeighborOnLeft = 0b01000000,
    NeighborOnBottom = 0b00100000,
    NeighborOnRight = 0b00010000,
    BuildingOnTop = 0b00001000,
    TroopOnTop = 0b00000100,
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable, PartialEq)]
pub struct Tile {
    pub coordinates: [u16; 2],
    texture_layer: u8,
    pub max_z: u8, //Max Z also means height.
    pub min_z: u8, //Not range, because range is not copiable
    pub flags: u8,
    //0 NEIGHBOR ON TOP
    //1 NEIGHBOR ON LEFT
    //2 NEIGHBOR ON BOTTOM
    //3 NEIGHBOR ON RIGHT
    //4 BUILDING ON TOP
    //5 TROOP ON TOP
    //6 NOT USED
    //7 NOT USED
    pub object_on_top_index_in_vector: u16,
}

impl Tile {
    pub fn new(coordinates: [u16; 2], max_z: u8) -> Self {
        Self {
            coordinates,
            max_z,
            min_z: 0,
            texture_layer: 0,
            flags: 0b00000000,
            object_on_top_index_in_vector: 0,
        }
    }

    pub fn is_building_on_top(&self) -> bool {
        self.flags & TileFlag::BuildingOnTop as u8 == TileFlag::BuildingOnTop as u8
    }
    pub fn is_troop_on_top(&self) -> bool {
        self.flags & TileFlag::TroopOnTop as u8 == TileFlag::TroopOnTop as u8
    }
    pub fn is_object_on_top(&self) -> bool {
        self.is_building_on_top() || self.is_troop_on_top()
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
