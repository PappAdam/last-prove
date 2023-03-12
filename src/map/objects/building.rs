use super::{
    super::Map,
    colliders::{Collider, ColliderIndex, COLLIDER_ARRAY, HasCollider},
    tile::{Tile, TileFlag},
    GameObject,
};
use crate::engine::vector2::{Convert, Vector2};

pub enum BuildingFlag {
    NotNone = 0b10000000,
}

#[derive(Debug)]
pub struct Building {
    pub coordinates: Vector2<u16>,
    pub texture_layer: u16,
    flags: u8,
    pub collider_index: ColliderIndex,
    //0: NOT  NONE (0 If None.)
    //1: NOT  SET
    //2: NOT  SET
    //3: NOT  SET
    //4: NOT  SET
    //5: NOT  SET
    //6: NOT  SET
    //7: NOT  SET
}

impl Building {
    pub fn troop_spawn_position(&self) -> Vector2<u16> {
        let offset: Vector2<i16> = match self.texture_layer % 4 {
            0 => Vector2::new(0, -1),
            1 => Vector2::new(1, 0),
            2 => Vector2::new(0, 1),
            3 => Vector2::new(-1, 0),
            _ => panic!("This cannot happen, just need to match because the compiler."),
        }
        .convert();

        (offset + self.coordinates.convert()).convert()
    }
}

impl GameObject for Building {
    fn is_none(&self) -> bool {
        self.flags & BuildingFlag::NotNone as u8 != BuildingFlag::NotNone as u8
    }

    fn set_to_none(&mut self) {
        self.flags &= !(BuildingFlag::NotNone as u8);
    }

    fn get_coordinates(&self) -> Vector2<f32> {
        self.coordinates.convert()
    }
}

impl HasCollider for Building {
    fn get_collider(&self) -> &'static Collider {
        &COLLIDER_ARRAY[self.collider_index]
    }
}

impl Map {
    pub fn build_building(&mut self, coordinates: Vector2<u16>, building_texture_layer: u16) {
        let building = Building {
            coordinates: coordinates.into(),
            texture_layer: building_texture_layer,
            flags: BuildingFlag::NotNone as u8,
            collider_index: ColliderIndex::BuildingCollider,
        };

        let building_index = self.building_vector.push(building);

        self.get_mut_tile_from_matr(coordinates)
            .expect("No tile found at build position")
            .set_building_on_top(Some(building_index));
    }

    pub fn destroy_building(&mut self, index: usize) {
        let building_coordinates = self.building_vector[index].coordinates;
        self.get_mut_tile_from_matr(building_coordinates.into())
            .expect("No tile found at build position")
            .set_building_on_top(None);
        self.building_vector.remove(index);
    }
}

impl Tile {
    fn set_building_on_top(&mut self, index: Option<u16>) {
        match index {
            Some(index) => {
                self.flags |= TileFlag::BuildingOnTop as u8;
                self.object_on_top_index_in_vector = index as u16;
            }
            None => self.flags &= !(TileFlag::BuildingOnTop as u8),
        }
    }
}
