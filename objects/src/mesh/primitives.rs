use std::ops::AddAssign;

use nalgebra::Vector3;
use renderer::utils::vertex::Vertex;

use super::Mesh;

impl Mesh {
    pub fn quad(
        positions: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: u32, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Vec<u32>) {
        //Normals only work if all the vertex have the same normals, and they are in the right order
        let normal = (positions[1] - positions[0])
            .cross(&(positions[3] - positions[0]))
            .normalize();
        dbg!(normal);
        let vertices = vec![
            Vertex::new(positions[0], color, normal),
            Vertex::new(positions[1], color, normal),
            Vertex::new(positions[2], color, normal),
            Vertex::new(positions[3], color, normal),
        ];
        let indicies = vec![
            (start_index + 0) as u32,
            (start_index + 1) as u32,
            (start_index + 2) as u32,
            (start_index + 1) as u32,
            (start_index + 2) as u32,
            (start_index + 3) as u32,
        ];
        return (vertices, indicies);
    }
}
