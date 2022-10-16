use std::fmt::{self, Display};

pub mod generate;
pub mod perlin;

#[derive(Copy, Clone)]
pub struct Tile;

pub struct Map {
    size: u16,
    matr: Vec<Vec<Option<Tile>>>,
    seed: u16,
}

impl Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if let Some(_) = self.matr[y][x] {
                    res = write!(f, "X ");
                } else {
                    res = write!(f, "_ ");
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
