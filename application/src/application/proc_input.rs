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
        // if self.input.get_key_down(winit::event::VirtualKeyCode::W) {
        //     let direction = self.camera.x_axis().cross(&Vector3::y_axis()).normalize()
        //         * self.delta_time.as_secs_f32()
        //         * 5.;
        //     self.camera.translate(direction.x, 0., direction.z);
        // }
        // if self.input.get_key_down(winit::event::VirtualKeyCode::S) {
        //     let direction = Vector3::y_axis().cross(&self.camera.x_axis()).normalize()
        //         * self.delta_time.as_secs_f32()
        //         * 5.;
        //     self.camera.translate(direction.x, 0., direction.z);
        // }
        // if self.input.get_key_down(winit::event::VirtualKeyCode::A) {
        //     self.camera
        //         .translate_local(1. * self.delta_time.as_secs_f32() * 5., 0., 0.);
        // }
        // if self.input.get_key_down(winit::event::VirtualKeyCode::D) {
        //     self.camera
        //         .translate_local(-1. * self.delta_time.as_secs_f32() * 5., 0., 0.);
        // }
    }
}