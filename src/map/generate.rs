use rand::Rng;
use std::vec;

use crate::engine::vector2::Vector2;


use super::perlin;
use super::{Map, tile::{Tile, TileType}};

impl Map {
    pub fn new(size: u16, seed: Option<u16>) -> Self {
        Self {
            size,
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
        let center_axis = (self.size / 2) as f32;
        let center = Vector2::new(center_axis, center_axis);
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                let treshold: f32 = Vector2::distance(center, Vector2::new(x as f32, y as f32)) / center_axis;
                let value = perlin_noise.perlin2d(x as f32, y as f32, 0.1, 2);
                if value > treshold {
                    self.matr[y][x] = Some(Tile::new(Vector2::new(x as f32, y as f32), TileType::debug , ((value - treshold) / 0.03) as u8));
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
