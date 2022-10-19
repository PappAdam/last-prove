use crate::engine::vector2::Vector2;

pub struct Camera {
    pub position: Vector2,
    pub zoom: f32,
    speed: f32,
}
impl Camera {
    pub fn new() -> Self{
        Self{ position: Vector2::new(0.0, 0.0), zoom: 1.0, speed: 150.0}
    }
    pub fn refresh_camera(&mut self, delta_time: f32, rel_mouse_position: (f32, f32)) {
        if rel_mouse_position.0 > 0.9 {
            self.position.x += self.speed * delta_time;
            
        }
        else if rel_mouse_position.0 < 0.1 {
            self.position.x -= self.speed * delta_time;
        }
        if rel_mouse_position.1 > 0.9 {
            self.position.y += self.speed * delta_time;
            
        }
        else if rel_mouse_position.1 < 0.1 {
            self.position.y -= self.speed * delta_time;
        }
    }
}