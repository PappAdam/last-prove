use crate::input;
use crate::map;
use sdl2::{
    image::LoadTexture,
    pixels::Color,
    render::Canvas,
    render::{self, Texture},
    video::Window,
    video::WindowContext,
    Sdl,
};

pub struct Game {
    pub context: Sdl,
    pub canvas: Canvas<Window>,
    pub map: map::Map,
    pub event_pump: sdl2::EventPump,
    pub texture_creator: render::TextureCreator<WindowContext>,
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
        let texture_creator = canvas.texture_creator();
        let mut map = map::Map::new(100, Some(20));
        map.generate();

        Self {
            texture_creator,
            context,
            canvas,
            map,
            event_pump,
            input,
        }
    }
}
