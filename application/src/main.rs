use std::{f32::consts::PI, time::Instant};

use nalgebra_glm::{
    look_at, look_at_lh, look_at_rh, rotate_normalized_axis, vec2, vec3, TVec2, Vec2,
};
use winit::{
    dpi::Position,
    event::{
        ElementState, Event, KeyboardInput, ModifiersState, MouseButton, VirtualKeyCode,
        WindowEvent,
    },
    event_loop::ControlFlow,
};

use renderer::msg;
use renderer::Renderer;

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

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("HAHA")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut renderer = match Renderer::new(&window) {
        Ok(base) => base,
        Err(err) => {
            msg!(error, err);
            panic!("{}", err);
        }
    };

    let mut start_time = Instant::now();
    let mut rotation = vec2(0f32, 0.);
    let mut is_rotate = false;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(..) => {
                renderer.rebuild_swapchain = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                rotation.y = position.x as f32 / window.inner_size().width as f32 * 2. * PI;
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state,
                    virtual_keycode,
                    ..
                } => match virtual_keycode {
                    Some(VirtualKeyCode::Space) => is_rotate = state == ElementState::Pressed,
                    _ => {}
                },
            },
            _ => {}
        },
        Event::MainEventsCleared => {
            let delta_time = start_time.elapsed();

            if renderer.rebuild_swapchain {
                renderer.rebuild_swapchain = false;
                if let Err(msg) = renderer.resize(&window) {
                    msg!(error, msg);
                    return;
                }
            }

            renderer.data.transform.view = look_at_lh(
                &vec3(rotation.y.sin(), 0., rotation.y.cos()),
                &vec3(0., 0., 0.),
                &vec3(0., 1., 0.),
            );

            if is_rotate {
                renderer.data.transform.rotation = rotate_normalized_axis(
                    &renderer.data.transform.rotation,
                    delta_time.as_secs_f32() * PI * 10.,
                    &vec3(1., 0., 0.),
                );
            }

            if let Err(msg) = renderer.draw() {
                msg!(error, msg);
                *control_flow = ControlFlow::Exit;
                return;
            }

            start_time = Instant::now();
        }
        _ => {}
    });
}
