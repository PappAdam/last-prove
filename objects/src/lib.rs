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

pub mod tags;
pub mod getters;
pub mod hitbox;
pub mod mesh;
pub mod transformations;

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
        let transform = *Matrix4::new_translation(&create_info.position).scale_object(create_info.scale);
        let transform_ptr = unsafe { &mut *(transform_buf.get_data_pointer(transform_index)) };
        *transform_ptr = transform;
        Ok(Self {
            flags: 0,
            tags: vec![],
            transform: transform_ptr,
            transform_index,
            mesh
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
#[derive(Default)]
pub struct GameObjectCreateInfo {
    position: Vector3<f32>,
    //We don't need rotation for now, add later if needed
    scale: f32,
}
impl GameObjectCreateInfo {
    #[inline]
    pub fn position(position: Vector3<f32>) -> Self {
        Self {
            position,
            scale: 1.,
        }
    }

    #[inline]
    pub fn position_scale(position: Vector3<f32>, scale: f32) -> Self {
        Self { position, scale }
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
