pub mod generate;
pub mod perlin;
pub mod tile;
mod automata;

use std::fmt::{self, Display};
use rand::Rng;
use std::vec;

pub struct Map {
    pub size: u16,
    pub height: u8,
    pub matr: Vec<Vec<Option<tile::Tile>>>,
    seed: u16,
}
impl Map {
    pub fn new(size: u16, height:u8, seed: Option<u16>) -> Self {
        Self {
            size,
            height,
            matr: vec::from_elem(vec::from_elem(None, size as usize), size as usize),
            seed: {
                match seed {
                    None => rand::thread_rng().gen::<u16>(),
                    Some(i) => i,
                }
            },
        }
    }
}

impl Display for Map {
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
