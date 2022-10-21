use rand::Rng;
use std::vec;

use super::perlin;
use super::{Map, Tile};

impl Map {
    pub fn new(size: u16, seed: Option<u16>) -> Self {
        Self {
            size: 100,
            matr: vec::from_elem(vec::from_elem(None, size as usize), size as usize),
            seed: {
                match seed {
                    None => rand::thread_rng().gen::<u16>(),
                    Some(i) => i,
                }
            },
        }
    }

    pub fn generate(mut self) -> Self {
        let perlin_noise = perlin::Perlin2D::new(self.seed as i32);

        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2) > 0.5 {
                    self.matr[y][x] = Some(Tile);
                }
            }
        }

        self
    }

    pub fn flat(mut self) -> Self {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                self.matr[y][x] = Some(Tile);
            }
        }

        self
    }
}
