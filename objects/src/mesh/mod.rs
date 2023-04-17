pub mod vertex;

use std::ops::{Add, AddAssign};

use self::vertex::Vertex;

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    vertices_count: u16,
    triangles_count: u16,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        assert_eq!(vertices.len() % 3, 0);
        Self {
            vertices_count: vertices.len() as u16,
            triangles_count: (vertices.len() / 3) as u16,
            vertices,
        }
    }
    pub fn add_vertex(&mut self, mut vertex: Vec<Vertex>) {
        assert_eq!(vertex.len() % 3, 0);
        self.vertices.append(&mut vertex);
        self.vertices_count += vertex.len() as u16;
        self.triangles_count += (vertex.len() / 3) as u16;
    }
}

impl AddAssign for Mesh {
    fn add_assign(&mut self, mut rhs: Self) {
        self.vertices.append(&mut rhs.vertices);
        self.vertices_count += rhs.vertices_count;
        self.triangles_count += rhs.triangles_count;
    }
}
impl Add for Mesh {
    type Output = Mesh;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
