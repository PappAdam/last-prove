use nalgebra::Vector2;
use winit::event::{ElementState, MouseButton};

pub struct Mouse {
    pub buttons: [ElementState; 3],
    pub pos: Vector2<f32>,
    pub delta_move: Vector2<f32>,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            buttons: [ElementState::Released; 3],
            pos: Default::default(),
            delta_move: Default::default(),
        }
    }

    #[inline]
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.delta_move.x = x as f32 - self.pos.x;
        self.delta_move.y = y as f32 - self.pos.y;
        self.pos.x = x as f32;
        self.pos.y = y as f32;
    }

    #[inline]
    pub fn set_button(&mut self, button: MouseButton, state: ElementState) {
        unsafe {
            self.buttons[*(&button as *const _ as *const u8) as usize] = state;
        }
    }
}
