use rand::Rng;
use std::vec;

pub fn generate(size: usize, density: f32, iterations: u8) -> Vec<Vec<u8>> {
    let mut matr = initialize_matrix(size, density);

    for _ in 0..iterations {
        matr = automata_iteration(matr);
    }

    matr
}

fn automata_iteration(old_matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let size = old_matrix.len();
    let mut new_matr = vec::from_elem(vec::from_elem(u8::MAX, size as usize), size as usize);
    for x in 0..size {
        for y in 0..size {
            let neighbor_count = get_neighbor_average_height(&old_matrix, x, y);
            if neighbor_count < 5 {
                new_matr[y][x] = 0;
            }
            else {
                new_matr[y][x] = 1;
            }
        }
    }
    new_matr
}
fn initialize_matrix(size: usize, density: f32) -> Vec<Vec<u8>> {
    let mut matr = vec::from_elem(vec::from_elem(0, size as usize), size as usize);
    let mut rng = rand::thread_rng();
    for y in 0..size as usize {
        for x in 0..size as usize {
            matr[y][x] = if density > rng.gen() { 1 } else { 0 }
        }
    }
    matr
}
fn get_neighbor_average_height(matrix: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let mut sum = 0;
    if x > 0 && y > 0 {
        sum += get_matrix_element(matrix, x - 1, y - 1);
        sum += get_matrix_element(matrix, x - 1, y);
        sum += get_matrix_element(matrix, x - 1, y + 1);
        sum += get_matrix_element(matrix, x, y - 1);
        sum += get_matrix_element(matrix, x + 1, y - 1);
    } else if x > 0 {
        sum += get_matrix_element(matrix, x - 1, y);
        sum += get_matrix_element(matrix, x - 1, y + 1);
    } else if y > 0 {
        sum += get_matrix_element(matrix, x, y - 1);
        sum += get_matrix_element(matrix, x + 1, y - 1);
    }
    sum += get_matrix_element(matrix, x + 1, y);
    sum += get_matrix_element(matrix, x, y + 1);
    sum += get_matrix_element(matrix, x + 1, y + 1);

    sum
}

#[allow(unused_comparisons)]
fn get_matrix_element(matrix: &Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
    let size = matrix.len();
    if x >= 0 && y >= 0 && x < size && y < size {
        return matrix[y][x];
    }
    return 0;
}
