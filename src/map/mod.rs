mod automata;
mod heightmap;
pub mod generate;
pub mod objects;

use std::fmt::{self, Display};
use std::vec;

use crate::engine::vector2::{Convert, Vector2};
use objects::object_vector::ObjVec;

use self::objects::building::Building;
use self::objects::colliders::HasCollider;
use self::objects::tile::Tile;
use self::objects::troop::Troop;
use self::objects::GameObjectReference;

pub struct Map {
    pub size: usize,
    pub height: u8,
    pub sea_level: u8,
    pub tile_matr: Vec<Vec<Tile>>,
    pub building_vector: ObjVec<Building>,
    pub troop_vector: ObjVec<Troop>,
    pub num_of_vulkan_instances: u32,
    pub num_of_tile_columns: u32,
}

#[allow(unused_comparisons)]
impl Map {
    pub fn new(size: usize, height: u8) -> Self {
        Self {
            size,
            height,
            sea_level: height / 2,
            tile_matr: vec::from_elem(
                vec::from_elem(Tile::default(), size as usize),
                size as usize,
            ),
            building_vector: ObjVec::with_capacity(10),
            troop_vector: ObjVec::with_capacity(10),
            num_of_vulkan_instances: 0,
            num_of_tile_columns: 0,
        }
    }

    pub fn get_shown_object_at_coordinates(
        &self,
        mouse_coordinates: Vector2<f32>,
    ) -> GameObjectReference {
        //Checking tiles in front of the click position
        let (clicked_object_in_front, height_of_front_click) =
            self.get_object_in_front_at_coordinates(mouse_coordinates, Vector2::zero());
        let side = (mouse_coordinates.x - mouse_coordinates.x.round())
            - (mouse_coordinates.y - mouse_coordinates.y.round());
        //Side is < 0 if on the left, > 0 if on the right.
        let side_offset: Vector2<f32> = {
            if side < 0.0 {
                Vector2::new(-1., 0.)
            } else {
                Vector2::new(0., -1.)
            }
        };
        let (clicked_object_on_side, height_of_side_of_click) =
            self.get_object_in_front_at_coordinates(mouse_coordinates, side_offset);

        if height_of_front_click >= height_of_side_of_click {
            return clicked_object_in_front;
        } else {
            return clicked_object_on_side;
        }
    }

    fn get_object_in_front_at_coordinates(
        &self,
        mouse_coordinates: Vector2<f32>,
        side_offset: Vector2<f32>,
    ) -> (GameObjectReference, u8) {
        //Returns the tile that is drawn on top of the original. (The tile that is shown on the screen)

        let rounded_coordinates = (mouse_coordinates + side_offset).round().convert();
        for z_up in 1..self.height + 1 {
            if let Some(checked_tile) = self.get_tile_from_matr(
                (rounded_coordinates + Vector2::uniform(self.height as i16)
                    - Vector2::uniform(z_up as i16))
                .convert(),
            ) {
                let object_on_top_index = checked_tile.object_on_top_index_in_vector;
                if checked_tile.is_building_on_top() {
                    let building =
                        &self.building_vector[checked_tile.object_on_top_index_in_vector.into()];
                    if building.get_collider().coordinates_inside(
                        building.coordinates.convert(),
                        mouse_coordinates + Vector2::uniform(checked_tile.max_z).convert(),
                    ) {
                        return (
                            GameObjectReference::Building(
                                &self.building_vector[object_on_top_index.into()],
                            ),
                            self.height - z_up + 1,
                        );
                    }
                } else if checked_tile.is_troop_on_top() {
                    let troop =
                        &self.troop_vector[checked_tile.object_on_top_index_in_vector.into()];
                    if troop.get_collider().coordinates_inside(
                        troop.coordinates,
                        mouse_coordinates + Vector2::uniform(checked_tile.max_z).convert(),
                    ) {
                        return (
                            GameObjectReference::Troop(
                                &self.troop_vector[object_on_top_index.into()],
                            ),
                            self.height - z_up + 1,
                        );
                    }
                }
                if checked_tile.max_z >= self.height - z_up {
                    return (
                        GameObjectReference::Tile(checked_tile),
                        self.height - z_up + 1,
                    );
                }
            }
        }

        (
            self.get_tile_from_matr((rounded_coordinates - Vector2::uniform(1)).convert())
                .into(),
            0,
        )
    }

    pub fn get_mut_tile_from_matr(&mut self, coordinates: Vector2<u16>) -> Option<&mut Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return Some(&mut self.tile_matr[coordinates.y as usize][coordinates.x as usize]);
        }
        None
    }
    pub fn get_tile_from_matr(&self, coordinates: Vector2<u16>) -> Option<&Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return Some(&self.tile_matr[coordinates.y as usize][coordinates.x as usize]);
        }
        None
    }
    pub fn copy_tile_from_matr(&self, coordinates: Vector2<u16>) -> Option<Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return Some(self.tile_matr[coordinates.y as usize][coordinates.x as usize]);
        }
        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                res = write!(f, "{} ", self.tile_matr[y][x]);
                if let Err(_) = res {
                    return res;
                }
            }
            res = write!(f, "\n");
        }

        return res;
    }
}
