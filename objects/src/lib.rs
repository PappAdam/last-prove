use mesh::Mesh;
use nalgebra::Transform3;

pub mod mesh;

pub struct GameObject {
    transform: Transform3<f32>,
    mesh: Mesh,
}