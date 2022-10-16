mod game;
mod input;
mod map;

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
                _ => {}
            }
        }

        game.input.refresh_input();

        game.canvas.present();
    }
}
