use std::mem::size_of_val;

use crate::{
    camera::hud::{HudActionOnClick, HudReference},
    map::objects::GameObjectReference,
};

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

            //Clicked a hud object
            if let Some(hud_object) = self
                .camera
                .get_hud_object_at_screen_position(mouse_position)
            {
                match hud_object.reference {
                    HudReference::Building(index) => match hud_object.action_on_click {
                        HudActionOnClick::Create => {
                            if self.map.try_troop_spawn(index) {
                                self.copy_into_troop_buffer();
                            }
                        }
                        HudActionOnClick::Destroy => {
                            self.map.destroy_building(index);
                            self.copy_into_building_buffer();
                            self.camera.close_hud_related_to(HudReference::Building(0))
                        }
                        _ => {}
                    },
                    //Troop HUD
                    HudReference::Troop(index) => match hud_object.action_on_click {
                        HudActionOnClick::Destroy => {
                            self.map.destroy_troop(index);
                            self.copy_into_troop_buffer();
                            self.camera.close_hud_related_to(HudReference::Troop(0))
                        }
                        _ => {}
                    },
                    _ => {}
                }
                self.copy_into_hud_buffer();
                return;
            }
            //Clicked a gameobject
            match self.map.get_shown_object_at_coordinates(
                self.camera
                    .screen_position_to_tile_coordinates(mouse_position),
            ) {
                GameObjectReference::Tile(clicked_tile) => {
                    if !clicked_tile.is_object_on_top() {
                        self.map.build_building(clicked_tile.coordinates.into(), 0);
                        self.copy_into_building_buffer();
                    }
                }
                GameObjectReference::Troop(clicked_troop) => {
                    let troop_index = {
                        (clicked_troop as *const _ as usize
                            - (self.map.troop_vector.content.as_ptr() as usize))
                            / size_of_val(clicked_troop)
                    };
                    self.camera
                        .open_hud_related_to(HudReference::Troop(troop_index));
                    self.copy_into_hud_buffer();
                }
                GameObjectReference::Building(clicked_building) => {
                    let building_index = {
                        (clicked_building as *const _ as usize
                            - (self.map.building_vector.content.as_ptr() as usize))
                            / size_of_val(clicked_building)
                    };

                    self.camera
                        .open_hud_related_to(HudReference::Building(building_index));
                    self.copy_into_hud_buffer();
                }
                GameObjectReference::None => {},
            }
        }
        if &self.input.keys_pressed_this_frame.len() != &0usize {
            for key_pressed in &self.input.keys_pressed_this_frame {
                self.camera.refresh_hud_on_key_press(*key_pressed);
            }
            self.copy_into_hud_buffer();
        }
    }
}
