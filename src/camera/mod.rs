use crate::{engine::vector2::Vector2, input::Input};

pub struct Camera {
    pub coordinates: Vector2, //The cameras coordinates are the coordinates of the tile in the center
    pub target_coordinates: Vector2,
    pub tile_size: Vector2,
    pub target_tile_size: Vector2,
    pub camera_size: Vector2,
}

impl Camera {
    pub fn new(camera_size: Vector2) -> Self {
        let tile_size = Vector2::new(64u8, 64) / camera_size;
        Self {
            coordinates: Vector2::zero(),
            target_coordinates: Vector2::zero(),
            tile_size,
            target_tile_size: tile_size,
            camera_size,
        }
    }

    pub fn refresh_camera(&mut self, input: &Input, delta_time: f32) {
        self.tile_size = Vector2::lerp(self.tile_size, self.target_tile_size, 60.0 * delta_time);
        self.coordinates = Vector2::lerp(
            self.coordinates,
            self.target_coordinates,
            60.0 * delta_time,
        );
        println!("{}", self.relative_screen_position_to_tile_coordinates(input.get_mouse_position()));

        let mouse_wheel = input.get_mouse_wheel();
        if mouse_wheel > 0 && self.target_tile_size.x < 1.0 && self.target_tile_size.x < 1.0 {
            self.target_tile_size *= 1.2;
            self.target_coordinates = Vector2::lerp(
                self.target_coordinates,
                self.relative_screen_position_to_tile_coordinates(input.get_mouse_position()),
                1.0 - (1.0 / 1.2),
            );
        } else if mouse_wheel < 0
            && self.target_tile_size.x > 0.01
            && self.target_tile_size.x > 0.01
        {
            self.target_tile_size /= 1.2;
            self.target_coordinates = Vector2::lerp(
                self.target_coordinates,
                self.relative_screen_position_to_tile_coordinates(input.get_mouse_position()),
                1.0 - (1.0 / 1.2),
            );
        }

        if input.get_mousebutton_down(winit::event::MouseButton::Middle) {
            self.target_coordinates =
                self.relative_screen_position_to_tile_coordinates(-input.get_mouse_movement());
            self.coordinates = self.target_coordinates
        }
    }

    pub fn window_resized(&mut self, new_screen_size: Vector2) {
        self.target_tile_size = self.target_tile_size * (self.camera_size / new_screen_size);
        self.tile_size = self.target_tile_size;
        self.camera_size = new_screen_size;
    }

    pub fn look_at_tile(&mut self, coordinates: Vector2) {
        self.coordinates = coordinates;
    }

    pub fn absolute_screen_position_to_tile_coordinates(
        &self,
        screen_position: Vector2,
    ) -> Vector2 {
        let x =
            screen_position.x / self.tile_size.x * 2.0 + screen_position.y / self.tile_size.y * 4.0;
        let y = -screen_position.x / self.tile_size.x * 2.0
            + screen_position.y / self.tile_size.y * 4.0;
        Vector2::new(x, y)
    }
    pub fn relative_screen_position_to_tile_coordinates(
        &self,
        screen_position: Vector2,
    ) -> Vector2 {
        let x = screen_position.x / self.tile_size.x + screen_position.y * 2.0 / self.tile_size.y;
        let y = -screen_position.x / self.tile_size.x + screen_position.y * 2.0 / self.tile_size.y;
        Vector2::new(x, y) + self.coordinates
    }

    pub fn tile_coordinates_to_screen_position(&self, coordinates: Vector2) -> Vector2 {
        let relative_coordinates = coordinates - self.coordinates; //Vector pointing from camera to tile
        let x = (relative_coordinates.x - relative_coordinates.y) * self.tile_size.x / 2.0;
        let y = (relative_coordinates.x + relative_coordinates.y) * self.tile_size.y / 4.0;
        self.coordinates - Vector2::new(x, y) / self.camera_size
    }

    pub fn absolute_screen_position_to_relative_screen_position(
        &self,
        screen_position: Vector2,
    ) -> Vector2 {
        screen_position / self.camera_size
    }
}