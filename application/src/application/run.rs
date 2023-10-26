use crate::input::EventState;

use objects::{
    transformations::Transformations, GameObjectCreateInfo, GameObjectFlag, GameObjectTransform,
    MeshPreset,
};
use winit::event::VirtualKeyCode;

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn main_loop(&mut self) {
        if let Some((clicked_object, click_position)) = self.world_mouse_intersection_point() {
            if clicked_object.has_flag(GameObjectFlag::Map) {
                if self
                    .input
                    .mouse_button_state(winit::event::MouseButton::Left, EventState::Pressed)
                {
                    self.create_obj(
                        &GameObjectCreateInfo::default()
                            .mesh_preset(MeshPreset::House)
                            .transform(GameObjectTransform::default().position(click_position)),
                    );
                }
                self.gameobjects[1].transform.set_position(click_position);
                
            }
        }

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
        self.create_obj(&GameObjectCreateInfo::default().mesh_preset(MeshPreset::Map).flags(&[GameObjectFlag::Map]));
        self.create_obj(
            &GameObjectCreateInfo::default()
                .mesh_preset(MeshPreset::MapSelection)
                .flags(&[GameObjectFlag::NotClickable]),
        );
    }
}
