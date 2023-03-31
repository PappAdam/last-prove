pub mod generate;
pub mod perlin;
pub mod utils;

use std::vec;

use renderer::utils::buffer_data::Vertex;

pub struct Map {
    pub size: usize,
    pub height: u8,
    pub vertecies: Vec<Vertex>,
}

#[allow(unused_comparisons)]
impl Map {
    pub fn new(size: usize, height: u8) -> Self {
        Self {
            size,
            height,
            vertecies: Vec::<Vertex>::new(),
        }
    }
}
