use mesh::{vertex::Vertex, Mesh};
use nalgebra::{Matrix4, Translation3, Vector3};

pub mod mesh;
pub mod transformations;

pub enum GameObjectType {
    Empty,
    Camera,
    Terrain(Mesh),
    Troop(usize),    //MESH INDEX
    Building(usize), //MESH INDEX
}

pub struct GameObject {
    transform: Matrix4<f32>,
    ty: GameObjectType,
}

impl GameObject {
    pub fn new(position: Vector3<f32>, ty: GameObjectType) -> Self {
        Self {
            transform: Translation3::from(position).to_homogeneous(),
            ty,
        }
    }

    pub fn get_mesh(&self, mesh_templates: &Vec<Mesh>) -> Mesh {
        match &self.ty {
            GameObjectType::Terrain(mesh) => mesh.clone(),
            GameObjectType::Building(index) | GameObjectType::Troop(index) => {
                mesh_templates[*index].clone()
            }
            _ => Mesh::default(),
        }
    }

    pub fn get_vertices(&self, mesh_templates: &Vec<Mesh>) -> Vec<Vertex> {
        let mut vertices = self.get_mesh(mesh_templates).vertices.clone();
        for vertex in &mut vertices {
            let new_point = self.transform.transform_point(&vertex.pos.into());
            vertex.pos = Vector3::new(new_point.x, new_point.y, new_point.z);
        }
        vertices
    }
}
