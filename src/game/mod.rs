use sdl2::{Sdl, render::{Canvas}, video::Window, pixels::Color};
use crate::{mapgen, input};

pub struct Game {
    pub context: Sdl,
    pub canvas: Canvas<Window>,
    pub map: mapgen::generate::Map,
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

        let mut map = mapgen::generate::Map::new(None).generate();

        let event_pump = context.event_pump().unwrap();

        let input = input::Input::init();

        Self { context, canvas, map, event_pump, input }
    }
}