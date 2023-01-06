use std::vec;
mod hud;

use crate::{
    engine::vector2::Vector2,
    gpustoredinstances::GpuStoredHUDObject,
    input::Input,
};
use hud::HudObject;

use self::hud::HudFlag;

pub struct Camera {
    pub coordinates: Vector2, //The cameras coordinates are the coordinates of the tile in the center
    pub target_coordinates: Vector2,
    pub tiles_fit: Vector2,
    pub target_tiles_fit: Vector2,
    pub camera_size: Vector2,
    hud_objects: Vec<HudObject>,
}

impl Camera {
    pub fn new(camera_size: Vector2) -> Self {
        let tiles_fit = camera_size / Vector2::new(64u8, 64);
        let hud_objects = hud::create_hud_elements();

        Self {
            coordinates: Vector2::zero(),
            target_coordinates: Vector2::zero(),
            tiles_fit,
            target_tiles_fit: tiles_fit,
            camera_size,
            hud_objects,
        }
    }

    pub fn refresh_camera(&mut self, input: &Input, delta_time: f32) {
        self.tiles_fit = Vector2::lerp(self.tiles_fit, self.target_tiles_fit, 60.0 * delta_time);
        self.coordinates =
            Vector2::lerp(self.coordinates, self.target_coordinates, 60.0 * delta_time);

        let mouse_wheel = input.get_mouse_wheel();
        if mouse_wheel > 0 && self.target_tiles_fit.x > 2.0 && self.target_tiles_fit.x > 2.0 {
            self.target_tiles_fit /= 1.2;
            self.target_coordinates = Vector2::lerp(
                self.target_coordinates,
                self.screen_position_to_tile_coordinates(input.get_mouse_position()),
                1.0 - (1.0 / 1.2),
            );
        } else if mouse_wheel < 0
            && self.target_tiles_fit.x < 100.0
            && self.target_tiles_fit.x < 100.0
        {
            self.target_tiles_fit *= 1.2;
            self.target_coordinates = Vector2::lerp(
                self.target_coordinates,
                self.screen_position_to_tile_coordinates(-input.get_mouse_position()),
                0.2,
            );
        }

        if input.get_mousebutton_down(winit::event::MouseButton::Middle) {
            self.target_coordinates =
                self.screen_position_to_tile_coordinates(-input.get_mouse_movement());
            self.coordinates = self.target_coordinates
        }
    }

    pub fn window_resized(&mut self, new_screen_size: Vector2) {
        self.target_tiles_fit = self.target_tiles_fit / (self.camera_size / new_screen_size);
        self.tiles_fit = self.target_tiles_fit;
        self.camera_size = new_screen_size;
    }

    pub fn ease_to_tile(&mut self, coordinates: Vector2) {
        self.target_coordinates = coordinates;
    }

    pub fn snap_to_tile(&mut self, coordinates: Vector2) {
        self.target_coordinates = coordinates;
        self.coordinates = coordinates;
    }

    pub fn screen_position_to_tile_coordinates(&self, screen_position: Vector2) -> Vector2 {
        let x = screen_position.x / (2.0 / self.tiles_fit.x)
            + screen_position.y * 2.0 / (2.0 / self.tiles_fit.y);
        let y = -screen_position.x / (2.0 / self.tiles_fit.x)
            + screen_position.y * 2.0 / (2.0 / self.tiles_fit.y);
        Vector2::new(x, y) + self.coordinates
    }

    pub fn get_hud_object_at_screen_position(
        &self,
        screen_position: Vector2,
    ) -> Option<&HudObject> {
        for hud_object in &self.hud_objects {
            if hud_object.screen_position_inside_hud(screen_position) {
                return Some(hud_object);
            }
        }
        None
    }

    pub fn get_hud_instance_coordinates(&self) -> Vec<GpuStoredHUDObject> {
        let mut gpu_stored_hud_objects =
            vec::from_elem(GpuStoredHUDObject::zero(), self.hud_objects.len());
        for (vector_index, hud_object) in self.hud_objects.iter().enumerate() {
            if hud_object.is_shown() {
                gpu_stored_hud_objects[vector_index] = GpuStoredHUDObject {
                    screen_position: [
                        hud_object.top_left.x,
                        hud_object.top_left.y,
                        hud_object.z_layer as f32,
                    ],
                    object_size: (hud_object.bottom_right - hud_object.top_left).into(),
                    texture_layer: 0,
                }
            }
        }
        gpu_stored_hud_objects
    }

    //pub fn tile_coordinates_to_screen_position(&self, coordinates: Vector2) -> Vector2 {
    //    let relative_coordinates = coordinates - self.coordinates; //Vector pointing from camera to tile
    //    let x = (relative_coordinates.x - relative_coordinates.y) * self.tile_size.x / 2.0;
    //    let y = (relative_coordinates.x + relative_coordinates.y) * self.tile_size.y / 4.0;
    //    self.coordinates - Vector2::new(x, y) / self.camera_size
    //}
}