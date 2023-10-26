use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Vector2, Vector3, Vector4};

use crate::{
    getters::Getters,
    mesh::primitives::{Quad, Triangle},
    GameObject,
};

use self::ray::{IntersectableWithRay, Ray};

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

impl IntersectableWithRay for GameObject<'_> {
    ///Checks if a given screen position collides with the object or not.
    /// Returns the global coordinate with the screen Z coordinate of the collision if yes
    fn intersection_point(&self, ray: &Ray) -> Option<(Vector3<f32>, f32)> {
        //Intead of transforming the vertices with the model transform, we only tranform the ray
        //The relative positions of the vertices and the ray will be the same this way.
        if (self.transform.get_position().x - ray.origin.x).abs() > 4.
            || (self.transform.get_position().x - ray.origin.x).abs() > 4.
        {
            return None;
        }
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
