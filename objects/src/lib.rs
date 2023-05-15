use std::panic;

use mesh::Mesh;
use nalgebra::Matrix4;
use renderer::{engine::aligned_array::AlignedArray, resources::buffer::Buffer};

pub mod mesh;
pub mod transformations;
pub mod getters;

pub enum ObjectType {
    Camera,
    SomeObject,
    //  ...
}

pub struct GameObject<'a> {
    object_type: ObjectType,
    transform: &'a mut Matrix4<f32>,
    transform_index: usize,
    mesh: &'a Mesh,
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
            transform: unsafe { &mut *(transform_buf.get_data_pointer(transform_index)) },
            transform_index,
            mesh
        })
    }

    pub fn get_mesh(&self) -> &'a Mesh {
        self.mesh
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
