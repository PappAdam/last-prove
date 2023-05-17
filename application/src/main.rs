mod application;
mod input;

use std::{borrow::BorrowMut, f32::consts::PI, sync::Arc, time::Instant};

use application::App;
use input::Input;
use nalgebra::{Matrix4, Vector3};
use objects::{
    getters::Getters, mesh::Mesh, transformations::Transformations, GameObject, ObjectType,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::Fullscreen,
};

use renderer::{engine::aligned_array::AlignedArray, msg};
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
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut app = App::init(&window);
    let mut meshes = Vec::<Mesh>::new();

    app.setup(&mut meshes);

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
                app.input.handle_mouse_press(button, state);
            }
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
            app.renderer.data.world_view.view = *app.get_cam();
            app.renderer.data.dynamic_uniform_buffer.update(
                &app.renderer.base.device,
                &[app.renderer.data.descriptor_sets[app.renderer.current_frame_index]],
            );

            app.camera_move();
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
