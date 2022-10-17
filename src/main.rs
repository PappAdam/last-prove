pub mod game;
pub mod input;
pub mod map;
pub mod render;

use crate::render::{Render, TileTextures};

use sdl2::event::Event;

fn main() {
    let mut game = game::Game::new();
    let texture_creator = game.canvas.texture_creator();
    let textures = TileTextures::init(&texture_creator);

    'running: loop {
        game.canvas.clear();

        game.render_objects(&textures);

        for event in game.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode,
                    repeat: false,
                    ..
                } => game.input.on_key_pressed(keycode),
                Event::KeyUp {
                    keycode,
                    repeat: false,
                    ..
                } => game.input.on_key_released(keycode),
                _ => {}
            }
        }

        game.input.refresh_input();

        game.canvas.present();
    }
}
