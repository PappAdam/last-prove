use crate::engine::vector2::Vector2;

use self::{building::Building, tile::Tile, troop::Troop};

pub mod building;
pub mod object_vector;
pub mod tile;
pub mod troop;
pub mod colliders;

#[derive(Debug)]
pub enum GameObjectReference<'a> {
    None,
    Troop(&'a Troop),
    Building(&'a Building),
    Tile(&'a Tile),
}

impl<'a> Into<GameObjectReference<'a>> for Option<&'a Tile> {
    fn into(self) -> GameObjectReference<'a> {
        match self {
            Some(tile) => GameObjectReference::Tile(tile),
            None => GameObjectReference::None,
        }
    }
}

pub trait GameObject {
    fn is_none(&self) -> bool;
    fn set_to_none(&mut self);
    fn get_coordinates(&self) -> Vector2<f32>; //This type because troops can have float values.
}
