use core::fmt;
use super::perlin;
use rand::Rng;

const SIZE_X: u16 = 100;
const SIZE_Y: u16 = 100;

#[derive(Copy, Clone)]
struct Tile;

pub struct Map {
    size_x: u16,
    size_y: u16,
    matr: [[Option<Tile>; SIZE_X as usize]; SIZE_Y as usize],
    seed: u16,
}

impl Map {
    pub fn new(seed: Option<u16>) -> Self {
        Self {
            size_x: 100,
            size_y: 100,
            matr: [[None; SIZE_X as usize]; SIZE_Y as usize],
            seed : {
                match seed {
                    None => rand::thread_rng().gen::<u16>(),
                    Some(i) => i
                }
            },
        }
    }
    
    pub fn generate(&mut self) -> Self {
        if self.seed == 0 {} //Random seed

        let perlin_noise = perlin::Perlin2D::new(self.seed as i32);

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
            seed: 16,
        }
    }
}

impl fmt::Display for Map { // Print
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

        res
    }
}
