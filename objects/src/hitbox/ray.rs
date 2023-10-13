use std::ops::{Mul, MulAssign};

use nalgebra::{Matrix4, Vector3, Vector4};

use crate::mesh::primitives::Polygon;

use super::Hitbox;

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn hitbox_intersection_point(&self, hitbox: &Hitbox) -> Option<(Vector3<f32>, f32)> {
        let mut intersection_point = None;
        //Checking all quads
        for quad in &hitbox.quads {
            let quad_plane = plane_from_points(&quad.get_vertex_positions(&hitbox.vertices));
            let possible_new_intersection_point = self.plane_intersection_point(quad_plane);
            if intersection_point.is_none() {
                intersection_point = possible_new_intersection_point;
                continue;
            }
            if let Some(new_intersection_point) = possible_new_intersection_point {
                if new_intersection_point.1 < unsafe { intersection_point.unwrap_unchecked().1 } {
                    intersection_point = possible_new_intersection_point
                }
            }
        }
        //Checking all triangles
        for triangle in &hitbox.triangles {
            let triangle_plane =
                plane_from_points(&triangle.get_vertex_positions(&hitbox.vertices));
            let possible_new_intersection_point = self.plane_intersection_point(triangle_plane);
            if intersection_point.is_none() {
                intersection_point = possible_new_intersection_point;
                continue;
            }
            if let Some(new_intersection_point) = possible_new_intersection_point {
                if new_intersection_point.1 < unsafe { intersection_point.unwrap_unchecked().1 } {
                    intersection_point = possible_new_intersection_point
                }
            }
        }
        intersection_point
    }

    #[inline]
    fn shape_intersection_point<const CORNER_COUNT: usize> (&self, shape: Polygon<CORNER_COUNT>, vertices: &Vec<Vector3<f32>>) -> Option<(Vector3<f32>, f32)> {
        None
    }

    #[inline]
    fn plane_intersection_point(&self, plane: Vector4<f32>) -> Option<(Vector3<f32>, f32)> {
        let plane_normal = plane.xyz();
        let plane_d = plane.w;

        let dot_direction_normal = plane_normal.dot(&self.direction);

        //The ray is paralell to the plane -> No intersection
        if dot_direction_normal.abs() < 1e-3 {
            return None;
        }

        let dot_origin_normal = plane_normal.dot(&self.origin);

        //Origin + Direction * t = Intersection point
        let t = (plane_d - dot_origin_normal) / dot_direction_normal;

        let intersection_point = self.origin + self.direction * t;

        Some((intersection_point, t))
    }
}

#[inline]
fn plane_from_points(points: &Vec<Vector3<f32>>) -> Vector4<f32> {
    debug_assert!(
        points.len() >= 3,
        "At least 3 points are needed to determine plane equation!"
    );
    let a_to_b = points[1] - points[0];
    let a_to_c = points[2] - points[0];
    let normal = a_to_c.cross(&a_to_b).normalize();
    let plane_d = points[0].x * normal.x + points[0].y * normal.y + points[0].z * normal.z;
    return Vector4::new(normal.x, normal.y, normal.z, plane_d);
}

impl MulAssign<Matrix4<f32>> for Ray {
    fn mul_assign(&mut self, rhs: Matrix4<f32>) {
        self.origin = (rhs * Vector4::new(self.origin.x, self.origin.y, self.origin.z, 1.)).xyz();
        self.direction =
            (rhs * Vector4::new(self.direction.x, self.direction.y, self.direction.z, 0.)).xyz();
    }
}

impl Mul<&Ray> for Matrix4<f32> {
    type Output = Ray;

    fn mul(self, rhs: &Ray) -> Self::Output {
        let origin = (self * Vector4::new(rhs.origin.x, rhs.origin.y, rhs.origin.z, 1.)).xyz();
        let direction =
            (self * Vector4::new(rhs.direction.x, rhs.direction.y, rhs.direction.z, 0.)).xyz();
        Ray::new(origin, direction)
    }
}
