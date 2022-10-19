use crate::engine::vector2::Vector2;

pub enum TileType {
    debug,
    dirt,
}

pub struct Tile {
    position: Vector2,
    tile_type: TileType,
    //status: TileStatus for clicked events and stuff like that maybe
}
impl Tile {
    fn new(position: Vector2, tile_type: TileType) -> Self {
        Self { position, tile_type }
    }
}