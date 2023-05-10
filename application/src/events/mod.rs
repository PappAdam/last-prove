use winit::event::{ElementState, MouseButton, VirtualKeyCode};

use self::input::Input;

pub mod input;
pub mod mouse;

impl Input {
    #[inline]
    pub fn get_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys[key as usize] == ElementState::Pressed
    }

    #[inline]
    pub fn on_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe {
            self.mouse.buttons[*(&button as *const _ as *const usize)] == ElementState::Pressed
        }
    }
}
