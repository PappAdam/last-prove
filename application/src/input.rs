use nalgebra::Vector2;
use winit::event::{ElementState, ModifiersState, MouseButton, VirtualKeyCode, Event};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Input {
    pub keys: [EventState; 163],
    pub modifier: ModifiersState,
    pub mouse: Mouse,
    last_modified_keys: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventState {
    Pressed,
    Released,
    Up,
    Down,
}

impl Input {
    pub fn init() -> Self {
        let keys = [EventState::Released; 163];
        Self {
            keys,
            modifier: Default::default(),
            last_modified_keys: Vec::default(),
            mouse: Mouse {
                buttons: [EventState::Released; 3],
                pos: Default::default(),
                delta_move: Default::default(),
                wheel: 0.,
            },
        }
    }

    #[inline]
    pub fn set_modif(&mut self, modif: ModifiersState) {
        self.modifier = modif
    }

    #[inline]
    pub fn handle_key_press(&mut self, key: Option<VirtualKeyCode>, state: ElementState) {
        if let Some(key) = key {
            match state {
                ElementState::Pressed => {
                    self.keys[key as usize] = EventState::Pressed;
                },
                ElementState::Released => {
                    self.keys[key as usize] = EventState::Released;
                },
            }
            self.last_modified_keys.push(key as u8)
        }
    }

    #[inline]
    pub fn handle_mouse_move(&mut self, x: f64, y: f64) {
        self.mouse.delta_move.x = x as f32 - self.mouse.pos.x;
        self.mouse.delta_move.y = y as f32 - self.mouse.pos.y;
        self.mouse.pos.x = x as f32;
        self.mouse.pos.y = y as f32;
    }

    #[inline]
    pub fn handle_mouse_press(&mut self, button: MouseButton, state: ElementState) {
        unsafe {
            match state {
                ElementState::Pressed => {
                    self.mouse.buttons[Self::get_mouse_button_index(button)] = EventState::Pressed;
                },
                ElementState::Released => {
                    self.mouse.buttons[Self::get_mouse_button_index(button)] = EventState::Released;
                },
            }
        }
    }

    #[inline]
    pub fn handle_mouse_wheel(&mut self, scroll_y: f32) {
        self.mouse.wheel += scroll_y;
    }

    #[inline]
    pub fn refresh(&mut self) {
        self.mouse.delta_move.x = 0.;
        self.mouse.delta_move.y = 0.;
        self.mouse.wheel = 0.;

        self.last_modified_keys.iter().for_each(|k| {
            match self.keys[*k as usize] {
                EventState::Pressed => self.keys[*k as usize] = EventState::Down,
                EventState::Released => self.keys[*k as usize] = EventState::Up,
                _ => ()
            }
        });

        self.last_modified_keys.clear();

        self.mouse.buttons.iter_mut().for_each(|mut b| {
            match b {
                EventState::Pressed => {*b = EventState::Down},
                EventState::Released => {*b = EventState::Up},
                _ => ()
            }
        });
    }

    #[inline]
    pub fn key_state(&self, key: VirtualKeyCode, state: EventState) -> bool {
        match state {
            EventState::Pressed | EventState::Released=> self.keys[key as usize] == state,
            EventState::Up => self.keys[key as usize] == EventState::Up || self.keys[key as usize] == EventState::Released,
            EventState::Down => self.keys[key as usize] == EventState::Down || self.keys[key as usize] == EventState::Pressed,
        }
    }

    #[inline]
    pub fn mouse_button_state(&self, button: MouseButton, state: EventState) -> bool {
        match state {
            EventState::Pressed | EventState::Released=> self.get_mouse_button_state(button) == state,
            EventState::Up => self.get_mouse_button_state(button) == EventState::Up || self.get_mouse_button_state(button) == EventState::Released,
            EventState::Down => self.get_mouse_button_state(button) == EventState::Down || self.get_mouse_button_state(button) == EventState::Pressed,
        }
    }

    #[inline]
    pub fn get_mouse_wheel(&self) -> f32 {
        self.mouse.wheel
    }

    #[inline]
    pub fn get_relative_mouse_position(&self) -> Vector2<f32> {
        Vector2::new(
            (self.mouse.pos.x / WINDOW_WIDTH as f32) * 2. - 1.,
            (self.mouse.pos.y / WINDOW_HEIGHT as f32) * 2. - 1.,
        )
    }

    fn get_mouse_button_state(&self, button: MouseButton) -> EventState {
        unsafe {
            self.mouse.buttons[*(&button as *const _ as *const u8) as usize]
        }
    }

    pub fn get_mouse_button_index(button: MouseButton) -> usize {
        unsafe { *(&button as *const _ as *const u8) as usize }
    }

}

pub struct Mouse {
    pub buttons: [EventState; 3],
    pub pos: Vector2<f32>,
    pub delta_move: Vector2<f32>,
    pub wheel: f32,
}

