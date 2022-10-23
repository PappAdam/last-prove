use crate::engine::{float::EngineFloat, vector2::Vector2};
use crate::input::Keystate;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector2,
    pub zoom: f32,
    zoom_inc: u8,
    target_zoom: f32,
    drag_origin: Vector2,
    dragging: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector2::default(),
            zoom: 1.0,
            zoom_inc: 4,
            target_zoom: 1.0,
            drag_origin: Vector2::default(),
            dragging: false,
        }
    }
    pub fn refresh_camera(
        &mut self,
        delta_time: f32,
        mouse_position: Vector2,
        middle_mouse_btn_state: &Keystate,
        mouse_wheel: i8,
    ) {
        match middle_mouse_btn_state {
            Keystate::Pressed => {
                self.dragging = true;
                self.drag_origin = mouse_position
            }
            Keystate::Released => self.dragging = false,
            _ => {}
        }
        if self.dragging {
            let dir = mouse_position - self.drag_origin;
            self.position += dir * delta_time;
        }

        if self.target_zoom != self.zoom {
            self.zoom = f32::lerp(self.zoom, self.target_zoom, 0.03);
        }

        if (self.target_zoom - self.zoom).abs() < 0.001 {
            self.zoom = self.target_zoom
        }

        if mouse_wheel != 0 {
        
            self.target_zoom += mouse_wheel as f32 * self.zoom_inc as f32;
        }

    }
}