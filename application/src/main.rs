mod input;
mod mainstruct;

use std::{f32::consts::PI, time::Instant};

use input::Input;
use nalgebra::{Matrix4, Vector3};
use objects::{mesh::Mesh, GameObject, ObjectType, transformations::Transformations, getters::Getters};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::ControlFlow,
    platform::run_return::EventLoopExtRunReturn,
    window::Fullscreen,
};

use renderer::Renderer;
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

    let mut renderer = match Renderer::new(&window) {
        Ok(base) => base,
        Err(err) => {
            msg!(error, err);
            panic!("{}", err);
        }
    };
    let mut camera_transform = Matrix4::<f32>::identity();
    let mut camera = Matrix4::identity();
    let mut transform_array =
        AlignedArray::<Matrix4<f32>>::from_dynamic_ub_data(&renderer.data.dynamic_uniform_buffer);

    let meshes = [
        Mesh::from_obj(&renderer, "resources/models/rat_obj.obj"),
        Mesh::from_obj(&renderer, "resources/models/ez.obj"),
    ];

    let mut ez_go =
        GameObject::object(&mut transform_array, &meshes[0], ObjectType::SomeObject).unwrap();
    let mut az_go =
        GameObject::object(&mut transform_array, &meshes[1], ObjectType::SomeObject).unwrap();

    let mut start_time = Instant::now();
    let mut input = Input::init();

    event_loop.run_return(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(..) => {
                renderer.rebuild_swapchain = true;
            }
            WindowEvent::CursorMoved { position, .. } => {
                input.handle_mouse_move(position.x, position.y);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                input.handle_mouse_press(button, state);
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
                let direction = camera.x_axis().cross(&Vector3::y_axis()).normalize()
                    * delta_time.as_secs_f32();
                camera.traslate(direction.x, 0., direction.z);
            }
            if input.get_key_down(winit::event::VirtualKeyCode::S) {
                let direction = Vector3::y_axis().cross(&camera.x_axis()).normalize()
                    * delta_time.as_secs_f32();
                camera.traslate(direction.x, 0., direction.z);
            }
            if input.get_key_down(winit::event::VirtualKeyCode::A) {
                camera.traslate_local(1. * delta_time.as_secs_f32(), 0., 0.);
            }
            if input.get_key_down(winit::event::VirtualKeyCode::D) {
                camera.traslate_local(-1. * delta_time.as_secs_f32(), 0., 0.);
            }

            if input.get_key_down(winit::event::VirtualKeyCode::L) {
                dbg!(camera.z_axis());
            }
            renderer.data.world_view.view = camera;

            renderer.data.dynamic_uniform_buffer.update(
                &renderer.base.device,
                &[renderer.data.descriptor_sets[renderer.current_frame_index]],
            );
            renderer.prepare_renderer().unwrap();
            renderer.stage_mesh(ez_go.renderable_form());
            renderer.stage_mesh(az_go.renderable_form());

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
