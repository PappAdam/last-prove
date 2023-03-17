use num::{pow, traits::AsPrimitive};

use crate::engine::vector2::Vector2;

use super::HeightMap;

impl HeightMap {
    pub fn island(size: usize) -> HeightMap {
        let rnd: u8 = (1.0 * rand::random::<f32>()).floor() as u8;
        match rnd {
            0 => {
                return Self::circle_grandient(size);
            }
            _ => panic!("No such island exists: {}", rnd),
        }
    }

    fn circle_grandient(size: usize) -> HeightMap {
        let mut heightmap = HeightMap::new(size);

        for y in 0..size {
            for x in 0..size {
                let i: f32 = (x as f32 / size as f32 * 2. - 1.).abs();
                let j: f32 = (y as f32 / size as f32 * 2. - 1.).abs();
                
                //USE THIS FOR RECTANGLE SHAPED GRADIENT
                // let value = {
                //     if i > j {
                //         i
                //     } else {
                //         j
                //     }
                // };

                let value = Vector2::distance(Vector2::new(i, j), Vector2::zero()) - 0.18;
                let a = 3;
                let b = 2.2;
                let gradient_value = 1. - pow(value, a) / (pow(value, a) + pow(b - b * value, a));

                heightmap[y][x] = gradient_value;
            }
        }

        heightmap
    }
}
