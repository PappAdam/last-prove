mod game;
mod input;
mod map;
mod engine;

use sdl2::event::Event;

fn main() {
    let mut game = game::Game::new();

    print!("{}", game.map);

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
        
        game.input.refresh_input();
        game.canvas.present();
    }
}
