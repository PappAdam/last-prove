use crate::engine::vector2::{Convert, Vector2};

use super::{
    super::Map,
    building::Building,
    colliders::{Collider, ColliderIndex, COLLIDER_ARRAY, HasCollider},
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
    pub coordinates: Vector2<f32>,
    flags: u8,
    pub collider_index: ColliderIndex,
}

impl Troop {
    pub fn new(coordinates: Vector2<u16>) -> Self {
        Troop {
            coordinates: coordinates.convert(),
            flags: TroopFlag::NotNone as u8,
            collider_index: ColliderIndex::TroopCollider,
        }
    }

    pub fn coordinates_inside_collider(&self, coordinates: Vector2<f32>) -> bool {
        &COLLIDER_ARRAY[self.collider_index];
        false
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

impl HasCollider for Troop {
    fn get_collider(&self) -> &'static Collider {
        &COLLIDER_ARRAY[self.collider_index]
    }
}

impl Map {
    pub fn try_troop_spawn(&mut self, building_index: usize) -> bool {
        let building = &self.building_vector[building_index];
        let troop_coordinates = building.troop_spawn_position();
        if let Some(tile_to_spawn_on) = self.get_tile_from_matr(troop_coordinates) {
            if !tile_to_spawn_on.is_object_on_top() {
                self.spawn_troop(troop_coordinates);
                return true;
            }
        }
        return false;
    }
    pub fn spawn_troop(&mut self, cooridnates: Vector2<u16>) {
        let troop = Troop::new(cooridnates);
        let troop_index = self.troop_vector.push(troop);

        self.get_mut_tile_from_matr(cooridnates)
            .expect("No tile found at build position")
            .set_troop_on_top(Some(troop_index));
    }
    pub fn destroy_troop(&mut self, index: usize) {
        let troop_coordinates = self.troop_vector[index as usize].coordinates;
        self.get_mut_tile_from_matr(troop_coordinates.round().convert())
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
