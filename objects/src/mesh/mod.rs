pub mod vertex;
pub mod templates;

use std::{
    fs::File,
    io::BufReader,
    ops::{Add, AddAssign},
};

use nalgebra::Vector3;
use obj::{load_obj, Obj};

use self::vertex::Vertex;

#[derive(Default, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    indicies: Vec<u16>,
    vertices_count: u16,
    triangles_count: u16,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indicies: Vec<u16>) -> Self {
        assert_eq!(indicies.len() % 3, 0);
        Self {
            vertices_count: vertices.len() as u16,
            triangles_count: (indicies.len() / 3) as u16,
            vertices,
            indicies,
        }
    }
    pub fn from_obj(path: &str) -> Mesh {
        let input = BufReader::new(File::open(path).unwrap());
        let obj: Obj<obj::Vertex, u16> = load_obj(input).unwrap();

        let mut vertex_buffer = Vec::new();
        for vertex in obj.vertices {
            let position = vertex.position;
            let new_vertex = Vertex::new(
                position.into(),
                Vector3::new(1., 1., 1.),
                vertex.normal.into(),
            );
            vertex_buffer.push(new_vertex);
        }
        let mut index_buffer = Vec::new();
        for index in obj.indices {
            index_buffer.push(index);
        }

        Mesh::new(vertex_buffer, index_buffer)
    }
    pub fn get_indicies(&self) -> &Vec<u16> {
        return &self.indicies;
    }
    pub fn add_vertex(&mut self, mut vertex: Vec<Vertex>) {
        assert_eq!(vertex.len() % 3, 0);
        self.vertices.append(&mut vertex);
        self.vertices_count += vertex.len() as u16;
        self.triangles_count += (vertex.len() / 3) as u16;
    }
}

// impl AddAssign for Mesh {
//     fn add_assign(&mut self, mut rhs: Self) {
//         self.vertices.append(&mut rhs.vertices);
//         self.vertices_count += rhs.vertices_count;
//         self.triangles_count += rhs.triangles_count;
//     }
// }
// impl Add for Mesh {
//     type Output = Mesh;

//     fn add(mut self, rhs: Self) -> Self::Output {
//         self += rhs;
//         self
//     }
// }
