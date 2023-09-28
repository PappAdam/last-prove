use std::{f32::consts::PI, time::Duration};

use nalgebra::{Matrix4, Vector2};
use objects::{hitbox::Hitbox, mesh::Mesh, GameObject, GameObjectCreateInfo};
use renderer::{
    engine::{aligned_array::AlignedArray, object_vector::ObjVec},
    utils::MAX_WORLD_OBJECTS,
    Renderer,
};
use winit::window::Window;

use crate::{input::Input, map::Map};

use self::{camera::Camera, gamecontroller::GameController};

mod camera;
pub mod click;
mod gamecontroller;
pub mod load;
pub mod run;

pub struct App<'a> {
    pub input: Input,
    pub renderer: Renderer,

    map: Map,
    gameobjects: ObjVec<GameObject<'a>>,
    game_controller: GameController,

    transform_array: AlignedArray<Matrix4<f32>>,

    pub camera: Camera,

    //It is like minecraft's time, going from 0 to 65535
    pub delta_time: Duration,
}

impl<'a> App<'a> {
    pub fn init(window: &Window, map_size: usize) -> Self {
        let mut renderer = Renderer::new(window).expect("Failed to setup renderer");
        let map = Map::generate(map_size);
        Self {
            input: Input::init(),

            map,
            gameobjects: ObjVec::with_capacity(MAX_WORLD_OBJECTS),
            game_controller: GameController::init(&mut renderer),

            transform_array: AlignedArray::from_dynamic_ub_data(
                &renderer.data.dynamic_uniform_buffer,
            ),

            renderer,

            camera: Camera::init(
                Vector2::new(-(map_size as f32) / 2., -(map_size as f32) / 2.),
                PI / 6.,
                0.1,
            ),

            delta_time: Duration::ZERO,
        }
    }

    /// ##Gameobject creation
    /// returns the index of the created gameobject
    pub fn create_obj(
        &mut self,
        mesh: &'a Mesh,
        hitbox: &'a Hitbox,
        create_info: &GameObjectCreateInfo,
    ) -> usize {
        let obj = GameObject::create(&mut self.transform_array, mesh, hitbox, create_info)
            .expect("Failed to create gameObject");

        self.gameobjects.push(obj)
    }
}
