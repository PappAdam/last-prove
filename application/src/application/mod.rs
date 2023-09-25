use std::time::Duration;

use nalgebra::{Matrix4, Vector3};
use objects::{mesh::Mesh, GameObject, GameObjectCreateInfo};
use renderer::{
    engine::{aligned_array::AlignedArray, object_vector::ObjVec},
    utils::MAX_WORLD_OBJECTS,
    Renderer,
};
use winit::window::Window;

use crate::input::Input;

use self::gamecontroller::GameController;

mod gamecontroller;
pub mod proc_input;
pub mod run;

pub struct App<'a> {
    pub input: Input,
    pub renderer: Renderer,

    gameobjects: ObjVec<GameObject<'a>>,
    game_controller: GameController,

    transform_array: AlignedArray<Matrix4<f32>>,
    camera: Matrix4<f32>,
    pub camera_view_location: Vector3<f32>,

    //It is like minecraft's time, going from 0 to 65535
    pub delta_time: Duration,
}

impl<'a> App<'a> {
    pub fn init(window: &Window) -> Self {
        let mut renderer = Renderer::new(window).expect("Failed to setup renderer");
        Self {
            input: Input::init(),
            transform_array: AlignedArray::from_dynamic_ub_data(
                &renderer.data.dynamic_uniform_buffer,
            ),
            game_controller: GameController::init(&mut renderer),
            renderer,
            camera: Matrix4::identity(),
            camera_view_location: Vector3::new(0., 0., 0.),
            gameobjects: ObjVec::with_capacity(MAX_WORLD_OBJECTS),
            delta_time: Duration::ZERO,
        }
    }

    /// ##Gameobject creation
    /// returns the index of the created gameobject
    pub fn create_obj(&mut self, mesh: &'a Mesh, create_info: &GameObjectCreateInfo) -> usize {
        let obj = GameObject::create(&mut self.transform_array, mesh, create_info)
            .expect("Failed to create gameObject");

        self.gameobjects.push(obj)
    }

    #[inline]
    pub fn load_mesh(&mut self, path: &str, meshes: &mut Vec<Mesh>) {
        meshes.push(Mesh::from_file(&mut self.renderer, path));
    }

    pub fn get_cam(&'a self) -> &'a Matrix4<f32> {
        &self.camera
    }
}
