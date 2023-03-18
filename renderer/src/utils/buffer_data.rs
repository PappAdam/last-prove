use std::f64::consts;

use crate::{
    engine::lin_alg::{Convert, Vector2, Vector3},
    msg,
};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vector2<f32>,
    pub color: Vector3<f32>,
}

impl Vertex {
    pub fn from_pos(pos: Vector2<f32>) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn new(pos: Vector2<f32>, color: Vector3<f32>) -> Self {
        Self { pos, color }
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            pos: Vector2::default(),
            color: Vector3 {
                x: 1.,
                y: 1.,
                z: 0.3,
            },
        }
    }
}

pub fn quad() -> Vec<Vertex> {
    let mut vertecies: Vec<Vertex> = Vec::with_capacity(4000000);

    for y in 0..2 {
        for x in 0..2 {
            vertecies.push(Vertex::from_pos(
                [x as f32 / 30. - 1., y as f32 / 30. - 1.].conv(),
            ))
        }
    }

    vertecies
}
