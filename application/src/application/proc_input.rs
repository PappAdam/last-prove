use std::f32::consts::PI;

use nalgebra::{Vector2, Vector3};
use objects::{getters::Getters, transformations::Transformations};
use winit::event::MouseButton;

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn camera_move(&mut self) {
        if self.input.get_mouse_button_down(MouseButton::Middle) {
            self.camera
                .orbit(
                    0.,
                    -self.input.mouse.delta_move.x * 0.001,
                    0.,
                    Vector3::new(0., 0., 0.),
                )
                .orbit_local(
                    self.input.mouse.delta_move.y * 0.001,
                    0.,
                    0.,
                    Vector3::new(0., 0., 0.),
                );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::W) {
            let direction = -self.camera.z_axis().xz().normalize() * self.delta_time.as_secs_f32();
            self.camera.translate(direction.x, 0., direction.y);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::S) {
            let direction = self.camera.z_axis().xz().normalize() * self.delta_time.as_secs_f32();
            self.camera.translate(direction.x, 0., direction.y);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::A) {
            self.camera.translate_local(1. * self.delta_time.as_secs_f32(), 0., 0.);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::D) {
            self.camera.translate_local(-1. * self.delta_time.as_secs_f32(), 0., 0.);
        }
        Transformations::scale(
            &mut self.camera,
            1. + self.input.get_mouse_wheel() * 0.2,
            1. + self.input.get_mouse_wheel() * 0.2,
            1. + self.input.get_mouse_wheel() * 0.2,
        );
    }
}
