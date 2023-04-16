mod events;
mod utils;

use std::{f32::consts::PI, time::Instant};

use events::input::Input;
use nalgebra::{Point, Point3, Vector2, Vector3, Vector4, Vector6};
use objects::mesh::Vertex;
use utils::{create_cube, Side};
use winit::{
    dpi::{LogicalSize, PhysicalSize, Position},
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
    window::Fullscreen,
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
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut vertecies = Vec::<Vertex>::new();
    vertecies.append(&mut create_cube(
        Side::CUBE,
        Vector3::new(0., 0., 0.),
        Vector3::new(2., 1., 2.),
        Vector3::new(1., 1., 1.),
        0.5,
    ));
    vertecies.append(&mut create_cube(
        Side::CUBE & !Side::BOTTOM,
        Vector3::new(0., -1., 0.),
        Vector3::new(1., 1., 1.),
        Vector3::new(1., 1., 1.),
        0.5,
    ));

    let mut renderer = match Renderer::new(&window, &vertecies) {
        Ok(base) => base,
        Err(err) => {
            msg!(error, err);
            panic!("{}", err);
        }
    };

    let mut start_time = Instant::now();
    let mut rotation = Vector2::new(2f32, 1.);
    let mut is_rotate = false;

    let mut input = Input::init();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(..) => {
                renderer.rebuild_swapchain = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                input.mouse.set_pos(position.x, position.y);
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
                    input.handle_key_press(virtual_keycode, state);
                }
            },
            WindowEvent::ModifiersChanged(modifier) => input.set_modif(modifier),
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

            // renderer.data.transform.view = nalgebra::Matrix::look_at_lh(
            //     &Point3::new(rotation.y.sin(), rotation.x.cos(), rotation.y.cos()),
            //     &Point3::new(0., 0., 0.),
            //     &Vector3::y_axis(),
            // );

            if let Err(msg) = renderer.draw() {
                msg!(error, msg);
                *control_flow = ControlFlow::Exit;
                return;
            }

            input.refresh();
            start_time = Instant::now();
        }
        _ => {}
    });
}
