#[derive(Debug, Clone)]
pub struct Tile {
    pub solid: bool,
}

impl Tile {
    pub fn new(solid: bool) -> Self {
        Self { solid }
    }
}
