mod engine;
mod game;
mod input;
mod map;
mod render;

use crate::{
    engine::vector2::Vector2,
    render::{Render, TileTextures},
};
use sdl2::event::Event;

fn main() {
    let mut game = game::Game::new();
    let texture_creator = game.canvas.texture_creator();
    let textures = TileTextures::init(&texture_creator);

    println!("{}", game.map);

    'running: loop {
        game.canvas.clear();

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
                Event::MouseButtonDown { mouse_btn, .. } => {
                    game.input.on_mousebutton_pressed(mouse_btn)
                }
                Event::MouseButtonUp { mouse_btn, .. } => {
                    game.input.on_mousebutton_released(mouse_btn)
                }
                Event::MouseMotion { x, y, .. } => {
                    game.input.on_mouse_moved(Vector2::new(x as f32, y as f32))
                }
                Event::MouseWheel { y, .. } => game.input.on_mousewheel_scrolled(y as i8),
                _ => {}
            }
        }

        //println!("Camera position: {}", game.camera.position);
        //println!("Relative mouse position: {:?}", game.input.get_rel_mouse_position(game.window_size));
        //println!("FPS: {}", (1.0 / game.delta_time) as i32);
        println!("Zoom: {}", game.camera.zoom);

        game.render_objects(&textures)
            .expect("Something went wrong while rendering!");
        game.refresh_game();
        game.canvas.present();
    }
}
