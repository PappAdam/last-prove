use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Vector2, Vector3, Vector4};

use crate::{
    mesh::primitives::{Quad, Triangle},
    GameObject,
};

use self::ray::Ray;

pub mod ray;

#[derive(Debug)]
pub struct Hitbox {
    pub vertices: Vec<Vector3<f32>>,
    pub quads: Vec<Quad>,
    pub triangles: Vec<Triangle>,
}

impl Hitbox {
    pub fn new(vertices: Vec<Vector3<f32>>, triangles: Vec<Triangle>, quads: Vec<Quad>) -> Self {
        Self {
            vertices,
            quads,
            triangles,
        }
    }
    pub fn from_file(path: &str) -> Self {
        let obj_file = BufReader::new(File::open(path.to_owned() + "/Hitbox.obj").unwrap());

        //The buffers get filled up when reading face data
        let mut vertices = vec![];
        let mut triangles = vec![];
        let mut quads = vec![];

        //Iterating over lines
        for line in obj_file.lines() {
            let line = line.unwrap();
            let splitted_line = line.split(' ').collect::<Vec<_>>();
            match splitted_line[0] {
                //Vertex xample: v 0.0000000 1.0000000 0.5000000
                "v" => vertices.push(Vector3::new(
                    splitted_line[1].parse::<f32>().unwrap(),
                    -splitted_line[2].parse::<f32>().unwrap(),
                    splitted_line[3].parse::<f32>().unwrap(),
                )),
                //Face example 1/1/1 2/1/1 3/1/1 4/1/1
                //Format is following: positionindex1/colorindex1/normalindex1 positionindex2/...
                //Only position is required here.
                "f" => {
                    match splitted_line.len() {
                        //Triangle
                        4 => triangles.push(Triangle::new(
                            &vertices,
                            [
                                splitted_line[1].parse::<usize>().unwrap() - 1,
                                splitted_line[2].parse::<usize>().unwrap() - 1,
                                splitted_line[3].parse::<usize>().unwrap() - 1,
                            ],
                        )),
                        //Quad
                        5 => quads.push(Quad::new(
                            &vertices,
                            [
                                splitted_line[1].parse::<usize>().unwrap() - 1,
                                splitted_line[2].parse::<usize>().unwrap() - 1,
                                splitted_line[3].parse::<usize>().unwrap() - 1,
                                splitted_line[4].parse::<usize>().unwrap() - 1,
                            ],
                        )),
                        _ => {
                            panic!("Not valid hitbox face!")
                        }
                    }
                }
                row => {
                    dbg!(
                        "Hitbox doesn't use this information, should be removed from file: "
                            .to_owned()
                            + &line
                    );
                }
            }
        }
        Hitbox::new(vertices, triangles, quads)
    }
    // pub fn into_mesh(&self, renderer: &mut Renderer, color: Vector3<f32>) -> Mesh {
    //     //Collecting vertices
    //     let mut vertex_buffer = Vec::with_capacity(self.vertices.len());
    //     for vertex in &self.vertices {
    //         vertex_buffer.push(Vertex::new(*vertex, color, Vector3::y()));
    //     }

    //     Mesh::new(
    //         renderer,
    //         vertex_buffer,
    //         self.indicies.iter().map(|v| *v as u32).collect(),
    //     )
    // }
}

impl GameObject<'_> {
    ///Checks if a given screen position collides with the object or not.
    /// Returns the global coordinate with the screen Z coordinate of the collision if yes
    pub fn ray_object_intersection_point(&self, ray: &Ray) -> Option<(Vector3<f32>, f32)> {
        //Intead of transforming the vertices with the model transform, we only tranform the ray
        //The relative positions of the vertices and the ray will be the same this way. 
        let ray = self.transform.try_inverse().unwrap() * ray;
        let untransformed_intersection_point = ray.hitbox_intersection_point(&self.mesh.hitbox);

        if let None = untransformed_intersection_point {
            return None;
        }
        let (mut intersection_point, t) =
            unsafe { untransformed_intersection_point.unwrap_unchecked() };

        intersection_point = (*self.transform
            * Vector4::new(
                intersection_point.x,
                intersection_point.y,
                intersection_point.z,
                1.,
            ))
        .xyz();
        Some((intersection_point, t))
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
