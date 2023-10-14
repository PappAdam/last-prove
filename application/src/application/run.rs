use nalgebra::Vector3;
use objects::{hitbox::{Hitbox, Triangle, self}, mesh::Mesh, transformations::Transformations, GameObjectCreateInfo};
use winit::event::{MouseButton, VirtualKeyCode};
use crate::input::EventState;

use super::{
    load::MeshPreset,
    App,
};

use crate::MAP_SIZE;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        if self.input.mouse_button_state(MouseButton::Right, EventState::Pressed) {
            if let Some((_, pos)) = self.click_detection() {
                let ind = self.create_obj(MeshPreset::House, &GameObjectCreateInfo::position(pos));
                self.gameobjects[ind].transform.scale_object(100.);
            }
        }

        if self.input.key_state(VirtualKeyCode::Space, EventState::Pressed) {
            self.renderer.current_pipeline_index = (self.renderer.current_pipeline_index as i8 - 1).abs() as usize;
        }

        for gameobject in &self.gameobjects {
            gameobject.render(&self.renderer);
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
