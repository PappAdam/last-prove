mod automata;
pub mod generate;
pub mod perlin;
pub mod objects;

use std::fmt::{self, Display};
use std::vec;

use crate::engine::vector2::{Vector2, Convert};
use objects::object_vector::ObjVec;

use self::objects::GameObject;
use self::objects::building::Building;
use self::objects::tile::Tile;
use self::objects::troop::Troop;

pub struct Map {
    pub size: usize,
    pub height: u8,
    pub tile_matr: Vec<Vec<Option<Tile>>>,
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
            tile_matr: vec::from_elem(vec::from_elem(None, size as usize), size as usize),
            building_vector: ObjVec::with_capacity(10),
            troop_vector: ObjVec::with_capacity(10),
            num_of_vulkan_instances: 0,
            num_of_tile_columns: 0,
        }
    }

    pub fn get_shown_tile_at_coordinates(&self, mouse_tile_coordinates: Vector2<f32>) -> Option<&Tile> {
        let rounded_mouse_coordinates = mouse_tile_coordinates.round().convert();

        //Checking tiles in front of the click position
        let (mut final_clicked_tile, height_of_click) =
            self.get_tile_in_front_at_coordinates(rounded_mouse_coordinates);
        dbg!(final_clicked_tile);

        let side = (mouse_tile_coordinates.x - mouse_tile_coordinates.x.round())
            - (mouse_tile_coordinates.y - mouse_tile_coordinates.y.round());
        //Side is < 0 if on the left, > 0 if on the right.
        let side_offset = {
            if side < 0.0 {
                Vector2::new(-1.0, 0.0)
            } else {
                Vector2::new(0.0, -1.0)
            }
        };
        let (clicked_tile_on_side, height_of_side_of_click) =
            self.get_tile_in_front_at_coordinates((rounded_mouse_coordinates.convert() + side_offset).convert());

        if let Some(_) = final_clicked_tile {
            if let Some(clicked_tile_on_side) = clicked_tile_on_side {
                if height_of_side_of_click >= height_of_click + 1 {
                    final_clicked_tile = Some(clicked_tile_on_side);
                }
            }
        } else {
            final_clicked_tile = self.get_tile_from_matr((rounded_mouse_coordinates.convert() + side_offset).convert());
            if let None = final_clicked_tile {
                final_clicked_tile =
                    self.get_tile_from_matr(rounded_mouse_coordinates - Vector2::uniform(1))
            }
        }
        final_clicked_tile
    }

    fn get_tile_in_front_at_coordinates(
        &self,
        rounded_coordinates: Vector2<u16>,
    ) -> (Option<&Tile>, u8) {
        //Returns the tile that is drawn on top of the original. (The tile that is shown on the screen)
        for z_up in 1..self.height + 1 {
            if let Some(other_tile) = self.get_tile_from_matr(
                rounded_coordinates + Vector2::uniform(self.height as u16) - Vector2::uniform(z_up as u16),
            ) {
                if other_tile.max_z >= self.height - z_up {
                    return (Some(other_tile), self.height - z_up);
                }
            }
        }
        (None, 0)
    }

    pub fn get_mut_tile_from_matr(&mut self, coordinates: Vector2<u16>) -> Option<&mut Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].as_mut();
        }
        None
    }
    pub fn get_tile_from_matr(&self, coordinates: Vector2<u16>) -> Option<&Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].as_ref();
        }
        None
    }
    pub fn copy_tile_from_matr(&self, coordinates: Vector2<u16>) -> Option<Tile> {
        if coordinates.x >= 0
            && coordinates.x < self.size as u16
            && coordinates.y >= 0
            && coordinates.y < self.size as u16
        {
            return self.tile_matr[coordinates.y as usize][coordinates.x as usize].clone();
        }
        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                match self.tile_matr[y][x] {
                    None => res = write!(f, "_ "),
                    Some(tile) => res = write!(f, "{} ", tile.max_z),
                }

                if let Err(_) = res {
                    return res;
                }
            }
            res = write!(f, "\n");
        }

        return res;
    }
}
