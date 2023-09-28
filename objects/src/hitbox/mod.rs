use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::Vector3;

type Quad = [u16; 4];

pub struct Hitbox {
    vertices: Vec<Vector3<f32>>,
    quads: Vec<Quad>,
}

impl Hitbox {
    pub fn new(vertices: Vec<Vector3<f32>>, quads: Vec<Quad>) -> Self {
        Self { vertices, quads }
    }
    pub fn from_file(path: &str) -> Self {
        let obj_file = BufReader::new(File::open(path.to_owned() + ".obj").unwrap());

        //The buffers get filled up when reading face data
        let mut vertex_buffer = vec![];
        let mut quads = vec![];

        for line in obj_file.lines() {
            let line = line.unwrap();
            let splitted_line = line.split(' ').collect::<Vec<_>>();
            match splitted_line[0] {
                //Vertex xample: v 0.0000000 1.0000000 0.5000000
                "v" => vertex_buffer.push(Vector3::new(
                    splitted_line[1].parse::<f32>().unwrap(),
                    -splitted_line[2].parse::<f32>().unwrap(),
                    splitted_line[3].parse::<f32>().unwrap(),
                )),
                //Face example 1/1/1 2/1/1 3/1/1 4/1/1
                //Format is following: positionindex1/colorindex1/normalindex1 positionindex2/...
                //Only position is required here.
                "f" => quads.push([
                    splitted_line[0].parse().unwrap(),
                    splitted_line[1].parse().unwrap(),
                    splitted_line[2].parse().unwrap(),
                    splitted_line[3].parse().unwrap(),
                ]),
                row => {
                    panic!("Hitbox can't handle this type: {row}")
                }
            }
        }
        Hitbox::new(vertex_buffer, quads)
    }
}
