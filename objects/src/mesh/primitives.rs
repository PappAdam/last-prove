use nalgebra::Vector3;
use renderer::utils::vertex::Vertex;

use super::Mesh;

pub type Triangle = Polygon<3>;
pub type Quad = Polygon<4>;
pub type Pentagon = Polygon<5>;
pub type Hexagon = Polygon<6>;

#[derive(Debug)]
pub struct Polygon<const CORNER_COUNT: usize> {
    indicies: [usize; CORNER_COUNT],
    normal: Vector3<f32>,
}
impl<const CORNER_COUNT: usize> Polygon<CORNER_COUNT> {
    #[inline]
    ///Creates a polygon, with indicies.
    ///Calculates a normal **calculated by the vertex positions**.
    ///If no normal is needed, use the **new_ignore_normal()** method
    pub fn new(vertices: &Vec<Vector3<f32>>, indicies: [usize; CORNER_COUNT]) -> Self {
        let a_to_b = vertices[indicies[1]] - vertices[indicies[0]];
        let a_to_c = vertices[indicies[2]] - vertices[indicies[0]];
        let normal = a_to_c.cross(&a_to_b).normalize();
        Self { indicies, normal }
    }

    #[inline]
    ///Creates a polygon, with indicies, and a **specified normal**.
    pub fn new_with_normal(indicies: [usize; CORNER_COUNT], normal: Vector3<f32>) -> Self {
        Self { indicies, normal }
    }
    #[inline]
    ///Returns the positions of the vertices **corresponding to the polygon's indicies**.
    pub fn get_vertex_positions(&self, vertices: &Vec<Vector3<f32>>) -> Vec<Vector3<f32>> {
        let mut polygon_vertices = Vec::with_capacity(CORNER_COUNT);
        self.indicies
            .iter()
            .for_each(|i| polygon_vertices.push(vertices[*i]));
        polygon_vertices
    }

    #[inline]
    ///**ONLY CONVEX SHAPES ALLOWED** <br>
    ///Returns the indicies of the triangulated polygon.
    ///Using fan-triangulation method
    pub fn triangulated_indicies(&self) -> Vec<u32> {
        let mut triangle_indicies = vec![];
        for triangle_index in 1..CORNER_COUNT - 1 {
            triangle_indicies.push(self.indicies[0] as u32);
            triangle_indicies.push(self.indicies[triangle_index] as u32);
            triangle_indicies.push(self.indicies[triangle_index + 1] as u32);
        }
        triangle_indicies
    }
}

impl As {
    
}

//Generating primitives for map mesh.
impl Mesh {
    pub fn quad(
        corners: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: usize, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Quad) {
        //Normals only work if all the vertex have the same normals, and they are in the right order
        let normal = (corners[1] - corners[0])
            .cross(&(corners[2] - corners[0]))
            .normalize();
        let vertices = vec![
            Vertex::new(corners[0], color, normal),
            Vertex::new(corners[1], color, normal),
            Vertex::new(corners[2], color, normal),
            Vertex::new(corners[3], color, normal),
        ];
        let quad = Quad::new_with_normal(
            [
                (start_index + 0),
                (start_index + 1),
                (start_index + 2),
                (start_index + 3),
            ],
            normal,
        );
        return (vertices, quad);
    }
    pub fn rounded_quad(
        positions: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: usize, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Vec<Quad>) {
        let angled_normal: f32 = 0.70710678118654752440084436210485;
        let side1_normal = positions[0];
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
                positions[1] - Vector3::new(0.1, -0.2, -0.1),
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[0] - Vector3::new(0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::x()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[1],
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[2],
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[2] - Vector3::new(-0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            Vertex::new(
                positions[1] - Vector3::new(0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::x()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[2],
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[3],
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[3] - Vector3::new(-0.1, -0.2, 0.1),
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[2] - Vector3::new(-0.1, -0.2, -0.1),
                color,
                (-Vector3::y() - Vector3::z()) * angled_normal,
            ),
            //NEW SIDE
            Vertex::new(
                positions[3],
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[0],
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[0] - Vector3::new(0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
            Vertex::new(
                positions[3] - Vector3::new(-0.1, -0.2, 0.1),
                color,
                (-Vector3::y() + Vector3::z()) * angled_normal,
            ),
        ];
        let quads = vec![
            Quad::new_with_normal(
                [
                    (start_index + 0),
                    (start_index + 1),
                    (start_index + 2),
                    (start_index + 3),
                ],
                vertices[0].normal,
            ),
            Quad::new_with_normal(
                [
                    (start_index + 4 + 0),
                    (start_index + 4 + 1),
                    (start_index + 4 + 2),
                    (start_index + 4 + 3),
                ],
                vertices[4].normal,
            ),
            Quad::new_with_normal(
                [
                    (start_index + 8 + 0),
                    (start_index + 8 + 1),
                    (start_index + 8 + 2),
                    (start_index + 8 + 3),
                ],
                vertices[8].normal,
            ),
            Quad::new_with_normal(
                [
                    (start_index + 12 + 0),
                    (start_index + 12 + 1),
                    (start_index + 12 + 2),
                    (start_index + 12 + 3),
                ],
                vertices[12].normal,
            ),
            Quad::new_with_normal(
                [
                    (start_index + 16 + 0),
                    (start_index + 16 + 1),
                    (start_index + 16 + 2),
                    (start_index + 16 + 3),
                ],
                vertices[16].normal,
            ),
        ];

        return (vertices, quads);
    }
}
