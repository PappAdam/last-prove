use super::Camera;
use crate::engine::vector2::Vector2;
use std::vec;
use winit::event::VirtualKeyCode;

pub fn create_hud_elements() -> Vec<HudObject> {
    let building_hud = HudObject::new_basic(
        Vector2::new(0.7, -0.7),
        Vector2::new(1.0, 0.7),
        0,
        HudReference::Building(0),
        HudActionOnClick::None,
    );
    let mut troop_hud = HudObject::new_basic(
        Vector2::new(-1.0, -0.7),
        Vector2::new(-0.7, 0.7),
        0,
        HudReference::Troop(0),
        HudActionOnClick::None,
    );

    troop_hud.add_child_object(HudObject::new_basic(Vector2::new(-0.85, -0.65), Vector2::new(-0.75, -0.55), 1, HudReference::Troop(0), HudActionOnClick::Create));

    vec![
        HudObject::new_static(Vector2::new(-0.55, -1.0), Vector2::new(0.55, -0.80), 0),
        building_hud,
        troop_hud,
    ]
}

enum HudTexture {
    Background,
    Create,
    Destroy
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HudReference {
    None,
    //Tile((u16, u16)),  //Coordinates
    Building(usize), //Index
    Troop(usize),    //Index
}

#[derive(Debug)]
pub enum HudActionOnClick {
    None,
    Create,
    Destroy,
}

pub enum HudFlag {
    Shown = 0b10000000,
}

#[derive(Debug, PartialEq)]
enum HudType {
    Static, //Object is always shown.
    Basic,  //Object visibility is toggled only through script
    Toggleable(VirtualKeyCode), //Object visibility is toggled by a key
            //Temporary,                  //Object is shown until next action
            //ShownByHover,               //Object is shown when mouse is hovered over an element
}

#[derive(Debug)]
pub struct HudObject {
    pub top_left: Vector2,     //Both are stored
    pub bottom_right: Vector2, //in relative screen position
    pub texture_layer: u8,
    pub z_layer: u8,           //Higher is closer to camera.
    hud_type: HudType,
    pub reference: HudReference,
    action_on_click: HudActionOnClick,
    pub flags: u8, //0: Shown (0 if not shown)
    //1: NOT SET
    //2: NOT SET
    //3: NOT SET
    //4: NOT SET
    //5: NOT SET
    //6: NOT SET
    //7: NOT SET
    pub child_huds: Vec<Self>,
}

impl HudObject {
    pub fn new_static(top_left: Vector2, bottom_right: Vector2, texture_layer: u8) -> Self {
        HudObject {
            top_left,
            bottom_right,
            texture_layer,
            z_layer: 0,
            hud_type: HudType::Static,
            reference: HudReference::None,
            action_on_click: HudActionOnClick::None,
            flags: HudFlag::Shown as u8,
            child_huds: vec![],
        }
    }
    pub fn new_basic(
        top_left: Vector2,
        bottom_right: Vector2,
        texture_layer: u8,
        reference: HudReference,
        action_on_click: HudActionOnClick,
    ) -> Self {
        HudObject {
            top_left,
            bottom_right,
            texture_layer,
            z_layer: 0,
            hud_type: HudType::Basic,
            reference,
            action_on_click,
            flags: 0,
            child_huds: vec![],
        }
    }
    pub fn new_toggleable_by_key(
        top_left: Vector2,
        bottom_right: Vector2,
        texture_layer: u8,
        toggle_key: VirtualKeyCode,
        reference: HudReference,
        action_on_click: HudActionOnClick,
    ) -> Self {
        HudObject {
            top_left,
            bottom_right,
            texture_layer,
            z_layer: 0,
            hud_type: HudType::Toggleable(toggle_key),
            reference,
            action_on_click,
            flags: HudFlag::Shown as u8,
            child_huds: vec![],
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
        self.flags &= !(HudFlag::Shown as u8);
        for child_hud_objects in &mut self.child_huds {
            child_hud_objects.hide()
        }
    }
    pub fn show(&mut self) {
        self.flags |= HudFlag::Shown as u8;
        for child_hud_objects in &mut self.child_huds {
            child_hud_objects.hide()
        }
    }

    pub fn toggle_visibility(&mut self) {
        if self.is_shown() {
            self.hide()
        } else {
            self.show()
        }
    }

    fn add_child_object(&mut self, object: Self) {
        self.child_huds.push(object);
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
                HudType::Basic => {
                    if key_pressed == VirtualKeyCode::Escape {
                        hud_object.hide()
                    }
                }
                _ => {}
            }
        }
    }

    pub fn open_hud_related_to(&mut self, reference: HudReference) {
        //Closing all previous huds.
        for hud_object in &mut self.hud_objects {
            if hud_object.hud_type != HudType::Static {
                hud_object.hide()
            }
        }

        //Opening corresponding hud objects to the reference.
        for hud_object in &mut self.hud_objects {
            //Opens building tab
            match hud_object.reference {
                HudReference::Building(_) => {
                    if let HudReference::Building(_) = &reference {
                        hud_object.show();
                        hud_object.reference = reference.clone();
                    }
                }
                HudReference::Troop(_) => {
                    if let HudReference::Troop(_) = &reference {
                        hud_object.show();
                        hud_object.reference = reference.clone();
                    }
                }
                _ => {}
            }
        }
    }
}