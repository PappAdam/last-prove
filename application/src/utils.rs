use std::ops::{BitAnd, BitOr, Not};

use nalgebra::Vector3;
use renderer::utils::buffer_data::Vertex;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Side(u8);

impl Side {
    pub const FRONT: Self = Self(0b1);
    pub const BACK: Self = Self(0b10);
    pub const TOP: Self = Self(0b100);
    pub const BOTTOM: Self = Self(0b1000);
    pub const LEFT: Self = Self(0b10000);
    pub const RIGHT: Self = Self(0b100000);

    pub const CUBE: Self = Self(!0);

    pub fn to_array() -> [Side; 6] {
        [
            Self::FRONT,
            Self::BACK,
            Self::TOP,
            Self::BOTTOM,
            Self::LEFT,
            Self::RIGHT,
        ]
    }
}

impl BitOr for Side {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Side {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

fn get_face_indicies_from_side(side: Side) -> ([u16; 6], Vector3<f32>) {
    match side {
        Side::FRONT => ([0, 2, 1, 1, 2, 3], Vector3::new(0., 0., 1.)),
        Side::BACK => ([5, 7, 4, 4, 7, 6], Vector3::new(0., 0., -1.)),
        Side::TOP => ([4, 0, 5, 5, 0, 1], Vector3::new(0., 1., 0.)),
        Side::BOTTOM => ([2, 6, 3, 3, 6, 7], Vector3::new(0., -1., 0.)),
        Side::LEFT => ([4, 6, 0, 0, 6, 2], Vector3::new(-1., 0., 0.)),
        Side::RIGHT => ([1, 3, 5, 5, 3, 7], Vector3::new(1., 0., 0.)),
        _ => ([0; 6], Vector3::zeros()),
    }
}

pub fn create_cube(
    sides: Side,
    position: Vector3<f32>,
    dimensions: Vector3<f32>,
    color: Vector3<f32>,
    size_multip: f32,
) -> Vec<Vertex> {
    let mut vertecies = [Vertex::default(); 8];
    for z in 0..2 {
        for y in 0..2 {
            for x in 0..2 {
                vertecies[z * 4 + y * 2 + x] = Vertex::new(
                    Vector3::new(
                        x as f32 * dimensions.x + position.x - dimensions.x,
                        y as f32 * dimensions.y + position.y - dimensions.y,
                        z as f32 * dimensions.z + position.z - dimensions.z,
                    ) * size_multip
                        + Vector3::new(size_multip, size_multip, size_multip),
                    color,
                    Default::default(),
                );
            }
        }
    }

    let mut cube_verts = Vec::<Vertex>::new();

    for side in Side::to_array() {
        if sides & side != Side(0) {
            let side = get_face_indicies_from_side(side);
            side.0.into_iter().for_each(|i| {
                let mut new_vert = vertecies[i as usize].clone();
                new_vert.normal = side.1;
                cube_verts.push(new_vert);
            });
        }
    }

    cube_verts
}
