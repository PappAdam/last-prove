use std::{
    ops::{Index, IndexMut, Mul},
    vec,
};

use noise::{NoiseFn, OpenSimplex};

pub struct HeightMap {
    size: usize,
    heightmap: Vec<Vec<f32>>,
}
impl HeightMap {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            heightmap: vec::from_elem(vec::from_elem(f32::default(), size), size),
        }
    }
    pub fn perlin_noise(size: usize, scale: f64, persistence: f64, octaves: usize) -> Self {
        let perlin = OpenSimplex::new(rand::random());
        // let perlin = OpenSimplex::new(10);

        let mut heightmap = Self::new(size);
        let mut amplitude = 1.0;

        for octave in 0..octaves {
            let frequency = 2.0_f64.powi(octave as i32);

            for y in 0..size {
                for x in 0..size {
                    let nx = (x as f64) / scale * frequency;
                    let ny = (y as f64) / scale * frequency;
                    heightmap[x][y] += (perlin.get([nx, ny]) * amplitude) as f32;
                }
            }
            amplitude *= persistence;
        }
        let center = size as f32 / 2.;
        // Normalize the heightmap
        for y in 0..size {
            for x in 0..size {
                let distance_from_center = (center - x as f32).abs().max((center - y as f32).abs());
                let relative_distance_from_center = distance_from_center / center;
                // heightmap[x][y] /= total_amplitude as f32;
                // heightmap[x][y] = (heightmap[x][y] + 1.) / 2.;
                heightmap[x][y] *= 1. - relative_distance_from_center;
                if heightmap[x][y] > 0.1 {
                    heightmap[x][y] = 1.
                } else {
                    heightmap[x][y] = 0.
                }
            }
        }

        heightmap
    }
}

impl Index<usize> for HeightMap {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.heightmap[index]
    }
}

impl IndexMut<usize> for HeightMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.heightmap[index]
    }
}

impl Mul<HeightMap> for HeightMap {
    type Output = Self;

    fn mul(self, rhs: HeightMap) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let mut result = HeightMap::new(self.size);

        for y in 0..self.size {
            for x in 0..self.size {
                result[y][x] = self[y][x] * rhs[y][x]
            }
        }

        result
    }
}
