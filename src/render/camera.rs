use crate::engine::vector2::Vector2;
use crate::input::Keystate;

pub struct Camera {
    pub position: Vector2,
    pub zoom: f32,
    drag_origin: Vector2,
    dragging: bool,
}
impl Camera {
    pub fn new() -> Self{
        Self{ position: Vector2::new(0.0, 0.0), zoom: 1.0, drag_origin: Vector2::new(0.0, 0.0), dragging: false}
    }
    pub fn refresh_camera(&mut self, delta_time: f32, mouse_position: Vector2, middle_mouse_btn_state: &Keystate) {
        match middle_mouse_btn_state {
            Keystate::Pressed => { self.dragging = true; self.drag_origin = mouse_position},
            Keystate::Released => { self.dragging = false},
            _ => {},
        }
        if self.dragging {
            let dir = mouse_position - self.drag_origin;
            self.position += dir * delta_time;
        }
    }
}