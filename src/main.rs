mod camera;
mod engine;
mod input;
mod map;
mod vulkanapp;

use core::time;
use engine::vector2::Vector2;
use map::building::Building;
use std::time::Instant;
use vulkanapp::VulkanApp;
use winit::event::{Event, WindowEvent, VirtualKeyCode};

fn main() {
    let (mut vulkan_app, event_loop) = VulkanApp::init();

    let mut last_frame = std::time::Instant::now();
    let mut frame_count: u128 = 0;
    let mut avg_elapsed = 0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                println!("AVG FPS: {}", 1000000 / avg_elapsed);
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
            WindowEvent::Resized(new_screen_size) => {
                vulkan_app.recreate_swapchain = true;
                vulkan_app.camera.window_resized(new_screen_size.into())
            }
            WindowEvent::KeyboardInput { input, .. } => vulkan_app.input.on_key_input(input),
            WindowEvent::MouseInput { button, state, .. } => {
                vulkan_app.input.on_mousebutton_input(button, state)
            }
            WindowEvent::CursorMoved { position, .. } => vulkan_app
                .input
                .on_mouse_moved(position.into(), vulkan_app.camera.camera_size),
            WindowEvent::MouseWheel { delta, .. } => vulkan_app.input.on_mousewheel_scrolled(delta),
            _ => {}
        },
        Event::RedrawEventsCleared => {
            frame_count += 1;
            let elapsed = last_frame.elapsed().as_micros();
            avg_elapsed = ((frame_count - 1) * avg_elapsed + elapsed) / frame_count;
            last_frame = std::time::Instant::now();

            if vulkan_app
                .input
                .get_mousebutton_pressed(winit::event::MouseButton::Left)
            {
                let mouse_coordinates = vulkan_app.camera.relative_screen_position_to_tile_coordinates(vulkan_app.input.get_mouse_position());
                if let Some(clicked_tile) = vulkan_app.map.get_clicked_tile(mouse_coordinates) {
                    vulkan_app.map.build_building(clicked_tile.coordinates.into(), 0);
                }
            }

            vulkan_app.refresh_game(elapsed as f32 / 1000000.0);
            vulkan_app.render();
        }
        _ => {}
    });
}
