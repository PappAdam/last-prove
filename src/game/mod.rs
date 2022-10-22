use crate::input::Input;
use crate::map;
use crate::render::camera::Camera;
use sdl2::{
    mouse::MouseButton,
    pixels::Color,
    render::Canvas,
    sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency},
    video::Window,
    Sdl,
};

pub struct Game {
    pub context: Sdl,
    pub window_size: (u16, u16),
    pub canvas: Canvas<Window>,
    pub map: map::Map,
    pub event_pump: sdl2::EventPump,
    pub input: Input,
    pub camera: Camera,
    pub delta_time: f32,
    last: u64,
    //camera: <T>,
}

impl Game {
    pub fn new() -> Self {
        let context = sdl2::init().expect("couldn't crate sdl context");
        let video_subsys = context.video().expect("couldn't create video subsystem");

        let window_size = (960, 540);

        let window = video_subsys
            .window("title", window_size.0 as u32, window_size.1 as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        let event_pump = context.event_pump().unwrap();
        let input = Input::init(window_size);
        
        #[warn(unused_mut)]
        let mut map = map::Map::new(100, None).generate();
        
        let camera = Camera::new();

        let delta_time = 0.0;
        let last = 0;

        Self {
            context,
            window_size,
            canvas,
            map,
            event_pump,
            input,
            camera,
            delta_time,
            last,
        }
    }

    pub fn refresh_game(&mut self) {
        self.camera.refresh_camera(
            self.delta_time,
            self.input.get_mouse_position(),
            self.input.get_mousebutton_state(MouseButton::Middle),
            self.input.get_mouse_wheel(),
        );
        self.input.refresh_input();
        self.refresh_delta_time();
    }

    pub fn refresh_delta_time(&mut self) {
        unsafe {
            let now = SDL_GetPerformanceCounter();
            self.delta_time = (now - self.last) as f32 / SDL_GetPerformanceFrequency() as f32;
            self.last = now;
        }
    }
}
