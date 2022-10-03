use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::time::Duration;

fn main() {
    let sdl_cont = sdl2::init().expect("couldn't crate sdl context");
    let video_subsys = sdl_cont.video().expect("couldn't create video subsystem");

    let window = video_subsys
        .window("title", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut event_pump = sdl_cont.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.present();
    }
}
