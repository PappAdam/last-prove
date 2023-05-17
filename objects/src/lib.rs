use std::panic;

use ash::vk;
use mesh::Mesh;
use nalgebra::Matrix4;
use renderer::{
    engine::aligned_array::{AlignedArray, NoneValue},
    resources::buffer::Buffer,
    Renderer,
};

pub mod getters;
pub mod mesh;
pub mod transformations;

#[derive(PartialEq, PartialOrd)]
pub enum ObjectType {
    None,
    SomeObject,
    //  ...
}

pub struct GameObject<'a> {
    object_type: ObjectType,
    pub transform: &'a mut Matrix4<f32>,
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
            mesh,
        })
    }

    #[inline]
    pub fn render(&self, renderer: &Renderer) {
        renderer.stage_mesh(self.renderable_form())
    }

    pub fn get_mesh(&self) -> &'a Mesh {
        self.mesh
    }

    #[inline]
    pub fn renderable_form(&self) -> (vk::Buffer, vk::Buffer, u32, usize) {
        self.get_mesh().into_tuple(self.transform_index)
    }
}

#[derive(Debug)]
pub enum ObjectCreationError {
    NotEnoughSpace,
}

impl<'a> NoneValue for GameObject<'a> {
    fn is_none(&self) -> bool {
        self.object_type == ObjectType::None
    }

    fn set_to_none(&mut self) {
        self.object_type = ObjectType::None;
    }
}
