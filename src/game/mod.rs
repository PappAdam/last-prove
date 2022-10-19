use crate::input;
use crate::map;
use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub struct Game {
    pub context: Sdl,
    pub window_size: (u16, u16),
    pub canvas: Canvas<Window>,
    pub map: map::Map,
    pub event_pump: sdl2::EventPump,
    pub input: input::Input,
    //camera: <T>,
}

impl Game {
    pub fn new() -> Self {
        let context = sdl2::init().expect("couldn't crate sdl context");
        let video_subsys = context.video().expect("couldn't create video subsystem");
        
        let window_size = (800, 600);
        
        let window = video_subsys
            .window("title", window_size.0 as u32, window_size.1 as u32)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        let event_pump = context.event_pump().unwrap();
        let input = input::Input::init();

        let mut map = map::Map::new(100, Some(20));
        map.generate();

        Self {
            context,
            window_size,
            canvas,
            map,
            event_pump,
            input,
        }
    }
}
