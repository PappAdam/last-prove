use nalgebra::{Vector3, Vector4};

pub struct Ray {
    origin: Vector3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }

    pub fn plane_intersection_point(&self, plane: Vector4<f32>) -> Option<(Vector3<f32>, f32)> {
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
