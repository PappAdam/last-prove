mod events;

use std::{f32::consts::PI, time::Instant};

use events::input::Input;
use nalgebra::{Matrix4, Point, Point3, Vector2, Vector3, Vector4, Vector6};
use objects::{mesh::Mesh, GameObject, GameObjectHandler, GameObjectType};
use winit::{
    dpi::{LogicalSize, PhysicalSize, Position},
    event::{Event, KeyboardInput, MouseButton, WindowEvent},
    event_loop::ControlFlow,
    window::Fullscreen,
};

use renderer::Renderer;
use renderer::{
    engine::aligned_array::{self, AlignedArray},
    msg,
};

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

    // let mesh_templates = objects::mesh::templates::create_templates();
    let mut gameobject_handler = GameObjectHandler::new();

    let mut camera = GameObject::new(Vector3::new(0., 0., 0.), objects::GameObjectType::Camera);

    simplelog::CombinedLogger::init(loggers).unwrap();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("HAHA")
        .with_inner_size(PhysicalSize::new(1920, 1080))
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut renderer = match Renderer::new(&window) {
        Ok(base) => base,
        Err(err) => {
            msg!(error, err);
            panic!("{}", err);
        }
    };

    gameobject_handler.add_object(GameObject::new(
        Vector3::new(0., 0., 0.),
        objects::GameObjectType::Terrain(Mesh::from_obj(&renderer, "resources/models/ez.obj", 0)),
    ));

    let mut aligned_array =
        AlignedArray::<Matrix4<f32>>::from_dynamic_ub_data(&renderer.data.dynamic_uniform_buffer);

    aligned_array[0] = gameobject_handler.gameobjects[0].get_transform();

    renderer
        .data
        .dynamic_uniform_buffer
        .update(&renderer.base.device, &renderer.data.descriptor_sets);

    let mut start_time = Instant::now();
    let mut input = Input::init();

    renderer.data.world_view.view = camera.get_transform();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
                // gameobject_handler.gameobjects.iter().for_each(|go| {
                //     if let GameObjectType::Terrain(mesh) = &go.ty {
                //         mesh.free(&renderer.base.device);
                //     }
                // })
            }
            WindowEvent::Resized(..) => {
                renderer.rebuild_swapchain = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                input.mouse.set_pos(position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                input.mouse.set_button(button, state);
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

            aligned_array[0] = gameobject_handler.gameobjects[0].get_transform();

            let _ = renderer.prepare_renderer();

            gameobject_handler.gameobjects[0].rotate(delta_time.as_secs_f32(), 0., 0.);

            renderer.data.dynamic_uniform_buffer.update(
                &renderer.base.device,
                &[renderer.data.descriptor_sets[renderer.current_frame_index]],
            );

            if let GameObjectType::Terrain(mesh) = &gameobject_handler.gameobjects[0].ty {
                renderer.stage_mesh(mesh.into_tuple());
            }

            if let Err(msg) = renderer.flush() {
                msg!(error, msg);
                *control_flow = ControlFlow::Exit;
                return;
            }

            input.refresh();
        }
        _ => {}
    });
}
