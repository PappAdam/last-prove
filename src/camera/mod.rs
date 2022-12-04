use crate::{engine::vector2::Vector2, input::Input};

pub struct Camera {
    pub coordinates: Vector2, //The cameras coordinates are the coordinates of the tile in the center
    pub tile_size: u8,
    pub camera_size: Vector2,
}

impl Camera {
    pub fn new(camera_size: Vector2) -> Self {
        Self {
            coordinates: Vector2::zero(),
            tile_size: 64,
            camera_size,
        }
    }

    pub fn refresh_camera(&mut self, input: &Input) {
        let mouse_wheel = input.get_mouse_wheel();
        if mouse_wheel > 0 && self.tile_size < 255 {
            self.tile_size += 1;
        } else if mouse_wheel < 0 && self.tile_size > 4 {
            self.tile_size -= 1;
        }

        if input.get_mousebutton_down(winit::event::MouseButton::Middle) {
            self.coordinates -= self.screen_position_to_coordinates(input.get_mouse_movement());
        }
    }

    pub fn window_resized(&mut self, new_screen_size: Vector2) {
        self.camera_size = new_screen_size;
    }

    pub fn screen_position_to_coordinates(&self, screen_position: Vector2) -> Vector2 {
        let x = screen_position.x / self.tile_size as f32 * 2.0
            + screen_position.y / self.tile_size as f32 * 4.0;
        let y = -screen_position.x / self.tile_size as f32 * 2.0
            + screen_position.y / self.tile_size as f32 * 4.0;
        Vector2::new(x, y)
    }
}
