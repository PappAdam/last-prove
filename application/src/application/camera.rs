use std::f32::consts::PI;

use nalgebra::{Matrix4, Vector2, Vector3};
use objects::{getters::Getters, transformations::Transformations};
use winit::event::MouseButton;

use crate::{input::Input, MAP_SIZE};

pub struct Camera {
    transform: Matrix4<f32>,
    //Keeping track of camera's properties, same values can be extracted from transform matrix
    position: Vector2<f32>,
    tilt: f32,
    scale: f32,
}

impl Camera {
    #[inline]
    pub fn init(position: Vector2<f32>, tilt: f32, scale: f32) -> Self {
        Self {
            transform: *Matrix4::identity()
                .scale(scale)
                .translate(position.x, 0., position.y)
                .orbit(0., PI / 4., 0., Vector3::zeros())
                .orbit_local(tilt, 0., 0., Vector3::zeros()),
            position,
            tilt,
            scale,
        }
    }
    #[inline]
    pub fn get_transform(&self) -> &Matrix4<f32> {
        &self.transform
    }
    #[inline]
    pub fn get_position(&self) -> &Vector2<f32> {
        &self.position
    }
    #[inline]
    pub fn translate_camera(&mut self, translation: Vector2<f32>) -> &mut Self {
        self.transform.translate(translation.x, 0., translation.y);
        self.position += translation;
        self
    }
    #[inline]
    pub fn scale_camera(&mut self, scale: f32) -> &mut Self {
        self.scale *= scale;
        Transformations::scale(&mut self.transform, scale);
        self.transform[12] *= scale;
        self.transform[13] *= scale;
        self.transform[14] *= scale;
        self
    }
    #[inline]
    pub fn tilt_camera(&mut self, tilt: f32) -> &mut Self {
        self.tilt += tilt;
        self.transform.orbit_local(tilt, 0., 0., Vector3::zeros());
        self
    }
    #[inline]
    pub fn camera_move(&mut self, input: &Input, delta_time_seconds: f32) {
        //CAMERA ROTATION
        if input.get_mouse_button_down(MouseButton::Middle) {
            //Horizontal rotation, no constraits
            self.transform
                .orbit(0., -input.mouse.delta_move.x * 0.001, 0., Vector3::zeros());

            //Vertical rotation
            let camera_tilt_change = input.mouse.delta_move.y * 0.001;
            //Vertical rotation constraints
            if (self.tilt < PI / 2. && camera_tilt_change > 0.)
                || (self.tilt > PI / 12. && camera_tilt_change < 0.)
            {
                self.tilt_camera(camera_tilt_change);
            }
        }

        //CAMERA TRANSLATION
        let mut camera_position_change = Vector2::zeros();
        //Keyboard inputs, collecting translations
        if input.get_key_down(winit::event::VirtualKeyCode::W) {
            let direction = (self.transform.y_axis()).xz().normalize() / self.scale;
            camera_position_change += direction;
        }
        if input.get_key_down(winit::event::VirtualKeyCode::S) {
            let direction = (-self.transform.y_axis()).xz().normalize() / self.scale;
            camera_position_change += direction;
        }
        if input.get_key_down(winit::event::VirtualKeyCode::A) {
            camera_position_change += self.transform.x_axis().xz();
        }
        if input.get_key_down(winit::event::VirtualKeyCode::D) {
            camera_position_change += -self.transform.x_axis().xz();
        }

        //Camera moving constaints
        if (self.position.x > 0. && camera_position_change.x > 0.)
            || (self.position.x < -(MAP_SIZE as f32) && camera_position_change.x < 0.)
        {
            camera_position_change.x = 0.;
        }
        if (self.position.y > 0. && camera_position_change.y > 0.)
            || (self.position.y < -(MAP_SIZE as f32) && camera_position_change.y < 0.)
        {
            camera_position_change.y = 0.;
        }

        //Applying translation
        camera_position_change *= delta_time_seconds;
        self.translate_camera(camera_position_change);

        //CAMERA SCALING
        let scale_factor = 1.2_f32.powi(input.get_mouse_wheel() as i32);
        //Scaling constaints
        // if (self.scale > 0.02 && scale_factor < 1.) || (self.scale < 0.5 && scale_factor > 1.) {
            //Applying scaling
            self.scale_camera(scale_factor);
        // }
    }
}
