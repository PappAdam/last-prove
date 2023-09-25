use nalgebra::{Matrix4, Vector3};
use objects::{getters::Getters, transformations::Transformations};
use winit::event::MouseButton;

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn camera_move(&mut self) {
        // if self.input.get_mouse_button_down(MouseButton::Middle) {
        if self.input.get_mouse_button_down(MouseButton::Middle) {
            self.camera
                .orbit(
                    0.,
                    -self.input.mouse.delta_move.x * 0.001,
                    0.,
                    // self.camera_view_location / self.camera.get_scale(),
                    Vector3::zeros(),
                )
                .orbit_local(
                    self.input.mouse.delta_move.y * 0.001,
                    0.,
                    0.,
                    // self.camera_view_location,
                    Vector3::zeros(),
                );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::W) {
            let direction = (-self.camera.z_axis()).xz().normalize()
                * self.delta_time.as_secs_f32()
                / self.camera.get_scale();
            self.camera.translate(direction.x, 0., direction.y);
            dbg!(self.camera.get_position());
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::S) {
            let direction = self.camera.z_axis().xz().normalize() * self.delta_time.as_secs_f32()
                / self.camera.get_scale();
            self.camera.translate(direction.x, 0., direction.y);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::A) {
            self.camera
                .translate_local(1. * self.delta_time.as_secs_f32(), 0., 0.);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::D) {
            self.camera
                .translate_local(-1. * self.delta_time.as_secs_f32(), 0., 0.);
        }
        let scale_factor = 1.2_f32.powi(self.input.get_mouse_wheel() as i32);
        self.camera *= Matrix4::new_scaling(scale_factor);
        self.camera[12] *= scale_factor;
        self.camera[13] *= scale_factor;
        self.camera[14] *= scale_factor;
    }
}
