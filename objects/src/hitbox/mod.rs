use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use renderer::{utils::vertex::Vertex, Renderer};

use crate::{mesh::Mesh, GameObject};

type Triangle = [usize; 3];

pub struct Hitbox {
    vertices: Vec<Vector3<f32>>,
    triangles: Vec<Triangle>,
}

impl Hitbox {
    pub fn new(vertices: Vec<Vector3<f32>>, triangles: Vec<Triangle>) -> Self {
        Self {
            vertices,
            triangles,
        }
    }
    pub fn from_file(path: &str) -> Self {
        let obj_file = BufReader::new(File::open(path.to_owned() + "/Hitbox.obj").unwrap());

        //The buffers get filled up when reading face data
        let mut vertex_buffer = vec![];
        let mut triangles = vec![];

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
                "f" => {
                    let triangle = [
                        splitted_line[1].parse::<usize>().unwrap() - 1,
                        splitted_line[2].parse::<usize>().unwrap() - 1,
                        splitted_line[3].parse::<usize>().unwrap() - 1,
                    ];
                    triangles.push(triangle)
                }
                row => {
                    panic!("Hitbox can't handle this type: {row}")
                }
            }
        }
        Hitbox::new(vertex_buffer, triangles)
    }
    pub fn into_mesh(&self, renderer: &mut Renderer, color: Vector3<f32>) -> Mesh {
        //Collecting vertices
        let mut vertex_buffer = Vec::with_capacity(self.vertices.len());
        for vertex in &self.vertices {
            vertex_buffer.push(Vertex::new(*vertex, color, Vector3::y()));
        }
        //Collecting indicies
        let mut index_buffer = Vec::with_capacity(self.triangles.len() * 3);
        for triangle in &self.triangles {
            index_buffer.append(&mut vec![
                triangle[0] as u32,
                triangle[1] as u32,
                triangle[2] as u32,
            ])
        }

        Mesh::new(renderer, vertex_buffer, index_buffer)
    }
}

impl GameObject<'_> {
    ///Checks if a given screen position collides with the object or not.
    /// Returns the global coordinate of the collision if yes.
    pub fn check_object_clicked(
        &self,
        camera: &Matrix4<f32>,
        relative_mouse_position: Vector2<f32>,
    ) -> f32 {
        let model_view_matrix = *camera * *self.transform;
        let wh_ratio = 1080. / 1920.;
        let mut transformed_vertices = Vec::with_capacity(self.hitbox.vertices.len());
        for vertex in &self.hitbox.vertices {
            let mut transformed_vertex =
                model_view_matrix * Vector4::new(vertex.x, vertex.y, vertex.z, 1.);
            transformed_vertex.x *= wh_ratio;
            transformed_vertices.push(transformed_vertex);
        }
        let mut object_clicked = false;
        for triangle in &self.hitbox.triangles {
            let points = [
                transformed_vertices[triangle[0]].xy(),
                transformed_vertices[triangle[1]].xy(),
                transformed_vertices[triangle[2]].xy(),
            ];
            if mouse_inside_triangle(points, relative_mouse_position) {
                object_clicked = true;
                break;
            }
        }
        dbg!(object_clicked);
        0.
    }
}
#[inline]
fn mouse_inside_triangle(
    triangle_points: [Vector2<f32>; 3],
    relative_mouse_position: Vector2<f32>,
) -> bool {
    let triangle_area = area_of_triangle(&triangle_points);
    let sub_triangle1_area = area_of_triangle(&[
        triangle_points[0],
        triangle_points[1],
        relative_mouse_position,
    ]);
    let sub_triangle2_area = area_of_triangle(&[
        triangle_points[1],
        triangle_points[2],
        relative_mouse_position,
    ]);
    let sub_triangle3_area = area_of_triangle(&[
        triangle_points[2],
        triangle_points[0],
        relative_mouse_position,
    ]);
    if ((sub_triangle1_area + sub_triangle2_area + sub_triangle3_area) - triangle_area).abs() < 1e-7 {
        return true;
    }

    false
}
#[inline]
fn area_of_triangle(points: &[Vector2<f32>; 3]) -> f32 {
    //(1/2)  |x1          (    y2      −      y3    )
    0.5 * ((points[0].x * (points[1].y - points[2].y)
    //  +   x2          (    y3      −     y1     )
        + points[1].x * (points[2].y - points[0].y)
    //  +    x3         (    y1      −     y2     )|
        + points[2].x * (points[0].y - points[1].y))
        .abs())
}