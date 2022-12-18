use std::vec;

use rand::Rng;

pub fn generate(size: usize, density: f32) -> Vec<Vec<u8>> {
    let mut matr = vec::from_elem(vec::from_elem(0, size as usize), size as usize);
    
    let mut rng = rand::thread_rng();
    for y in 0..size as usize {
        for x in 0..size as usize {
            matr[y][x] = if density > rng.gen() {
                1
            }
            else {
                0
            }
        }
    }



    matr
}