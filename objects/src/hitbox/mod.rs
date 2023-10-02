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
    ) -> Option<Vector3<f32>> {
        let model_view_matrix = *camera * *self.transform;
        let wh_ratio = 1080. / 1920.;
        let mut transformed_vertices = Vec::with_capacity(self.hitbox.vertices.len());
        for vertex in &self.hitbox.vertices {
            let mut transformed_vertex =
                model_view_matrix * Vector4::new(vertex.x, vertex.y, vertex.z, 1.);
            transformed_vertex.x *= wh_ratio;
            transformed_vertices.push(transformed_vertex.xyz());
        }
        let mut closest_collision_point = None;
        for triangle in &self.hitbox.triangles {
            let points = [
                transformed_vertices[triangle[0]],
                transformed_vertices[triangle[1]],
                transformed_vertices[triangle[2]],
            ];
            if let Some(collision_point) = mouse_inside_triangle(points, relative_mouse_position) {
                if closest_collision_point.is_none() {
                    closest_collision_point = Some(collision_point);
                    continue;
                }
                if closest_collision_point.unwrap().z < collision_point.z {
                    closest_collision_point = Some(collision_point);
                }
            }
        }
        if let Some(mut closest_collision_point) = closest_collision_point {
            closest_collision_point.x /= wh_ratio;
            let collision_global_coordinate = model_view_matrix.try_inverse().unwrap()
                * Vector4::new(
                    closest_collision_point.x,
                    closest_collision_point.y,
                    closest_collision_point.z,
                    1.,
                );
            return Some(collision_global_coordinate.xyz());
        }
        None
    }
}
#[inline]
fn mouse_inside_triangle(
    triangle_points: [Vector3<f32>; 3],
    relative_mouse_position: Vector2<f32>,
) -> Option<Vector3<f32>> {
    //PLEASE DONT MODIFY THIS IDK WHAT IT DOES
    let v0 = triangle_points[1].xy() - triangle_points[0].xy();
    let v1 = triangle_points[2].xy() - triangle_points[0].xy();
    let v2 = relative_mouse_position - triangle_points[0].xy();
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denominator = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denominator;
    let w = (d00 * d21 - d01 * d20) / denominator;
    let u = 1. - v - w;
    if ((v + w + u) - 1.).abs() < 1e-7 && v >= 0. && w >= 0. && u >= 0. {
        let clicked_z =
            triangle_points[0].z * u + triangle_points[1].z * v + triangle_points[2].z * w;
        return Some(Vector3::new(
            relative_mouse_position.x,
            relative_mouse_position.y,
            clicked_z,
        ));
    }
    None
}
