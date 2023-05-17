use std::f32::consts::PI;

use nalgebra::Vector3;
use objects::{getters::Getters, transformations::Transformations};

use super::App;

impl<'a> App<'a> {
    #[inline]
    pub fn camera_move(&mut self) {
        if self.input.get_key_down(winit::event::VirtualKeyCode::Q) {
            self.camera.orbit(
                0.,
                (PI / 2.) * self.delta_time.as_secs_f32(),
                0.,
                Vector3::new(0., 0., 0.),
            );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::E) {
            self.camera.orbit(
                0.,
                -(PI / 2.) * self.delta_time.as_secs_f32(),
                0.,
                Vector3::new(0., 0., 0.),
            );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::R) {
            self.camera.orbit_local(
                (PI / 2.) * self.delta_time.as_secs_f32(),
                0.,
                0.,
                Vector3::new(0., 0., 0.),
            );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::F) {
            self.camera.orbit_local(
                -(PI / 2.) * self.delta_time.as_secs_f32(),
                0.,
                0.,
                Vector3::new(0., 0., 0.),
            );
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::W) {
            let direction = self.camera.x_axis().cross(&Vector3::y_axis()).normalize()
                * self.delta_time.as_secs_f32();
            self.camera.translate(direction.x, 0., direction.z);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::S) {
            let direction = Vector3::y_axis().cross(&self.camera.x_axis()).normalize()
                * self.delta_time.as_secs_f32();
            self.camera.translate(direction.x, 0., direction.z);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::A) {
            self.camera
                .translate_local(1. * self.delta_time.as_secs_f32(), 0., 0.);
        }
        if self.input.get_key_down(winit::event::VirtualKeyCode::D) {
            self.camera
                .translate_local(-1. * self.delta_time.as_secs_f32(), 0., 0.);
        }

        if self.input.get_key_down(winit::event::VirtualKeyCode::L) {
            dbg!(self.camera.z_axis());
        }
    }
}
