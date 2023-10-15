use std::default;

use ash::vk;
use hitbox::Hitbox;
use mesh::Mesh;
use nalgebra::{Matrix4, Vector3};
use renderer::{
    engine::aligned_array::{AlignedArray, NoneValue},
    Renderer,
};
use tags::ObjectTag;
use transformations::Transformations;

pub mod getters;
pub mod hitbox;
pub mod mesh;
pub mod tags;
pub mod transformations;

#[derive(Default, Clone, Copy)]
pub enum MeshPreset {
    #[default]
    Empty = 0,
    Map,
    House,
    MapSelection,
    Plane,
    Sphere,
}

#[derive(Clone, Copy)]
pub enum GameObjectFlag {
    None = 0b00000001,
    NotClickable = 0b00000010,
}

pub struct GameObject<'a> {
    pub transform: &'a mut Matrix4<f32>,
    transform_index: usize,
    mesh: &'a Mesh,
    pub tags: Vec<ObjectTag>,
    flags: u8,
}

impl<'a> GameObject<'a> {
    pub fn create(
        transform_buf: &mut AlignedArray<Matrix4<f32>>,
        mesh: &'a Mesh,
        create_info: &GameObjectCreateInfo,
    ) -> Result<Self, ObjectCreationError> {
        let transform_index = transform_buf
            .push(Matrix4::identity())
            .map_err(|_| ObjectCreationError::NotEnoughSpace)?;
        let transform = *Matrix4::new_translation(&create_info.transform.position)
            .scale_object(create_info.transform.scale);
        let transform_ptr = unsafe { &mut *(transform_buf.get_data_pointer(transform_index)) };
        *transform_ptr = transform;
        Ok(Self {
            flags: 0,
            tags: vec![],
            transform: transform_ptr,
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
    pub fn flag_active(&self, flag: GameObjectFlag) -> bool {
        self.flags & flag as u8 == flag as u8
    }
    pub fn set_flag(&mut self, flag: GameObjectFlag) {
        self.flags |= flag as u8
    }
}

#[derive(Debug)]
pub enum ObjectCreationError {
    NotEnoughSpace,
}

pub struct GameObjectTransform {
    pub position: Vector3<f32>,
    //We don't need rotation for now, add later if needed
    pub scale: f32,
}

impl Default for GameObjectTransform {
    fn default() -> Self {
        Self {
            scale: 1.,
            position: Vector3::default(),
        }
    }
}

impl GameObjectTransform {
    #[inline]
    pub fn new(position: Vector3<f32>, scale: f32) -> Self {
        Self { position, scale }
    }

    #[inline]
    pub fn position(mut self, position: Vector3<f32>) -> Self {
        self.position = position;
        self
    }

    #[inline]
    pub fn scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
}

impl<'a> NoneValue for GameObject<'a> {
    fn is_none(&self) -> bool {
        self.flag_active(GameObjectFlag::None)
    }

    fn set_to_none(&mut self) {
        self.set_flag(GameObjectFlag::None)
    }
}

#[derive(Default)]
pub struct GameObjectCreateInfo {
    pub transform: GameObjectTransform,
    pub preset: MeshPreset,
}

impl GameObjectCreateInfo {
    pub fn new(transform: GameObjectTransform, preset: MeshPreset) -> Self {
        Self { transform, preset }
    }

    pub fn transform(mut self, transform: GameObjectTransform) -> Self {
        self.transform = transform;
        self
    }

    pub fn mesh_preset(mut self, preset: MeshPreset) -> Self {
        self.preset = preset;
        self
    }
}
