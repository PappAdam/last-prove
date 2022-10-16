use std::collections::HashMap;

use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

#[derive(Debug)]
pub enum Keystate {
    Up,
    Down,
    Pressed,
    Released,
}

pub struct Input {
    mouse_wheel: Option<i8>,
    mouse_position: (u16, u16),
    mousebuttons: HashMap<MouseButton, Keystate>,
    buttons: HashMap<Keycode, Keystate>,
}
#[allow(dead_code)]
impl Input {
    pub fn init() -> Self {
        let mouse_wheel = None;
        let mouse_position = (0, 0);
        let mousebuttons = HashMap::new();
        let buttons = HashMap::new();
        Self {
            mouse_wheel,
            mouse_position,
            mousebuttons,
            buttons,
        }
    }

    pub fn on_key_pressed(&mut self, keycode: Option<Keycode>) {
        match keycode {
            None => {}
            Some(keycode) => {
                self.buttons.insert(keycode, Keystate::Pressed);
            }
        }
    }
    pub fn on_key_released(&mut self, keycode: Option<Keycode>) {
        match keycode {
            None => {}
            Some(keycode) => {
                self.buttons.insert(keycode, Keystate::Released);
            }
        }
    }
    pub fn on_mousebutton_pressed(&mut self, mouse_btn: MouseButton) {
        match mouse_btn {
            MouseButton::Unknown => {}
            _ => {
                self.mousebuttons.insert(mouse_btn, Keystate::Pressed);
            }
        }
    }
    pub fn on_mousebutton_released(&mut self, mouse_btn: MouseButton) {
        match mouse_btn {
            MouseButton::Unknown => {}
            _ => {
                self.mousebuttons.insert(mouse_btn, Keystate::Released);
            }
        }
    }
    pub fn on_mousewheel_scrolled(&mut self, y: i8) {
        self.mouse_wheel = Some(y);
    }
    pub fn on_mouse_moved(&mut self, mouse_position: (u16, u16)) {
        self.mouse_position = mouse_position;
    }

    pub fn refresh_input(&mut self) {
        for key in self.buttons.iter_mut() {
            match *key.1 {
                Keystate::Pressed => *key.1 = Keystate::Down,
                Keystate::Released => *key.1 = Keystate::Up,
                _ => {}
            }
        }
        for mouse_btn in self.mousebuttons.iter_mut() {
            match *mouse_btn.1 {
                Keystate::Pressed => *mouse_btn.1 = Keystate::Down,
                Keystate::Released => *mouse_btn.1 = Keystate::Up,
                _ => {}
            }
        }
        self.mouse_wheel = None;
    }

    fn get_key_state(&self, keycode: Keycode) -> &Keystate {
        match self.buttons.get(&keycode) {
            Some(keycode) => keycode,
            None => &Keystate::Up,
        }
    }
    //3 functions for getting specific key states, use !get_key_down for key up.
    pub fn get_key_down(&self, keycode: Keycode) -> bool {
        match self.get_key_state(keycode) {
            Keystate::Down | Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_key_pressed(&self, keycode: Keycode) -> bool {
        match self.get_key_state(keycode) {
            Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_key_released(&self, keycode: Keycode) -> bool {
        match self.get_key_state(keycode) {
            Keystate::Released => true,
            _ => false,
        }
    }

    fn get_mousebutton_state(&self, mouse_btn: MouseButton) -> &Keystate {
        match self.mousebuttons.get(&mouse_btn) {
            Some(mouse_btn) => mouse_btn,
            None => &Keystate::Up,
        }
    }
    //3 functions for getting specific mousebutton states, use !get_mousebutton_down for mousebutton up.
    pub fn get_mousebutton_down(&self, mouse_btn: MouseButton) -> bool {
        match self.get_mousebutton_state(mouse_btn) {
            Keystate::Down | Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_mousebutton_pressed(&self, mouse_btn: MouseButton) -> bool {
        match self.get_mousebutton_state(mouse_btn) {
            Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_mousebutton_released(&self, mouse_btn: MouseButton) -> bool {
        match self.get_mousebutton_state(mouse_btn) {
            Keystate::Released => true,
            _ => false,
        }
    }

    pub fn get_mouse_position(&self) -> (u16, u16) {
        self.mouse_position
    }
    pub fn get_mouse_wheel(&self) -> Option<i8> {
        self.mouse_wheel
    }

    pub fn print_pressed_buttons(&self) {
        for key in &self.buttons {
            match key.1 {
                Keystate::Down => {
                    println!("{} DOWN", key.0)
                }
                Keystate::Up => {
                    println!("{} UP", key.0)
                }
                Keystate::Pressed => {
                    println!("{} PRESSED", key.0)
                }
                Keystate::Released => {
                    println!("{} RELEASED", key.0)
                }
            }
        }
    }
}
