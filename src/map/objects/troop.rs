use crate::engine::vector2::Vector2;

use super::{
    super::Map,
    tile::{Tile, TileFlag},
    GameObject,
};

struct TroopStats {
    range: u8,
    max_health: u16,
    current_health: u16,
    damage: u16,
    movement_speed: u8,
}

enum TroopFlag {
    NotNone = 0b10000000,
}

#[derive(Debug)]
pub struct Troop {
    pub coordinates: [u16; 2],
    flags: u8,
}

impl Troop {
    pub fn new(coordinates: Vector2<u16>) -> Self {
        Troop {
            coordinates: coordinates.into(),
            flags: TroopFlag::NotNone as u8,
        }
    }
}

impl GameObject for Troop {
    fn is_none(&self) -> bool {
        self.flags & TroopFlag::NotNone as u8 != TroopFlag::NotNone as u8
    }

    fn set_to_none(&mut self) {
        self.flags &= !(TroopFlag::NotNone as u8);
    }

    fn get_coordinates(&self) -> Vector2<f32> {
        self.coordinates.into()
    }
}

impl Map {
    pub fn spawn_troop(&mut self, cooridnates: Vector2<u16>) {
        let troop = Troop::new(cooridnates);
        let troop_index = self.troop_vector.push(troop);

        self.get_mut_tile_from_matr(cooridnates)
            .expect("No tile found at build position")
            .set_troop_on_top(Some(troop_index));
    }
    pub fn destroy_troop(&mut self, index: usize) {
        let troop_coordinates = self.troop_vector[index as usize].coordinates;
        self.get_mut_tile_from_matr(troop_coordinates.into())
            .expect("No tile found at build position")
            .set_troop_on_top(None);
        self.troop_vector.remove(index);
    }
}

impl Tile {
    fn set_troop_on_top(&mut self, index: Option<u16>) {
        match index {
            Some(index) => {
                self.flags |= TileFlag::TroopOnTop as u8;
                self.object_on_top_index_in_vector = index;
            }
            None => self.flags &= !(TileFlag::TroopOnTop as u8),
        }
    }
}
