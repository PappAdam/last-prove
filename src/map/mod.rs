pub mod generate;
pub mod perlin;
pub mod tile;

use std::fmt::{self, Display};

pub struct Map<'a> {
    pub size: u16,
    pub matr: Vec<Vec<Option<tile::Tile<'a>>>>,
    seed: u16,
}

impl<'a> Display for Map<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                match self.matr[y][x] {
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
