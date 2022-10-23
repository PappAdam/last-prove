use std::fmt::{Display, Result};

use crate::engine::vector2::Vector2;

#[derive(Clone, Copy)]
pub enum TileType {
    debug,
    dirt,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub position: Vector2,
    pub tile_type: TileType,
    pub max_z: u8, //Max Z also means height.
    pub min_z: u8, //Not range, because range is not copiable
    //status: TileStatus for clicked events and stuff like that maybe
}
impl Tile {
    pub fn new(position: Vector2, tile_type: TileType, min_z: u8, max_z: u8) -> Self {
        Self { position, tile_type, max_z, min_z}
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        let mut res: Result = Ok(());
        write!(f, "Tile:\n\tX: {}\n\tY: {}\n\tMax Z: {}\n\tMin Z: {}", self.position.x, self.position.y, self.max_z, self.min_z);

        res
    }
}