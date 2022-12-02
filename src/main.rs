mod vulkanapp;
mod map;
mod engine;
mod input;


use map::Map;
use vulkanapp::VulkanApp;
use winit::{
    event::{Event, WindowEvent}, dpi::PhysicalPosition,
};

fn main() {
    let vulkan_app = VulkanApp::init();

    let mut asdinput = input::Input::init((600, 800));
    
    let map = Map::new(20, 10, None).generate_automata(0.5);
    println!("{}", map);

    vulkan_app.event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => asdinput.on_key_input(input),
                    WindowEvent::MouseInput { button, state, .. } => asdinput.on_mousebutton_input(button, state),
                    WindowEvent::CursorMoved { position, .. } => asdinput.on_mouse_moved(position.into()),
                    WindowEvent::MouseWheel { delta, .. } => asdinput.on_mousewheel_scrolled(delta),
                    _ => {  }
                }
            },
            _ => {}
        }
        asdinput.refresh_input();
    });
    
}
