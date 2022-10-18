pub struct Camera {
    x: f32,
    y: f32,
    zoom: f32,
}
impl Camera {
    pub fn new() -> Self{
        Self{ x: 0.0, y: 0.0, zoom: 1.0}
    }
    pub fn refresh_camera(&mut self, delta_time: f32) {
        if true { //This needs to be relative mouseposition, however that needs to be implemented.
            self.x += 5.0 * delta_time;
            
        }
        else if false {
            
        }
    }
}