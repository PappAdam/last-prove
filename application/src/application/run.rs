use crate::input::EventState;
use nalgebra::Vector3;
use objects::{
    hitbox::{self, Hitbox, Triangle},
    mesh::Mesh,
    transformations::Transformations,
    GameObjectCreateInfo, GameObjectTransform, MeshPreset,
};
use winit::event::{MouseButton, VirtualKeyCode};

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        if self
            .input
            .key_state(VirtualKeyCode::Space, EventState::Pressed)
        {
            self.renderer.current_pipeline_index =
                (self.renderer.current_pipeline_index as i8 - 1).abs() as usize;
        }

        for gameobject in &self.gameobjects {
            gameobject.render(&self.renderer);
        }
    }

    pub fn setup(&mut self) {
        self.create_obj(
            &GameObjectCreateInfo::default()
                .mesh_preset(MeshPreset::House)
                .transform(GameObjectTransform::default().scale(100.)),
        );
    }
}
