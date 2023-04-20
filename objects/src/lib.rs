use mesh::{vertex::Vertex, Mesh};
use nalgebra::{Matrix4, Translation3, Vector3};

pub mod mesh;

pub struct GameObject {
    transform: Matrix4<f32>,
    mesh: Mesh,
}

impl GameObject {
    pub fn new(position: Vector3<f32>, mesh: Mesh) -> Self {
        Self {
            transform: Translation3::from(position).to_homogeneous(),
            mesh,
        }
    }
    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
    pub fn scale(&mut self, amount: f32) {
        let scale_matrix = nalgebra::Matrix4::new(
            amount, 0.0, 0.0, 0.0, 0.0, amount, 0.0, 0.0, 0.0, 0.0, amount, 0.0, 0.0, 0.0, 0.0, 1.0
        );
        self.transform *= scale_matrix;
    }
    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = self.mesh.vertices.clone();
        for vertex in &mut vertices {
            let new_point = self.transform.transform_point(&vertex.pos.into());
            vertex.pos = Vector3::new(new_point.x, new_point.y, new_point.z);
        }
        vertices
    }
}
