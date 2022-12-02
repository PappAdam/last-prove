mod vulkanapp;
mod map;
mod engine;
use map::Map;
use vulkanapp::VulkanApp;
use winit::{
    event::{Event, WindowEvent},
};

fn main() {
    // let vulkan_app = VulkanApp::init();

    // vulkan_app.event_loop.run(move |event, _, control_flow| {
    //     match event {
    //         Event::WindowEvent { event, .. } => {
    //             match event {
    //                 WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
    //                 _ => {  }
    //             }
    //         },
    //         _ => {}
    //     }
    // })

    let map = Map::new(20, 10, None).generate_automata(1.0);
    println!("{}", map);
}
