use crate::{camera::hud::HudReference, engine::vector2::Vector2, map::tile::TileFlag};

use super::VulkanApp;

impl VulkanApp {
    pub fn refresh_game(&mut self, delta_time: f32) {
        self.process_input_commands();

        self.camera.refresh_camera(&self.input, delta_time);
        self.input.refresh_input(delta_time);
    }

    fn process_input_commands(&mut self) {
        if self
            .input
            .get_mousebutton_pressed(winit::event::MouseButton::Left)
        {
            let mouse_position = self.input.get_mouse_position();
            let mouse_coordinates = self
                .camera
                .screen_position_to_tile_coordinates(mouse_position);

            //Clicked a hud object
            if let Some(hud_object) = self
                .camera
                .get_hud_object_at_screen_position(mouse_position)
            {
                match hud_object.reference {
                    HudReference::Building(index) => {
                        let building = &self.map.building_vector[index];
                        let troop_coordinates = building.facing() + building.coordinates.into();
                        if let Some(tile_to_spawn_on) = self.map.get_tile_from_matr(troop_coordinates) {
                            if !tile_to_spawn_on.is_occupied() {
                                self.map.spawn_troop(troop_coordinates);
                                self.copy_into_troop_buffer();
                            }
                        }
                    }
                    _ => {}
                }
                return;
            }
            //Clicked a tile
            if let Some(clicked_tile) = self.map.get_shown_tile_at_coordinates(mouse_coordinates) {
                //No object on top
                if !clicked_tile.is_occupied() {
                    self.map.build_building(clicked_tile.coordinates.into(), 0);
                    self.copy_into_building_buffer();
                } else {
                    //Has building on top
                    if clicked_tile.is_building_on_top() {
                        self.camera.open_hud_related_to(HudReference::Building(
                            clicked_tile.object_on_top_index_in_vector as usize,
                        ))
                    }
                    //Has troop on top
                    else if clicked_tile.is_troop_on_top() {
                        self.camera.open_hud_related_to(HudReference::Troop(
                            clicked_tile.object_on_top_index_in_vector as usize,
                        ))
                    }
                }
            }
        }
        for key_pressed in &self.input.keys_pressed_this_frame {
            self.camera.refresh_hud_on_key_press(*key_pressed);
        }
        self.copy_into_hud_buffer();
    }
}
