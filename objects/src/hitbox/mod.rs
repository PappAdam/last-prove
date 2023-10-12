use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use nalgebra::{Matrix4, Vector2, Vector3, Vector4};

use crate::{getters::Getters, GameObject};

use self::ray::Ray;

pub mod ray;

#[derive(Debug)]
pub struct Triangle {
    indicies: [usize; 3],
    normal: Vector3<f32>,
}
impl Triangle {
    #[inline]
    pub fn new(vertices: &Vec<Vector3<f32>>, indicies: [usize; 3]) -> Self {
        let a_to_b = vertices[indicies[1]] - vertices[indicies[0]];
        let a_to_c = vertices[indicies[2]] - vertices[indicies[0]];
        let normal = a_to_c.cross(&a_to_b).normalize();
        Self { indicies, normal }
    }
    #[inline]
    pub fn get_vertices(&self, vertices: &Vec<Vector3<f32>>) -> [Vector3<f32>; 3] {
        [
            vertices[self.indicies[0]],
            vertices[self.indicies[1]],
            vertices[self.indicies[2]],
        ]
    }
}

#[derive(Debug)]
pub struct Hitbox {
    pub vertices: Vec<Vector3<f32>>,
    pub triangles: Vec<Triangle>,
}

impl Hitbox {
    pub fn new(vertices: Vec<Vector3<f32>>, indicies: Vec<usize>) -> Self {
        let mut triangles = Vec::with_capacity(indicies.len() / 3);
        for triangle_index in 0..indicies.len() / 3 {
            triangles.push(Triangle::new(
                &vertices,
                [
                    indicies[triangle_index * 3 + 0],
                    indicies[triangle_index * 3 + 1],
                    indicies[triangle_index * 3 + 2],
                ],
            ));
        }
        Self {
            vertices,
            triangles,
        }
    }
    pub fn from_file(path: &str) -> Self {
        let obj_file = BufReader::new(File::open(path.to_owned() + "/Hitbox.obj").unwrap());

        //The buffers get filled up when reading face data
        let mut vertices = vec![];
        let mut indicies = vec![];

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
                    indicies.push(splitted_line[1].parse::<usize>().unwrap() - 1);
                    indicies.push(splitted_line[2].parse::<usize>().unwrap() - 1);
                    indicies.push(splitted_line[3].parse::<usize>().unwrap() - 1);
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
        Hitbox::new(vertices, indicies)
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
    pub fn check_object_clicked(
        &self,
        camera: &Matrix4<f32>,
        mut relative_mouse_position: Vector2<f32>,
    ) -> Option<(Vector3<f32>, f32)> {
        relative_mouse_position.x *= 1920. / 1080.;
        let ray_origin = (self.transform.try_inverse().unwrap()
            * camera.try_inverse().unwrap()
            * Vector4::new(relative_mouse_position.x, relative_mouse_position.y, 0., 1.))
        .xyz();
        let cam_direction = camera.z_axis();
        let ray_direction = (self.transform.try_inverse().unwrap()
            * Vector4::new(cam_direction.x, cam_direction.y, cam_direction.z, 0.))
        .xyz();
        let ray = Ray::new(ray_origin, ray_direction);
        let plane = plane_from_points(
            self.mesh.hitbox.triangles[0].get_vertices(&self.mesh.hitbox.vertices),
        );
        let possible_intersection_point = ray.plane_intersection_point(plane);
        if let None = possible_intersection_point {
            return None;
        }
        let (mut intersection_point, t) = unsafe { possible_intersection_point.unwrap_unchecked() };
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

fn plane_from_points(points: [Vector3<f32>; 3]) -> Vector4<f32> {
    let a_to_b = points[1] - points[0];
    let a_to_c = points[2] - points[0];
    let normal = a_to_c.cross(&a_to_b).normalize();
    let plane_d = points[0].x * normal.x + points[0].y * normal.y + points[0].z * normal.z;
    return Vector4::new(normal.x, normal.y, normal.z, plane_d);
}
