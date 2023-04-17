use mesh::{Mesh, vertex::Vertex};
use nalgebra::{Matrix4, Translation3, Vector3, Vector4};

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
    pub fn get_vertices(&self) -> Vec<Vertex> {
        let mut vertices = self.mesh.vertices.clone();
        for vertex in &mut vertices {
            let new_point = self.transform.transform_point(&vertex.pos.into());
            vertex.pos = Vector3::new(new_point.x, new_point.y, new_point.z);
            println!("{}", new_point.z);
        }
        vertices
    }
}
