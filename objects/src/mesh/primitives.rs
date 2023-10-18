use nalgebra::Vector3;
use renderer::utils::vertex::Vertex;

use super::Mesh;

pub type Triangle = Polygon<3>;
pub type Quad = Polygon<4>;
pub type Pentagon = Polygon<5>;
pub type Hexagon = Polygon<6>;

#[derive(Debug)]
///A shape that can have CORNER_COUNT amount of corners.
///Does not store the vertex positions, just the indicies.
///Doesn't check if the points are on one plane or not *(TODO if needed later)*
pub struct Polygon<const CORNER_COUNT: usize> {
    indicies: [usize; CORNER_COUNT],
    pub normal: Vector3<f32>,
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

impl Mesh {
    ///Returns a quad with the vertices of it.
    ///Uses right-handed normal calculation, CW order of positions is advised.
    pub fn quad(
        corners: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: usize, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Quad) {
        //This is why specific (CW) order is required. Opposite order will produce a normal facing the opposite way.
        let normal = (corners[2] - corners[0])
            .cross(&(corners[1] - corners[0]))
            .normalize();

        //Creating the verticies with the specified color and the calculated normal
        let vertices = vec![
            Vertex::new(corners[0], color, normal),
            Vertex::new(corners[1], color, normal),
            Vertex::new(corners[2], color, normal),
            Vertex::new(corners[3], color, normal),
        ];

        //Creating the quad, indexing will start from start_index.
        //Normal is also stored for faster click_detection calculation.
        let quad = Quad::new_with_normal(
            [
                (start_index + 0),
                (start_index + 1),
                (start_index + 2),
                (start_index + 3),
            ],
            normal,
        );
        //Returning the created values.
        return (vertices, quad);
    }
    #[inline]
    ///Returns with 5 quads that round down the first one
    ///Uses left-handed normal calculation, CW order of positions is advised.
    pub fn rounded_quad(
        positions: [Vector3<f32>; 4],
        color: Vector3<f32>,
        start_index: usize, //We return a Vec<Vertex> instead of a [Vertex; 4], so we can append the return value to Vecs without converting types.
    ) -> (Vec<Vertex>, Vec<Quad>) {
        //Initializing all needed values.
        let top_corner_0 = positions[0];
        let top_corner_1 = positions[1];
        let top_corner_2 = positions[2];
        let top_corner_3 = positions[3];
        let bottom_corner_0 = top_corner_0 - Vector3::new(0.1, -0.2, 0.1);
        let bottom_corner_1 = top_corner_1 - Vector3::new(0.1, -0.2, -0.1);
        let bottom_corner_2 = top_corner_2 - Vector3::new(-0.1, -0.2, -0.1);
        let bottom_corner_3 = top_corner_3 - Vector3::new(-0.1, -0.2, 0.1);

        //Creating each quad (5 in total)
        //Each quad will have it's own vertices in order to have edges in render. (4 * 5 = 20 vertices in total)
        //Start index is increasing by 4 after each quad.
        let (mut top_quad_vertices, top_quad) = Mesh::quad(
            [top_corner_0, top_corner_1, top_corner_2, top_corner_3],
            color,
            start_index,
        );
        let (mut side_quad_0_vertices, side_quad_0) = Mesh::quad(
            [top_corner_0, bottom_corner_0, bottom_corner_1, top_corner_1],
            color,
            start_index + 4,
        );
        let (mut side_quad_1_vertices, side_quad_1) = Mesh::quad(
            [top_corner_1, bottom_corner_1, bottom_corner_2, top_corner_2],
            color,
            start_index + 8,
        );
        let (mut side_quad_2_vertices, side_quad_2) = Mesh::quad(
            [top_corner_2, bottom_corner_2, bottom_corner_3, top_corner_3],
            color,
            start_index + 12,
        );
        let (mut side_quad_3_vertices, side_quad_3) = Mesh::quad(
            [top_corner_3, bottom_corner_3, bottom_corner_0, top_corner_0],
            color,
            start_index + 16,
        );
        //End of quad creation

        //We chain all 20 vertices of the 5 quads together
        let mut vertices = Vec::with_capacity(20);
        vertices.append(&mut top_quad_vertices);
        vertices.append(&mut side_quad_0_vertices);
        vertices.append(&mut side_quad_1_vertices);
        vertices.append(&mut side_quad_2_vertices);
        vertices.append(&mut side_quad_3_vertices);

        //We chain all 5 quads together.
        let quads = vec![top_quad, side_quad_0, side_quad_1, side_quad_2, side_quad_3];

        //We return the chained quads.
        return (vertices, quads);
    }
}
