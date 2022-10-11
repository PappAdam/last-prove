use core::fmt;
use std::fmt::{Display, Error};

use crate::perlin;

use rand::Rng;

const SIZE_X: i32 = 100;
const SIZE_Y: i32 = 100;

#[derive(Copy, Clone)]
struct Tile;

pub struct Map {
    size_x: i32,
    size_y: i32,
    matr: [[Option<Tile>; SIZE_X as usize]; SIZE_Y as usize],
}

impl Map {
    pub fn new() -> Self {
        Self {
            size_x: SIZE_X,
            size_y: SIZE_Y,
            matr: [[None; SIZE_X as usize]; SIZE_Y as usize],
        }
    }

    pub fn with_random_seed(&mut self) -> Self {
        let rand = rand::thread_rng().gen::<u16>() as i32;

        let perlin_noise = perlin::Perlin2D::new(rand);

        for y in 0..self.size_y as usize {
            for x in 0..self.size_x as usize {
                if perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2) > 0.5 {
                    self.matr[y][x] = Some(Tile);
                }
            }
        }

        Self {
            size_x: self.size_x,
            size_y: self.size_x,
            matr: self.matr,
        }
    }

    pub fn with_given_seed(&mut self, seed: u16) -> Self {
        let perlin_noise = perlin::Perlin2D::new(seed as i32);

        for y in 0..self.size_y as usize {
            for x in 0..self.size_x as usize {
                if perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2) > 0.5 {
                    self.matr[y][x] = Some(Tile);
                }
            }
        }

        Self {
            size_x: self.size_x,
            size_y: self.size_x,
            matr: self.matr,
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res: fmt::Result = Ok(());

        for y in 0..self.size_y as usize {
            for x in 0..self.size_x as usize {
                if let Some(_) = self.matr[y][x] {
                    res = write!(f, "X ");
                } else {
                    res = write!(f, "_ ");
                }

                match res {
                    Err(_) => return res,
                    _ => (),
                }
            }
            res = write!(f, "\n");
        }

        return res;
    }
}
