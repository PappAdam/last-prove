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
        self.calculate_min_z()
    }

    pub fn flat(mut self, z: u8) -> Self {
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                self.matr[y][x] = Some(Tile::new(Vector2::new(x as f32, y as f32), TileType::debug , 0, z));
            }
        }
        self
    }

    pub fn calculate_min_z(mut self) -> Self{
        for y in 0..self.size as usize {
            for x in 0..self.size as usize {
                if let Some(mut tile) = self.matr[y][x] {
                    //Calculate min_z for tiles behid this one
                    if tile.max_z > 0 {
                        let mut z_down = tile.max_z;
                        let mut z_up = 0;
    
                        while z_down > 0 {
                            z_up += 1;
                            if let Some(mut other_tile) = self.matr[y - z_up][x - z_up] {
                                if other_tile.min_z < z_down{
                                    other_tile.min_z = z_down;
                                }
                                self.matr[y - z_up][x - z_up] = Some(other_tile);
                            }
                            else {break;}
                            z_down -= 1;
                        }
                    }


                    //Calculate min_z for tiles that are blocked by neighbors
                    let left_neighbor: Option<u8>;
                    match self.matr[y + 1][x] {
                        Some(tile) => {left_neighbor = Some(tile.max_z)},
                        None => left_neighbor = None,
                    }

                    let right_neighbor: Option<u8>;
                    match self.matr[y][x + 1] {
                        Some(tile) => {right_neighbor = Some(tile.max_z)},
                        None => right_neighbor = None,
                    }
                    if let Some(left_neighbor) = left_neighbor {
                        if let Some(right_neighbor) = right_neighbor {
                            let neighbor_lowest_z = if left_neighbor < right_neighbor {left_neighbor} else {right_neighbor};
                            if neighbor_lowest_z >= tile.max_z {
                                tile.min_z = neighbor_lowest_z;
                            }
                            if tile.max_z > neighbor_lowest_z {
                                tile.min_z = neighbor_lowest_z + 1;
                            }
                        }
                    }
                    self.matr[y][x] = Some(tile);
                }
            }
        }
        self
    }
}