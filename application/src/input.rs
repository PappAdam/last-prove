use nalgebra::Vector2;
use winit::event::{ElementState, ModifiersState, MouseButton, VirtualKeyCode};

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
            mouse: Mouse {
                buttons: [ElementState::Released; 3],
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
            self.keys[key as usize] = state
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
            self.mouse.buttons[*(&button as *const _ as *const u8) as usize] = state;
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
    }

    #[inline]
    pub fn get_key_down(&self, key: VirtualKeyCode) -> bool {
        self.keys[key as usize] == ElementState::Pressed
    }

    #[inline]
    pub fn get_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe {
            self.mouse.buttons[*(&button as *const _ as *const u8) as usize]
                == ElementState::Pressed
        }
    }
    
    #[inline]
    pub fn get_mouse_wheel(&self) -> f32 {
        self.mouse.wheel
    }
}

pub struct Mouse {
    pub buttons: [ElementState; 3],
    pub pos: Vector2<f32>,
    pub delta_move: Vector2<f32>,
    pub wheel: f32
}

impl Mouse {

}

enum MouseWheelState {
    Up(u8),
    Stationary,
    Down(u8),
}