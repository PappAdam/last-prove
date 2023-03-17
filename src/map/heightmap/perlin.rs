use num::traits::AsPrimitive;

use super::HeightMap;

impl HeightMap {
    pub fn perlin_noise(seed: Option<u16>, size: usize) -> HeightMap {
        let perlin_noise = Perlin2D::new(match seed {
            None => rand::Rng::gen::<u16>(&mut rand::thread_rng()),
            Some(i) => i,
        });

        let mut heightmap = HeightMap::new(size);

        for y in 0..size {
            for x in 0..size {
                heightmap[y][x] = perlin_noise.perlin2d(x.as_(), y.as_(), 0.02, 1)
            }
        }

        heightmap
    }
}

const HASH: [u8; 256] = [
    208, 34, 231, 213, 32, 248, 233, 56, 161, 78, 24, 140, 71, 48, 140, 254, 245, 255, 247, 247,
    40, 185, 248, 251, 245, 28, 124, 204, 204, 76, 36, 1, 107, 28, 234, 163, 202, 224, 245, 128,
    167, 204, 9, 92, 217, 54, 239, 174, 173, 102, 193, 189, 190, 121, 100, 108, 167, 44, 43, 77,
    180, 204, 8, 81, 70, 223, 11, 38, 24, 254, 210, 210, 177, 32, 81, 195, 243, 125, 8, 169, 112,
    32, 97, 53, 195, 13, 203, 9, 47, 104, 125, 117, 114, 124, 165, 203, 181, 235, 193, 206, 70,
    180, 174, 0, 167, 181, 41, 164, 30, 116, 127, 198, 245, 146, 87, 224, 149, 206, 57, 4, 192,
    210, 65, 210, 129, 240, 178, 105, 228, 108, 245, 148, 140, 40, 35, 195, 38, 58, 65, 207, 215,
    253, 65, 85, 208, 76, 62, 3, 237, 55, 89, 232, 50, 217, 64, 244, 157, 199, 121, 252, 90, 17,
    212, 203, 149, 152, 140, 187, 234, 177, 73, 174, 193, 100, 192, 143, 97, 53, 145, 135, 19, 103,
    13, 90, 135, 151, 199, 91, 239, 247, 33, 39, 145, 101, 120, 99, 3, 186, 86, 99, 41, 237, 203,
    111, 79, 220, 135, 158, 42, 30, 154, 120, 67, 87, 167, 135, 176, 183, 191, 253, 115, 184, 21,
    233, 58, 129, 233, 142, 39, 128, 211, 118, 137, 139, 255, 114, 20, 218, 113, 154, 27, 127, 246,
    250, 1, 8, 198, 250, 209, 92, 222, 173, 21, 88, 102, 219,
];

pub struct Perlin2D {
    seed: i32,
}

impl Perlin2D {
    pub fn new(seed: u16) -> Self {
        Self { seed: seed as i32 }
    }

    pub fn perlin2d(&self, x: f32, y: f32, freq: f32, depth: u8) -> f32 {
        let mut xa = x * freq;
        let mut ya = y * freq;
        let amp = 1.0;
        let mut fin = 0.0;
        let mut div = 0.0;
        for _ in 0..depth {
            div += 256f32 * amp;
            fin += self.noise2d(xa, ya) * amp;
            xa *= 2f32;
            ya *= 2f32;
        }
        fin / div
    }

    fn noise2(&self, x: i32, y: i32) -> i32 {
        let tmp = HASH[((y + self.seed as i32) % 256) as usize] as i32;
        HASH[((tmp + x) % 256) as usize] as i32
    }

    fn lin_inter(&self, x: f32, y: f32, s: f32) -> f32 {
        x + s * (y - x)
    }

    fn smooth_inter(&self, x: f32, y: f32, s: f32) -> f32 {
        self.lin_inter(x, y, s * s * (3 as f32 - 2 as f32 * s))
    }

    fn noise2d(&self, x: f32, y: f32) -> f32 {
        let x_int: i32 = x as i32;
        let y_int: i32 = y as i32;
        let x_frac = x - (x_int as f32);
        let y_frac = y - (y_int as f32);
        let s = self.noise2(x_int, y_int);
        let t = self.noise2(x_int + 1, y_int);
        let u = self.noise2(x_int, y_int + 1);
        let v = self.noise2(x_int + 1, y_int + 1);
        let low = self.smooth_inter(s as f32, t as f32, x_frac);
        let high = self.smooth_inter(u as f32, v as f32, x_frac);
        self.smooth_inter(low, high, y_frac)
    }
}
