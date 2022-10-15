mod mapgen;
pub mod perlin;
mod game;

use sdl2::{event::Event, keyboard::Keycode};

fn main() {
    let mut game = game::Game::new();

    print!("{}", game.map);

    'running: loop {
        game.canvas.clear();

        for event in game.event_pump.poll_iter() {
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

        game.canvas.present();
    }
}
