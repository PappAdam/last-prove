mod application;
mod input;
mod map;

use std::time::Instant;

use application::App;
use objects::{
    flags::{GameObjectFlag},
    hitbox::Hitbox,
    mesh::Mesh,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, MouseScrollDelta, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::Fullscreen,
};

use macros::load_consts;

load_consts!("application/src/constants.const");

use renderer::msg;
fn main() {
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = vec![simplelog::TermLogger::new(
        simplelog::LevelFilter::Info,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )];
    if let Ok(file) = std::fs::File::create("log.txt") {
        loggers.push(simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            file,
        ));
    }

    simplelog::CombinedLogger::init(loggers).unwrap();

    let mut event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("HAHA")
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut meshes: Vec<Mesh> = vec![];
    let mut app = App::init(&window, MAP_SIZE, &meshes);
    app.load_meshes(&mut meshes);
    app.setup();

    app.renderer.data.push_const.wh_ratio = app.renderer.base.surface_extent.width as f32
        / app.renderer.base.surface_extent.height as f32;
    app.renderer.data.push_const.min_z = -200.;
    app.renderer.data.push_const.max_z = 200.;

    let mut start_time = Instant::now();
    event_loop.run_return(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(..) => {
                app.renderer.rebuild_swapchain = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                app.input.handle_mouse_move(position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                app.input.handle_mouse_press(button, state)
            }
            WindowEvent::MouseWheel {
                delta: MouseScrollDelta::LineDelta(_, scroll_y),
                ..
            } => app.input.handle_mouse_wheel(scroll_y),

            WindowEvent::KeyboardInput {
                input: keyboard_input,
                ..
            } => match keyboard_input {
                KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                } => {
                    app.input.handle_key_press(virtual_keycode, state);
                }
            },
            WindowEvent::ModifiersChanged(modifier) => app.input.set_modif(modifier),
            _ => {}
        },
        Event::MainEventsCleared => {
            app.delta_time = start_time.elapsed();

            start_time = Instant::now();

            if app.renderer.rebuild_swapchain {
                app.renderer.rebuild_swapchain = false;
                if let Err(msg) = app.renderer.resize(&window) {
                    msg!(error, msg);
                    return;
                }
            }

            app.renderer.prepare_renderer().unwrap();

            app.renderer.data.dynamic_uniform_buffer.update(
                &app.renderer.base.device,
                &[app.renderer.data.descriptor_sets[app.renderer.current_frame_index]],
            );

            app.renderer.data.world_view.view = *app.camera.get_transform();
            app.camera
                .camera_move(&app.input, app.delta_time.as_secs_f32());
            app.main_loop();

            if let Err(msg) = app.renderer.flush() {
                msg!(error, msg);
                *control_flow = ControlFlow::Exit;
                return;
            }

            app.input.refresh();
        }
        _ => {}
    });
}
