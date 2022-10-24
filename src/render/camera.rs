use crate::engine::{float::EngineFloat, vector2::Vector2};

#[derive(Debug)]
pub struct Camera {
    pub position: Vector2,
    pub zoom: f32,
    zoom_inc: u8,
    //target_zoom: f32,
    //Smooth zooming is taken out due to it working weirdly
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector2::default(),
            zoom: 0.0,
            zoom_inc: 4,
            //target_zoom: 0.0,
        }
    }
    pub fn refresh_camera(
        &mut self,
        mouse_movement: Vector2,
        middle_mouse_btn: bool,

        mouse_wheel: i8,
    ) {
        if middle_mouse_btn {
            self.position -= mouse_movement;
        }

        //if self.target_zoom != self.zoom {
        //    self.zoom = f32::lerp(self.zoom, self.target_zoom, 0.03);
        //}

        //if (self.target_zoom - self.zoom).abs() < 0.001 {
        //    self.zoom = self.target_zoom
        //}

        if mouse_wheel != 0 {
            self.zoom += mouse_wheel as f32 * self.zoom_inc as f32;
        }
    }
}
