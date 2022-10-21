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
                    self.matr[y][x] = Some(Tile::new(Vector2::new(x as f32, y as f32), TileType::debug, 0,  ((value - treshold) / 0.1) as u8));
                }
            }
        }
        //calculate_min_z(&mut self);

        self
    }

    pub fn flat(mut self, z: u8) -> Self {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                self.matr[y][x] = Some(Tile::new(Vector2::new(x as f32, y as f32), TileType::debug , 0, z));
            }
        }
        calculate_min_z(&mut self);

        self
    }
}

fn calculate_min_z(map: &mut Map) {
    for x in 0..map.size as usize {
        for y in 0..map.size as usize {
            if let Some(tile) = &map.matr[x][y] {
                if tile.max_z > 0 {
                    let mut z_down = tile.max_z;
                    let mut z_up = 0;
                    let x = tile.position.x as usize;
                    let y = tile.position.y as usize;

                    while z_down > 0 && y > z_up && x > z_up {
                        z_up += 1;
                        z_down -= 1;
                        if let Some(mut tile) = &map.matr[x - z_up][y - z_up] {
                            if tile.min_z < z_down {
                                tile.min_z = z_down;
                            }
                            map.matr[x - z_up][y - z_up] = Some(tile);
                        }
                    }
                }
            }
        }
    }
}