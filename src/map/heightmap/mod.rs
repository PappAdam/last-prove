mod perlin;
mod island;
use std::{
    ops::{Index, IndexMut, Mul},
    vec, iter::Enumerate,
};

pub struct HeightMap {
    size: usize,
    heightmap: Vec<Vec<f32>>,
}
impl HeightMap {
    fn new(size: usize) -> Self {
        Self {
            size,
            heightmap: vec::from_elem(vec::from_elem(f32::default(), size), size),
        }
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