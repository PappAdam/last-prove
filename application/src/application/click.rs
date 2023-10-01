use nalgebra::Vector2;

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) {
        if self
            .input
            .get_mouse_button_down(winit::event::MouseButton::Left)
        {
            for object in &self.gameobjects {
                object.check_object_clicked(
                    self.camera.get_transform(),
                    self.input.get_relative_mouse_position(),
                );
            }
        }
    }
}
