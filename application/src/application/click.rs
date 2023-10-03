use nalgebra::Vector3;

use super::App;

impl<'a> App<'a> {
    pub fn click_detection(&self) -> Option<Vector3<f32>> {
        let mut collision_point = None;
        if self
            .input
            .get_mouse_button_down(winit::event::MouseButton::Left)
        {
            let closest_z = f32::MIN;
            for object in &self.gameobjects {
                if let Some((clicked_position, screen_z)) = object.check_object_clicked(
                    self.camera.get_transform(),
                    self.input.get_relative_mouse_position(),
                ) {
                    if screen_z > closest_z {
                        collision_point = Some(clicked_position);
                    }
                }
            }
        }
        collision_point
    }
}
