pub mod mouse;

use std::collections::HashMap;

use sdl2::keyboard::Keycode;
use {mouse::Mouse};

pub enum Keystate {
    Up,
    Down,
    Pressed,
    Released,
}
impl std::fmt::Display for Keystate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res;
        match self {
            Keystate::Down => res = write!(f, "Down"),
            Keystate::Up => res = write!(f, "Up"),
            Keystate::Pressed => res = write!(f, "Pressed"),
            Keystate::Released => res = write!(f, "Released"),
        }
        res
    }
}

pub struct Input {
    pub mouse: Mouse,

    pub buttons: HashMap<Keycode, Keystate>,
}
impl Input {
    pub fn init() -> Self {
        let mouse = Mouse::init();
        let buttons = HashMap::new();
        Self{ mouse, buttons }
    }
    pub fn on_key_pressed(&mut self, keycode: Option<Keycode>) {
        match keycode {
            None => {  },
            Some(keycode) => {
                self.buttons.insert(keycode, Keystate::Pressed);
            }
        }
    }
    pub fn on_key_released(&mut self, keycode: Option<Keycode>) {
        match keycode {
            None => {  },
            Some(keycode) => {
                self.buttons.insert(keycode, Keystate::Released);
            }
        }
    }
    pub fn refresh_input(&mut self) {
        for key in self.buttons.iter_mut() {
            match *key.1 {
                Keystate::Pressed => *key.1 = Keystate::Down,
                Keystate::Released => *key.1 = Keystate::Up,
                _ => {  }
            }
        }
    }

    pub fn get_key_state(&mut self, keycode: Keycode) -> &Keystate {
        match self.buttons.get(&keycode) {
            Some(keycode) => keycode,
            None => { &Keystate::Up },
        }
    }
    //3 functions for getting specific key states, use !get_key_down for key up.
    pub fn get_key_down(&mut self, keycode: Keycode) -> bool {
        match self.buttons.get(&keycode) {
            Some(keycode) => {
                match keycode {
                    Keystate::Down | Keystate::Pressed => return true,
                    _ => return false,
            }},
            None => return false,
        }
    }
    pub fn get_key_pressed(&mut self, keycode: Keycode) -> bool {
        match self.buttons.get(&keycode) {
            Some(keycode) => {
                match keycode {
                    Keystate::Pressed => return true,
                    _ => return false,
            }},
            None => return false,
        }
    }
    pub fn get_key_released(&mut self, keycode: Keycode) -> bool {
        match self.buttons.get(&keycode) {
            Some(keycode) => {
                match keycode {
                    Keystate::Released => return true,
                    _ => return false,
            }},
            None => return false,
        }
    }


    #[allow(dead_code)]
    pub fn print_pressed_buttons(&self) {
        for key in &self.buttons {
            match key.1 {
                Keystate::Down => { println!("{} DOWN", key.0) },
                Keystate::Up => { println!("{} UP", key.0) }
                Keystate::Pressed => { println!("{} PRESSED", key.0) }
                Keystate::Released => { println!("{} RELEASED", key.0) }
            }
        }
    }
}