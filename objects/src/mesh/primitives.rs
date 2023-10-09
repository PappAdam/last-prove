use nalgebra::Vector3;
use renderer::utils::vertex::Vertex;

use super::Mesh;

impl Mesh {
    pub fn quad(
        corners: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: u32, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Vec<u32>) {
        //Normals only work if all the vertex have the same normals, and they are in the right order
        let normal = (corners[1] - corners[0])
            .cross(&(corners[3] - corners[0]))
            .normalize();
        let vertices = vec![
            Vertex::new(corners[0], color, normal),
            Vertex::new(corners[1], color, normal),
            Vertex::new(corners[2], color, normal),
            Vertex::new(corners[3], color, normal),
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
    pub fn rounded_quad(
        positions: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: u32, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Vec<u32>) {
        let angled_normal: f32 = 0.70710678118654752440084436210485;
        let vertices = vec![
            //FLAT TOP
            Vertex::new(positions[0], color, -Vector3::y()),
            Vertex::new(positions[1], color, -Vector3::y()),
            Vertex::new(positions[2], color, -Vector3::y()),
            Vertex::new(positions[3], color, -Vector3::y()),
            //NEW SIDE
            Vertex::new(
                positions[0],
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[1],
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[0] - Vector3::new(0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[1] - Vector3::new(-0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[2],
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[3],
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[2] - Vector3::new(0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[3] - Vector3::new(-0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[1],
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[3],
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[1] - Vector3::new(-0.1, -0.2, 0.1),
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[3] - Vector3::new(-0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[0],
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[2],
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[0] - Vector3::new(0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[2] - Vector3::new(0.1, -0.2, -0.1),
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
        ];
        let indicies = vec![
            (start_index + 0) as u32,
            (start_index + 1) as u32,
            (start_index + 2) as u32,
            (start_index + 1) as u32,
            (start_index + 2) as u32,
            (start_index + 3) as u32,
            (start_index + 4 + 0) as u32,
            (start_index + 4 + 1) as u32,
            (start_index + 4 + 2) as u32,
            (start_index + 4 + 1) as u32,
            (start_index + 4 + 2) as u32,
            (start_index + 4 + 3) as u32,
            (start_index + 8 + 0) as u32,
            (start_index + 8 + 1) as u32,
            (start_index + 8 + 2) as u32,
            (start_index + 8 + 1) as u32,
            (start_index + 8 + 2) as u32,
            (start_index + 8 + 3) as u32,
            (start_index + 12 + 0) as u32,
            (start_index + 12 + 1) as u32,
            (start_index + 12 + 2) as u32,
            (start_index + 12 + 1) as u32,
            (start_index + 12 + 2) as u32,
            (start_index + 12 + 3) as u32,
            (start_index + 16 + 0) as u32,
            (start_index + 16 + 1) as u32,
            (start_index + 16 + 2) as u32,
            (start_index + 16 + 1) as u32,
            (start_index + 16 + 2) as u32,
            (start_index + 16 + 3) as u32,
        ];

        return (vertices, indicies);
    }
}
