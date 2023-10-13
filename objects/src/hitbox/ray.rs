use std::ops::{Mul, MulAssign};

use nalgebra::{Matrix4, Vector3, Vector4};

use crate::mesh::primitives::Polygon;

use super::Hitbox;

#[derive(Debug)]
pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    #[inline]
    ///Creates a new ray with a specified origin, and direction.
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    #[inline]
    ///Returns the closest collision point of the hitbox and the ray.
    ///Returns *None* if no intersection point was found.
    pub fn hitbox_intersection_point(&self, hitbox: &Hitbox) -> Option<(Vector3<f32>, f32)> {
        //Setting None as default, modifying it as we find one.
        let mut intersection_point = None;

        //Checking all quads
        for quad in &hitbox.quads {
            //Gets the intersection point if it is inside the shape.
            let quad_intersection_point = self.polygon_intersection_point(quad, &hitbox.vertices);

            //Continue if there is no intersection point
            if quad_intersection_point.is_none() {
                continue;
            }

            //If intersection point is none, we can just set it to the found one.
            if intersection_point.is_none() {
                intersection_point = quad_intersection_point;
            }

            //Neither intersection point is None at this point
            //If quad_intersection_point is closer, we set it as the new intersection point (Min search by distance)
            if unsafe {
                quad_intersection_point.unwrap_unchecked().1
                    < intersection_point.unwrap_unchecked().1
            } {
                intersection_point = quad_intersection_point;
            }
        }
        //End of quad checks

        //Checking all triangles
        for triangle in &hitbox.triangles {
            let triangle_intersection_point =
                self.polygon_intersection_point(triangle, &hitbox.vertices);
            //Continue if there is no intersection point
            if triangle_intersection_point.is_none() {
                continue;
            }

            //If intersection point is none, we can just set it to the found one.
            if intersection_point.is_none() {
                intersection_point = triangle_intersection_point;
            }

            //Neither intersection point is None at this point
            //If triangle_intersection_point is closer, we set it as the new intersection point (Min search by distance)
            if unsafe {
                triangle_intersection_point.unwrap_unchecked().1
                    < intersection_point.unwrap_unchecked().1
            } {
                intersection_point = triangle_intersection_point;
            }
        }
        //End of triangle checks

        //Returning the intersection point
        intersection_point
    }

    #[inline]
    ///Returns the intersection point with a polygon.
    ///Returns *None* if no intersection point was found.
    fn polygon_intersection_point<const CORNER_COUNT: usize>(
        &self,
        polygon: &Polygon<CORNER_COUNT>,
        vertices: &Vec<Vector3<f32>>,
    ) -> Option<(Vector3<f32>, f32)> {
        //Vector of positions of polygon corners
        let shape_corner_positions = polygon.get_vertex_positions(vertices);
        let polygon_plane = plane_from_points(&shape_corner_positions);

        //Getting the intersection point with the plane of the polygon
        let possible_plane_intersection_point = self.plane_intersection_point(polygon_plane);

        //Intersection point is None if the plane is paralell with the ray direction
        if let None = possible_plane_intersection_point {
            return None;
        }

        //Intersection point cannot be None, extracting values
        let plane_intersection_point =
            unsafe { possible_plane_intersection_point.unwrap_unchecked().0 };

        //Checking for intersection point to be inside the polygon
        for corner_index in 0..CORNER_COUNT {
            //We make a vector that represents an edge of the polygon.
            let a = shape_corner_positions[corner_index];
            let b = shape_corner_positions[(corner_index + 1) % CORNER_COUNT];
            let a_to_b = b - a;

            //If intersection point is inside the polygon this vector will face the same direction as the normal
            let corner_intersection_point_normal = (plane_intersection_point - a).cross(&a_to_b);

            //If corner_intersection_point_normal doesn't face the the same direction as the normal, the intersection point is outside the polygon.
            //This dot product can be 1 (facing same direction) or -1 (facing opposite direction)
            //Returning early if the point isn't inside Polygon
            if corner_intersection_point_normal.dot(&polygon.normal) < 0. {
                return None;
            }
        }
        //If we didn't return early, that means the point is inside the polygon, we can return with the intersection point.
        possible_plane_intersection_point
    }

    #[inline]
    ///Returns the intersection point with a plane.
    ///Returns *None* if the plane is paralell with the ray.
    ///Takes the plane **equation of a plane** as a parameter (a, b, c, d) as a Vector4<f32>
    fn plane_intersection_point(&self, plane: Vector4<f32>) -> Option<(Vector3<f32>, f32)> {
        //Extracting normal vector and d from parameter.
        let plane_normal = plane.xyz();
        let plane_d = plane.w;

        //If the dot product of the normal and the ray direction is 0, the plane is paralell to the ray.
        let dot_direction_normal = plane_normal.dot(&self.direction);
        //If the plane is paralell to the ray, there are no intersection points.
        if dot_direction_normal.abs() < 1e-3 {
            return None;
        }
        //If the plane isn't paralell to the ray, there is an intersection point.

        //This is just maths, cannot really explain it.
        let dot_origin_normal = plane_normal.dot(&self.origin);

        //Origin + Direction * t = Intersection point
        let t = (plane_d - dot_origin_normal) / dot_direction_normal;
        let intersection_point = self.origin + self.direction * t;

        Some((intersection_point, t))
    }
}

#[inline]
///Retuns the plane equation of a plane based on the first three points of the polygon.
///Doesn't check if the polygons are on the same plane or not.
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
