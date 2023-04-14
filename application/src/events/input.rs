use winit::event::{ElementState, ModifiersState, VirtualKeyCode};

use super::mouse::Mouse;

pub struct Input {
    pub keys: [ElementState; 163],
    pub modifier: ModifiersState,
    pub mouse: Mouse,
}

impl Input {
    pub fn init() -> Self {
        let keys = [ElementState::Released; 163];
        Self {
            keys,
            modifier: Default::default(),
            mouse: Mouse::new(),
        }
    }

    #[inline]
    pub fn set_modif(&mut self, modif: ModifiersState) {
        self.modifier = modif
    }

    #[inline]
    pub fn handle_key_press(&mut self, key: Option<VirtualKeyCode>, state: ElementState) {
        if let Some(key) = key {
            self.keys[key as usize] = state
        }
    }

    #[inline]
    pub fn refresh(&mut self) {
        self.mouse.delta_move.x = 0.;
        self.mouse.delta_move.y = 0.;
    }
}
