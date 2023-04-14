use winit::event::{ElementState, VirtualKeyCode};

use self::input::Input;

pub mod input;
pub mod mouse;

impl Input {
    #[inline]
    pub fn on_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys[key as usize] == ElementState::Pressed
    }
}
