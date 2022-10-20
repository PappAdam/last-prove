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
    pub height: u8,
    //status: TileStatus for clicked events and stuff like that maybe
}
impl Tile {
    pub fn new(position: Vector2, tile_type: TileType, height: u8) -> Self {
        Self { position, tile_type, height }
    }
}