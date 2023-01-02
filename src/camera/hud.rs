use crate::engine::vector2::Vector2;

pub fn create_hud_elements() -> Vec<HudObject> {
    vec![
        HudObject::new(Vector2::new(0.7, -0.9), Vector2::uniform(0.9)),
    ]
}

pub enum HudFlag {
    Shown = 0b10000000,
}

#[derive(Debug)]
pub struct HudObject {
    pub top_left: Vector2,     //Both are stored
    pub bottom_right: Vector2, //in relative screen position
    pub z_layer: u8,           //Higher is closer to camera.
    //action_on_click:
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
    pub fn new(top_left: Vector2, bottom_right: Vector2) -> Self {
        HudObject {
            top_left,
            bottom_right,
            z_layer: 0,
            flags: HudFlag::Shown as u8,
        }
    }
    pub fn screen_position_inside_hud(&self, click_position: Vector2) -> bool {
        if click_position.x > self.top_left.x
            && click_position.y > self.top_left.y
            && click_position.x < self.bottom_right.x
            && click_position.y < self.bottom_right.y
        {
            return true;
        }
        false
    }
    pub fn is_shown(&self) -> bool {
        self.flags & HudFlag::Shown as u8 == HudFlag::Shown as u8
    }
}
