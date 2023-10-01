use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::GameObject;

type Quad = [usize; 4];

pub struct Hitbox {
    vertices: Vec<Vector3<f32>>,
    quads: Vec<Quad>,
}

impl Hitbox {
    pub fn new(vertices: Vec<Vector3<f32>>, quads: Vec<Quad>) -> Self {
        Self { vertices, quads }
    }
    pub fn from_file(path: &str) -> Self {
        let obj_file = BufReader::new(File::open(path.to_owned() + "/Hitbox.obj").unwrap());

        //The buffers get filled up when reading face data
        let mut vertex_buffer = vec![];
        let mut quads = vec![];

        for line in obj_file.lines() {
            let line = line.unwrap();
            let splitted_line = line.split(' ').collect::<Vec<_>>();
            match splitted_line[0] {
                //Vertex xample: v 0.0000000 1.0000000 0.5000000
                "v" => vertex_buffer.push(Vector3::new(
                    splitted_line[1].parse::<f32>().unwrap(),
                    -splitted_line[2].parse::<f32>().unwrap(),
                    splitted_line[3].parse::<f32>().unwrap(),
                )),
                //Face example 1/1/1 2/1/1 3/1/1 4/1/1
                //Format is following: positionindex1/colorindex1/normalindex1 positionindex2/...
                //Only position is required here.
                "f" => quads.push([
                    splitted_line[1].parse::<usize>().unwrap() - 1,
                    splitted_line[2].parse::<usize>().unwrap() - 1,
                    splitted_line[3].parse::<usize>().unwrap() - 1,
                    splitted_line[4].parse::<usize>().unwrap() - 1,
                ]),
                row => {
                    panic!("Hitbox can't handle this type: {row}")
                }
            }
        }
        Hitbox::new(vertex_buffer, quads)
    }
}

impl GameObject<'_> {
    ///Checks if a given screen position collides with the object or not.
    /// Returns the global coordinate of the collision if yes.
    pub fn check_object_clicked(&self, camera: &Matrix4<f32>, relative_mouse_position: Vector2<f32>) -> f32 {
        let mut transformed_vertices = Vec::with_capacity(self.hitbox.vertices.len());
        for vertex in &self.hitbox.vertices {
            let transformed_vertex =
                *camera * *self.transform * Vector4::new(vertex.x, vertex.y, vertex.z, 1.);
            transformed_vertices.push(transformed_vertex);
        }
        for quad in &self.hitbox.quads {
            let points = [
                transformed_vertices[quad[0]].xy(),
                transformed_vertices[quad[1]].xy(),
                transformed_vertices[quad[2]].xy(),
                transformed_vertices[quad[3]].xy(),
            ];
            let area_of_quad = area_of_quad(&points);

            // let area_of_quad_to_mose = area_of_quad_to_mouse(&points, relative_mouse_position);
            assert_eq!(area_of_quad, area_of_quad_to_mouse(&points, Vector2::zeros()))
        }
        0.
    }
}
#[inline]
fn area_of_triangle(points: [Vector2<f32>; 3]) -> f32 {
    dbg!(&points);
    //(1/2)  |x1          (    y2      −      y3    )
    0.5 * ((points[0].x * (points[1].y - points[2].y)
    //  +   x2          (    y3      −     y1     )
        + points[1].x * (points[2].y - points[0].y)
    //  +    x3         (    y1      −     y2     )|
        + points[2].x * (points[0].y - points[1].y))
        .abs())
}
#[inline]
fn area_of_quad(points: &[Vector2<f32>; 4]) -> f32 {
    area_of_triangle([points[0], points[1], points[2]])
        + area_of_triangle([points[1], points[2], points[3]])
}

#[inline]
fn area_of_quad_to_mouse(points: &[Vector2<f32>; 4], click_position: Vector2<f32>) -> f32 {
    let clicked_area_0 = area_of_triangle([click_position, points[0], points[1]]);
    dbg!(clicked_area_0);
    let clicked_area_1 = area_of_triangle([click_position, points[1], points[2]]);
    dbg!(clicked_area_1);
    let clicked_area_2 = area_of_triangle([click_position, points[2], points[3]]);
    dbg!(clicked_area_2);
    let clicked_area_3 = area_of_triangle([click_position, points[3], points[0]]);
    dbg!(clicked_area_3);
    clicked_area_0 + clicked_area_1 + clicked_area_2 + clicked_area_3
}
