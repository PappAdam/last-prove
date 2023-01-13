use crate::engine::{object_vector::GameObject, vector2::Vector2};

use super::{
    tile::{Tile, TileFlag},
    Map,
};

pub enum BuildingFlag {
    NotNone = 0b10000000,
    FacingTop = 0b01000000,
    FacingLeft = 0b00100000,
}

#[derive(Debug)]
pub struct Building {
    pub coordinates: [u16; 2],
    pub texture_layer: u16,
    pub flags: u8,
    //0: NOT  NONE (0 If None.)
    //1: FACING DIRECTION T-B   (1 if on top, 0 if on bottom)
    //2: FACING DIRECTION L-R   (1 if on left, 0 if on bottom)
    //3: NOT  SET
    //4: NOT  SET
    //5: NOT  SET
    //6: NOT  SET
    //7: NOT  SET
}

impl Building {
    pub fn facing(&self) -> Vector2 {
        match self.texture_layer % 4 {
            0 => {Vector2::new(0.0, -1.0)},
            1 => {Vector2::new(1.0, 0.0)},
            2 => {Vector2::new(0.0, 1.0)},
            3 => {Vector2::new(-1.0, 0.0)},
            _ => panic!("This cannot happen, just need to match because the compiler.")
        }
    }
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
    pub fn build_building(&mut self, coordinates: Vector2, building_texture_layer: u16) {
        let building = Building {
            coordinates: coordinates.into(),
            texture_layer: building_texture_layer,
            flags: BuildingFlag::NotNone as u8,
        };

        let building_index = self.building_vector.push(building);

        self.get_mut_tile_from_matr(coordinates)
            .expect("No tile found at build position")
            .set_building_on_top(building_index);
    }

    pub fn destroy_building(&mut self, tile_coordinates_below_building: Vector2) {
        let tile_below_building = self
            .get_mut_tile_from_matr(tile_coordinates_below_building)
            .unwrap();
        tile_below_building.flags &= !(TileFlag::BuildingOnTop as u8);
        let building_index = tile_below_building.object_on_top_index_in_vector as usize;
        self.building_vector.remove(building_index);
    }
}

impl Tile {
    fn set_building_on_top(&mut self, building_index: u16) {
        self.flags |= TileFlag::BuildingOnTop as u8;
        self.object_on_top_index_in_vector = building_index;
    }
}
