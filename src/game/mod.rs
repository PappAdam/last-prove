use crate::input;
use crate::map;
use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl, sys::SDL_GetPerformanceFrequency};

pub struct Game {
    pub context: Sdl,
    pub canvas: Canvas<Window>,
    pub map: map::Map,
    pub event_pump: sdl2::EventPump,
    pub input: input::Input,
    pub delta_time: f64,
    last: u64,
    //camera: <T>,
}

impl Game {
    pub fn new() -> Self {
        let context = sdl2::init().expect("couldn't crate sdl context");
        let video_subsys = context.video().expect("couldn't create video subsystem");

        let window = video_subsys
            .window("title", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        let event_pump = context.event_pump().unwrap();
        let input = input::Input::init();
        let mut map = map::Map::new(100, Some(20));
        map.generate();

        let delta_time = 0.0;
        let last = 0;

        Self {
            context,
            canvas,
            map,
            event_pump,
            input,
            delta_time,
            last
        }
    }

    pub fn calculate_delta_time(&mut self, now: u64) {
        unsafe {
            self.delta_time = (now - self.last) as f64 / SDL_GetPerformanceFrequency() as f64;
            self.last = now;
        }
    }
}
