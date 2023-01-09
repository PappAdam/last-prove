use std::vec;

use winit::event::VirtualKeyCode;

use crate::{engine::vector2::Vector2};
use crate::vulkanapp::gpustoredinstances::GpuStoredHUDObject;

use super::Camera;

pub fn create_hud_elements() -> Vec<HudObject> {
    vec![
        HudObject::new_static(Vector2::new(-0.55, -1.0), Vector2::new(0.55, -0.80)),
        // HudObject::new_toggleable_by_key(
        //     Vector2::new(-1.0, -0.7),
        //     Vector2::new(-0.7, 0.7),
        //     VirtualKeyCode::A,
        // ),
        HudObject::new_toggleable_by_key(
            Vector2::new(0.7, -0.7),
            Vector2::new(1.0, 0.7),
            VirtualKeyCode::E,
        ),
    ]
}

pub enum HudFlag {
    Shown = 0b10000000,
}

#[derive(Debug)]
enum HudType {
    Static,                     //Object that are always shown
    Toggleable(VirtualKeyCode), //Object visibility is toggled by a key
    Temporary,                  //Object is shown until next action
    ShownByHover,               //Object is shown when mouse is hovered over an element
}

#[derive(Debug)]
pub struct HudObject {
    pub top_left: Vector2,     //Both are stored
    pub bottom_right: Vector2, //in relative screen position
    pub z_layer: u8,           //Higher is closer to camera.
    hud_type: HudType,
    pub flags: u8, //0: Shown (0 if not shown)
                   //1: NOT SET
                   //2: NOT SET
                   //3: NOT SET
                   //4: NOT SET
                   //5: NOT SET
                   //6: NOT SET
                   //7: NOT SET
}

impl HudObject {
    pub fn new_static(top_left: Vector2, bottom_right: Vector2) -> Self {
        HudObject {
            top_left,
            bottom_right,
            z_layer: 0,
            hud_type: HudType::Static,
            flags: HudFlag::Shown as u8,
        }
    }
    pub fn new_toggleable_by_key(
        top_left: Vector2,
        bottom_right: Vector2,
        toggle_key: VirtualKeyCode,
    ) -> Self {
        HudObject {
            top_left,
            bottom_right,
            z_layer: 0,
            hud_type: HudType::Toggleable(toggle_key),
            flags: HudFlag::Shown as u8,
        }
    }

    pub fn screen_position_inside_hud(&self, click_position: Vector2) -> bool {
        if click_position.x > self.top_left.x
            && click_position.y > self.top_left.y
            && click_position.x < self.bottom_right.x
            && click_position.y < self.bottom_right.y
            && self.is_shown()
        {
            return true;
        }
        false
    }

    pub fn is_shown(&self) -> bool {
        self.flags & HudFlag::Shown as u8 == HudFlag::Shown as u8
    }
    pub fn hide(&mut self) {
        self.flags &= !(HudFlag::Shown as u8)
    }
    pub fn show(&mut self) {
        self.flags |= HudFlag::Shown as u8
    }

    pub fn toggle_visibility(&mut self) {
        if self.is_shown() {
            self.hide()
        } else {
            self.show()
        }
    }
}

impl Camera {
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
    
    pub fn refresh_hud_on_key_press(&mut self, key_pressed: VirtualKeyCode) {
        println!("{:?}", key_pressed);
        for hud_object in &mut self.hud_objects {
            match hud_object.hud_type {
                HudType::Toggleable(hud_toggle_button) => {
                    if key_pressed == hud_toggle_button {
                        hud_object.toggle_visibility()
                    } else if key_pressed == VirtualKeyCode::Escape {
                        hud_object.hide()
                    }
                }
                _ => {}
            }
        }
    }
}
