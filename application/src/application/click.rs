use nalgebra::Vector3;
use objects::GameObject;

use crate::input::EventState;

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) -> Option<(&GameObject, Vector3<f32>)> {
        let mut collision_point = None;
        let closest_z = f32::MIN;
        if self
            .input
            .mouse_button_state(winit::event::MouseButton::Left, EventState::Down)
        {
            for object in &self.gameobjects {
                if object.flag_active(objects::GameObjectFlag::NotClickable) {
                    continue;
                }
                if let Some((clicked_position, screen_z)) = object.check_object_clicked(
                    self.camera.get_transform(),
                    self.input.get_relative_mouse_position(),
                ) {
                    if screen_z > closest_z {
                        collision_point = Some((object, clicked_position));
                    }
                }
            }
        }
        collision_point
    }
}
