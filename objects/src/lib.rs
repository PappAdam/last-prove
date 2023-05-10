use std::panic;

use mesh::Mesh;
use nalgebra::Matrix4;
use renderer::{engine::aligned_array::AlignedArray, resources::buffer::Buffer};

pub mod mesh;
pub mod transformations;

pub enum ObjectUsage<'a> {
    Empty,
    Object(&'a Mesh),
}

pub enum ObjectType {
    Camera,
    SomeObject,
    //  ...
}

pub struct GameObject<'a> {
    object_type: ObjectType,
    object_usage: ObjectUsage<'a>,
    transform: &'a mut Matrix4<f32>,
    transform_index: usize,
}

impl<'a> GameObject<'a> {
    pub fn object(
        transform_buf: &mut AlignedArray<Matrix4<f32>>,
        mesh: &'a Mesh,
        ty: ObjectType,
    ) -> Result<Self, ObjectCreationError> {
        let transform_index = transform_buf
            .push(Matrix4::identity())
            .map_err(|_| ObjectCreationError::NotEnoughSpace)?;

        Ok(Self {
            object_type: ty,
            object_usage: ObjectUsage::Object(mesh),
            transform: unsafe { &mut *(transform_buf.get_data_pointer(transform_index)) },
            transform_index,
        })
    }

    pub fn empty(transform: &'a mut Matrix4<f32>, ty: ObjectType) -> Self {
        Self {
            object_type: ty,
            object_usage: ObjectUsage::Empty,
            transform,
            transform_index: 0,
        }
    }

    pub fn get_mesh(&self) -> &'a Mesh {
        match self.object_usage {
            ObjectUsage::Empty => panic!("Object has usage empty; Unable to get mesh"),
            ObjectUsage::Object(mesh) => mesh,
        }
    }

    #[inline]
    pub fn renderable_form(&self) -> (&Buffer, &Buffer, u32, usize) {
        self.get_mesh().into_tuple(self.transform_index)
    }
}

#[derive(Debug)]
pub enum ObjectCreationError {
    NotEnoughSpace,
}
