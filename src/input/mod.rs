use crate::{engine::vector2::Vector2};
use std::collections::HashMap;
use winit::event::{ElementState, KeyboardInput, MouseButton, MouseScrollDelta, VirtualKeyCode};

#[derive(Debug)]
pub enum Keystate {
    Up,
    Down,
    Pressed,
    Released,
}

pub struct Input {
    mouse_wheel: i8,            //Can be -1, 0, or 1.
    mouse_position: Vector2<f32>,    //Relative screen position range is from -1 to 1
    mouse_movement: Vector2<f32>,    //Mouse movement means the position between last and current frame.
    mousebuttons: HashMap<MouseButton, Keystate>,
    buttons: HashMap<VirtualKeyCode, Keystate>,
    mouse_stationary: f32, //Amount of time indicating for how long hasn't the mouse been moved in seconds
    pub keys_pressed_this_frame: Vec<VirtualKeyCode>, //This vector clears itself every frame
}

impl Input {
    pub fn init() -> Self {
        let mouse_wheel = 0;
        let mouse_position = Vector2::zero();
        let mouse_movement = Vector2::default();
        let mousebuttons = HashMap::new();
        let buttons = HashMap::new();
        Self {
            mouse_wheel,
            mouse_position,
            mouse_movement,
            mousebuttons,
            buttons,
            keys_pressed_this_frame: vec![],
            mouse_stationary: 0f32,
        }
    }

    pub fn on_key_input(&mut self, keaboard_input: KeyboardInput) {
        match keaboard_input.virtual_keycode {
            Some(key_code) => match keaboard_input.state {
                ElementState::Pressed => {
                    self.buttons.insert(key_code, Keystate::Pressed);
                    self.keys_pressed_this_frame.push(key_code);
                }
                ElementState::Released => {
                    self.buttons.insert(key_code, Keystate::Released);
                }
            },
            None => {}
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
            MouseScrollDelta::PixelDelta(_) => {}
        }
    }
    pub fn on_mouse_moved(&mut self, new_mouse_position: Vector2<f32>, camera_size: Vector2<u16>) {
        let relative_new_mouse_position =
            new_mouse_position / (camera_size.into::<f32>() / 2.0) - Vector2::new(1u8, 1);
        self.mouse_movement += relative_new_mouse_position - self.mouse_position;
        self.mouse_position = relative_new_mouse_position;
    }

    pub fn refresh_input(&mut self, delta_time: f32) {
        //Setting every pressed button to down, every released button to up
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

        //Mouse stationary calculations
        if self.mouse_movement == Vector2::zero() {
            self.mouse_stationary += delta_time
        }
        else {
            self.mouse_stationary = 0f32;
        }

        //Resetting values that are representing single frame actions
        self.mouse_wheel = 0;
        self.mouse_movement.x = 0.0;
        self.mouse_movement.y = 0.0;
        self.keys_pressed_this_frame.clear();
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

    pub fn get_mouse_position(&self) -> Vector2<f32> {
        self.mouse_position
    }
    pub fn get_rel_mouse_position(&self, window_size: (u16, u16)) -> (f32, f32) {
        let mouse_position = self.get_mouse_position();
        (
            (mouse_position.x as f32) / (window_size.0 as f32 - 1.0),
            (mouse_position.y as f32) / (window_size.1 as f32 - 1.0),
        )
    }
    pub fn get_mouse_movement(&self) -> Vector2<f32> {
        self.mouse_movement
    }
    pub fn get_mouse_wheel(&self) -> i8 {
        self.mouse_wheel
    }
}
