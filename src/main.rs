mod game;
mod input;
mod map;
mod render;
mod engine;

use crate::render::{Render, TileTextures};
use sdl2::{event::Event, sys::SDL_GetPerformanceCounter};

fn main() {
    let mut game = game::Game::new();
    let texture_creator = game.canvas.texture_creator();
    let textures = TileTextures::init(&texture_creator);

    'running: loop {
        game.canvas.clear();
        unsafe { game.calculate_delta_time(SDL_GetPerformanceCounter()); }

        game.render_objects(&textures).expect("Something went wrong while rendering!");

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
                Event::MouseButtonDown {
                    mouse_btn,
                     .. 
                } => game.input.on_mousebutton_pressed(mouse_btn),
                Event::MouseButtonUp {
                    mouse_btn,
                     .. 
                } => game.input.on_mousebutton_released(mouse_btn),
                Event::MouseMotion { 
                    x,
                    y,
                    ..
                } => game.input.on_mouse_moved((x as u16, y as u16)),
                Event::MouseWheel {
                    y,
                    ..
                } => game.input.on_mousewheel_scrolled(y as i8),                
                _ => {}
            }
        }

        println!("FPS: {}", (1.0 / game.delta_time) as i32);

        game.input.refresh_input();
        game.canvas.present();
    }
}
