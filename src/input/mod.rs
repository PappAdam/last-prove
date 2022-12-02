use crate::engine::vector2::Vector2;
use std::collections::HashMap;
use winit::event::{MouseButton, VirtualKeyCode, KeyboardInput, ElementState, MouseScrollDelta};

#[derive(Debug)]
pub enum Keystate {
    Up,
    Down,
    Pressed,
    Released,
}

pub struct Input {
    mouse_wheel: i8,
    mouse_position: Vector2,
    mouse_movement: Vector2, //Mouse movement means the position between last and current frame.
    mousebuttons: HashMap<MouseButton, Keystate>,
    buttons: HashMap<VirtualKeyCode, Keystate>,
}
#[allow(dead_code)]
impl Input {
    pub fn init(window_size: (u16, u16)) -> Self {
        let mouse_wheel = 0;
        let mouse_position = Vector2::new((window_size.0 / 2) as f32, (window_size.0 / 2) as f32);
        let mouse_movement = Vector2::default();
        let mousebuttons = HashMap::new();
        let buttons = HashMap::new();
        Self {
            mouse_wheel,
            mouse_position,
            mouse_movement,
            mousebuttons,
            buttons,
        }
    }

    pub fn on_key_input(&mut self, input: KeyboardInput) {
        match input.state {
            ElementState::Pressed => {
                self.buttons.insert(input.virtual_keycode.unwrap(), Keystate::Pressed);
            }
            ElementState::Released => {
                self.buttons.insert(input.virtual_keycode.unwrap(), Keystate::Released);
            }
        }
    }
    pub fn on_mousebutton_input(&mut self, mouse_btn: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => self.mousebuttons.insert(mouse_btn, Keystate::Pressed),
            ElementState::Released => self.mousebuttons.insert(mouse_btn, Keystate::Released),
        };
    }

    pub fn on_mousewheel_scrolled(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, y) => self.mouse_wheel = y as i8,
            MouseScrollDelta::PixelDelta(_) => {  },
        }
    }
    pub fn on_mouse_moved(&mut self, mouse_position: Vector2) {
        self.mouse_movement = mouse_position - self.mouse_position;
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
        self.mouse_wheel = 0;
        self.mouse_movement.x = 0.0;
        self.mouse_movement.y = 0.0;
    }

    //Just functions to get input values.
    fn get_key_state(&self, keycode: VirtualKeyCode) -> &Keystate {
        match self.buttons.get(&keycode) {
            Some(keycode) => keycode,
            None => &Keystate::Up,
        }
    }
    //3 functions for getting specific key states, use !get_key_down for key up.
    pub fn get_key_down(&self, keycode: VirtualKeyCode) -> bool {
        match self.get_key_state(keycode) {
            Keystate::Down | Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_key_pressed(&self, keycode: VirtualKeyCode) -> bool {
        match self.get_key_state(keycode) {
            Keystate::Pressed => true,
            _ => false,
        }
    }
    pub fn get_key_released(&self, keycode: VirtualKeyCode) -> bool {
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

    pub fn get_mouse_position(&self) -> Vector2 {
        self.mouse_position
    }
    pub fn get_rel_mouse_position(&self, window_size: (u16, u16)) -> (f32, f32) {
        let mouse_position = self.get_mouse_position();
        (
            (mouse_position.x as f32) / (window_size.0 as f32 - 1.0),
            (mouse_position.y as f32) / (window_size.1 as f32 - 1.0),
        )
    }
    pub fn get_mouse_movement(&self) -> Vector2 {
        self.mouse_movement
    }
    pub fn get_mouse_wheel(&self) -> i8 {
        self.mouse_wheel
    }

    pub fn print_pressed_buttons(&self) {
        for key in &self.buttons {
            match key.1 {
                Keystate::Down => {
                    println!("{:?} DOWN", key.0)
                }
                Keystate::Up => {
                    println!("{:?} UP", key.0)
                }
                Keystate::Pressed => {
                    println!("{:?} PRESSED", key.0)
                }
                Keystate::Released => {
                    println!("{:?} RELEASED", key.0)
                }
            }
        }
    }
}
