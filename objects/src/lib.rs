use mesh::Mesh;
use nalgebra::{Matrix4, Translation3, Vector3};
use renderer::utils::vertex::Vertex;

pub mod mesh;
pub mod transformations;
pub mod getters;

pub enum GameObjectType {
    Empty,
    Camera,
    Terrain(Mesh),
    Troop(usize),    //MESH INDEX
    Building(usize), //MESH INDEX
}

pub struct GameObjectHandler {
    pub gameobjects: Vec<GameObject>,
}
impl GameObjectHandler {
    pub fn new() -> Self {
        Self {
            gameobjects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, gameobject: GameObject) {
        self.gameobjects.push(gameobject)
    }
}

pub struct GameObject {
    transform: Matrix4<f32>,
    pub ty: GameObjectType,
}

impl GameObject {
    pub fn new(position: Vector3<f32>, ty: GameObjectType) -> Self {
        Self {
            transform: Translation3::from(position).to_homogeneous(),
            ty,
        }
    }
}
