use nalgebra::Vector3;
use objects::GameObject;

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) -> Option<(&GameObject, Vector3<f32>)> {
        let mut collision_point = None;
        let closest_z = f32::MIN;
        if self
            .input
            .get_mouse_button_down(winit::event::MouseButton::Left)
        {
            for object in &self.gameobjects {
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
