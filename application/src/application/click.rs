use nalgebra::{Vector2, Vector3};

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) -> Option<Vector3<f32>> {
        if self
            .input
            .get_mouse_button_down(winit::event::MouseButton::Left)
        {
            for object in &self.gameobjects {
                if let Some(clicked_position) = object.check_object_clicked(
                    self.camera.get_transform(),
                    self.input.get_relative_mouse_position(),
                ) {
                    return Some(clicked_position);
                }
            }
        }
        None
    }
}
