use std::fmt::{Display, Result};
use sdl2::render::Texture;
use crate::engine::vector2::Vector2;

pub enum NeighborLocation { // Dir from Top -> counter clockwise
    Top = 0b1000,
    Left = 0b0100,
    Bottom = 0b0010,
    Right = 0b0001,
}

#[derive(Clone, Copy)]
pub struct Tile<'a> {
    pub position: Vector2,
    pub tile_type: Option<&'a Texture<'a>>,
    pub max_z: u8, //Max Z also means height.
    pub min_z: u8, //Not range, because range is not copiable
    //status: TileStatus for clicked events and stuff like that maybe
}

impl<'a> Tile<'a> {
    pub fn new(position: Vector2, tile_type: Option<&'a Texture<'a>>, min_z: u8, max_z: u8) -> Self {
        Self { position, tile_type, max_z, min_z}
    }
}

impl<'a> Display for Tile<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "Tile:\n\tX: {}\n\tY: {}\n\tMax Z: {}\n\tMin Z: {}", self.position.x, self.position.y, self.max_z, self.min_z)?;

        Ok(())
    }
}