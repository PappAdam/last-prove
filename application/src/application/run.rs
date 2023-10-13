use nalgebra::Vector3;
use objects::{hitbox::{Hitbox, Triangle, self}, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};
use winit::event::MouseButton;


use crate::input::EventState;

use super::{
    load::MeshPreset,
    App,
};

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        for gameobject in &self.gameobjects {
            gameobject.render(&self.renderer);
        }

        if self.input.mouse_button_state(MouseButton::Right, EventState::Down) {
            println!("asd");
        }
    }

    pub fn setup(&mut self, meshes: &'a mut Vec<Mesh>) {
        self.load_meshes(meshes);

        self.create_obj(
            MeshPreset::Map,
            &GameObjectCreateInfo::position(Vector3::new(0., 0., 0.)),
        );

    }
}
