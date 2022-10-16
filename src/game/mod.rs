use crate::input;
use crate::map;
use sdl2::{pixels::Color, render::Canvas, video::Window, Sdl};

pub struct Game {
    pub context: Sdl,
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

        Self {
            context,
            canvas,
            map,
            event_pump,
            input,
        }
    }
}
