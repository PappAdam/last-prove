mod events;
mod utils;

use std::{f32::consts::PI, time::Instant};

use events::input::Input;
use nalgebra::{Point, Point3, Vector2, Vector3, Vector4, Vector6};
use objects::{
    mesh::{vertex::Vertex, Mesh},
    GameObject, GameObjectHandler,
};
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

    let mesh_templates = objects::mesh::templates::create_templates();
    let mut gameobject_handler = GameObjectHandler::new();

    gameobject_handler.add_object(GameObject::new(
        Vector3::new(0., 0., 0.),
        objects::GameObjectType::Terrain(Mesh::from_obj("resources/models/Container.obj")),
    ));
    gameobject_handler.add_object(GameObject::new(
        Vector3::new(0., 0.5, 0.),
        objects::GameObjectType::Terrain(Mesh::from_obj("resources/models/Basic_house.obj")),
    ));

    let mut camera = GameObject::new(Vector3::new(0., 0., 0.), objects::GameObjectType::Camera);
    // camera.look_at(Vector3::new(0., 0., 0.));
    // camera.rotate_local(0., PI / 6., 0.);
    // camera.rotate_local(PI / 6., 0., 0.);
    dbg!(camera.get_transform());

    simplelog::CombinedLogger::init(loggers).unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("HAHA")
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let mut renderer = match Renderer::new(
        &window,
        &gameobject_handler.gameobjects[1].get_vertices(&mesh_templates),
        &gameobject_handler.gameobjects[1]
            .get_mesh(&mesh_templates)
            .get_indicies(),
    ) {
        Ok(base) => base,
        Err(err) => {
            msg!(error, err);
            panic!("{}", err);
        }
    };

    let mut start_time = Instant::now();
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
            start_time = Instant::now();

            if renderer.rebuild_swapchain {
                renderer.rebuild_swapchain = false;
                if let Err(msg) = renderer.resize(&window) {
                    msg!(error, msg);
                    return;
                }
            }
            //Idk where we should handle inputs, it is gonna be here for now.
            if input.get_key_down(winit::event::VirtualKeyCode::Q) {
                camera.orbit(
                    0.,
                    (PI / 2.) * delta_time.as_secs_f32(),
                    0.,
                    Vector3::new(0., 0., 0.),
                );
            }
            if input.get_key_down(winit::event::VirtualKeyCode::E) {
                camera.orbit(
                    0.,
                    -(PI / 2.) * delta_time.as_secs_f32(),
                    0.,
                    Vector3::new(0., 0., 0.),
                );
            }
            if input.get_key_down(winit::event::VirtualKeyCode::R) {
                camera.orbit_local(
                    (PI / 2.) * delta_time.as_secs_f32(),
                    0.,
                    0.,
                    Vector3::new(0., 0., 0.),
                );
            }
            if input.get_key_down(winit::event::VirtualKeyCode::F) {
                camera.orbit_local(
                    -(PI / 2.) * delta_time.as_secs_f32(),
                    0.,
                    0.,
                    Vector3::new(0., 0., 0.),
                );
            }
            if input.get_key_down(winit::event::VirtualKeyCode::W) {
                let mut direction = camera.z_axis();
                direction.y = 0.;
                direction.normalize_mut();
                camera.traslate_local(direction.x * delta_time.as_secs_f32(), 0., direction.z * delta_time.as_secs_f32());
            }
            if input.get_key_down(winit::event::VirtualKeyCode::S) {
                camera.traslate_local(0., 0., -1. * delta_time.as_secs_f32());
            }
            if input.get_key_down(winit::event::VirtualKeyCode::A) {
                camera.traslate_local(1. * delta_time.as_secs_f32(), 0., 0.);
            }
            if input.get_key_down(winit::event::VirtualKeyCode::D) {
                camera.traslate_local(-1. * delta_time.as_secs_f32(), 0., 0.);
            }
            // camera.orbit(0., 3.14 * delta_time.as_secs_f32(), 0., Vector3::zeros());
            // renderer.data.transform.view = nalgebra::Matrix::look_at_lh(
            //     &Point3::from(camera.get_position()),
            //     &Point3::new(0., 0., 0.),
            //     &Vector3::y_axis(),
            // );

            renderer.data.transform.view = camera.get_transform();

            if let Err(msg) = renderer.draw() {
                msg!(error, msg);
                *control_flow = ControlFlow::Exit;
                return;
            }

            input.refresh();
        }
        _ => {}
    });
}
