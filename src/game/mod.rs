use crate::input::Input;
use crate::map;
use crate::render::camera::Camera;
use sdl2::{
    mouse::MouseButton,
    pixels::Color,
    render::Canvas,
    sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    video::Window,
};

pub struct Game {
    pub window_size: (u16, u16),
    pub canvas: Canvas<Window>,
    pub map: map::Map,
    pub event_pump: sdl2::EventPump,
    pub input: Input,
    pub camera: Camera,
    pub delta_time: f32,
    last_frame: u64,
}

impl Game {
    pub fn new() -> Self {
        let context = sdl2::init().expect("couldn't crate sdl context");
        let video_subsys = context.video().expect("couldn't create video subsystem");

        let window_size = ((960f32*2f32) as u16, (540f32*2f32) as u16);

        let window = video_subsys
            .window("title", window_size.0 as u32, window_size.1 as u32)
            .position_centered()
            .fullscreen()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        let event_pump = context.event_pump().unwrap();
        let input = Input::init(window_size);
        
        let map = map::Map::new(200, 10, Some(20)).generate(); //.flat(0);
        
        let camera = Camera::new();

        let delta_time = 0.0;
        let last_frame = 0;

        Self {
            window_size,
            canvas,
            map,
            event_pump,
            input,
            camera,
            delta_time,
            last_frame,
        }
    }

    pub fn refresh_game(&mut self) {

        self.camera.refresh_camera(
            self.input.get_mouse_movement(),
            self.input.get_mousebutton_down(MouseButton::Middle),
            self.input.get_mouse_wheel(),
        );

        self.input.refresh_input();

        self.refresh_delta_time();
    }

    pub fn refresh_delta_time(&mut self) {
        unsafe {
            let now = SDL_GetPerformanceCounter();
            self.delta_time = (now - self.last_frame) as f32 / SDL_GetPerformanceFrequency() as f32;
            self.last_frame = now;
        }
    }
}
