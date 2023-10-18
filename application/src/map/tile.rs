use nalgebra::Vector2;

use super::Map;
use crate::MAP_SIZE;

#[derive(Clone)]
pub struct Tile {
    flags: u8,
    //0: Solid
    //1: Building on top
    //2: Troop on top
    //3: NOT USED
    //4: NOT USED
    //5: NOT USED
    //6: NOT USED
    //7: NOT USED
}

impl Tile {
    pub fn is_solid(&self) -> bool {
        self.flags & TileFlag::Solid as u8 == TileFlag::Solid as u8
    }
    pub fn flag_active(&self, flag: TileFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }
    pub fn set_flag(&mut self, flag: TileFlag) {
        self.flags |= flag as u8
    }
    pub(super) fn new() -> Self {
        Self {
            flags: TileFlag::Solid as u8,
        }
    }
    pub(super) fn none() -> Self {
        Self { flags: 0 }
    }
}

#[derive(Clone, Copy)]
pub enum TileFlag {
    Solid = 0b10000000,
    BuildingOnTop = 0b01000000,
    TroopOnTop = 0b00100000,
}

impl Map {
    #[inline]
    ///Returns the tile reference at the coordinates, if they aren't out of bounds.
    pub fn get_tile_at(&self, coordinates: &Vector2<usize>) -> Option<&Tile> {
        if coordinates.x > MAP_SIZE || coordinates.y > MAP_SIZE {
            return None;
        }
        return Some(&self.matrix[coordinates.y][coordinates.x]);
    }
    #[inline]
    ///Returns the tile reference at the coordinates.
    ///Crashes if index is out of bounds
    pub unsafe fn get_tile_at_unchecked(&self, coordinates: &Vector2<usize>) -> &Tile {
        debug_assert!(
            coordinates.x > MAP_SIZE || coordinates.y > MAP_SIZE,
            "Cannot get tile at invalid coordinates!"
        );
        return &self.matrix[coordinates.y][coordinates.x];
    }
    #[inline]
    ///Checks wether 
    ///
    pub fn is_tile_solid_at(&self, coordinates: &Vector2<usize>) -> bool {
        if coordinates.x > MAP_SIZE || coordinates.y > MAP_SIZE {
            return false;
        }
        return unsafe { self.get_tile_at_unchecked(coordinates).is_solid() };
    }
}
