use crate::engine::vector2::Vector2;

pub mod building;
pub mod object_vector;
pub mod tile;
pub mod troop;

pub trait GameObject {
    fn is_none(&self) -> bool;
    fn set_to_none(&mut self);
    fn get_coordinates(&self) -> Vector2<f32>; //This type because troops can have float values.
}
