mod buffers;
pub mod frames;
pub mod gpustoredinstances;
mod initialization;
mod other;
pub mod shaders;

use std::sync::Arc;

use crate::camera::Camera;
use crate::input::Input;
use crate::map::Map;
use gpustoredinstances::{GpuStoredGameObject, GpuStoredHUDObject};
use vulkano::buffer::DeviceLocalBuffer;
use vulkano::descriptor_set::pool::standard::StandardDescriptorPoolAlloc;
use vulkano::descriptor_set::PersistentDescriptorSet;
use vulkano::format::ClearValue;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::Framebuffer;
use vulkano::sync::GpuFuture;
use vulkano::{
    device::{Device, Queue},
    pipeline::GraphicsPipeline,
    render_pass::RenderPass,
    swapchain::{Surface, Swapchain},
};
use winit::window::Window;

pub struct VulkanApp {
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    framebuffers: Vec<Arc<Framebuffer>>,
    draw_image_index: usize, //The index of the image the GPU is drawing on.
    viewport: Viewport,
    render_pass: Arc<RenderPass>,
    clear_values: Vec<Option<ClearValue>>,
    gameobject_pipeline: Arc<GraphicsPipeline>,
    hud_pipeline: Arc<GraphicsPipeline>,
    tile_texture_descriptor_set: Arc<PersistentDescriptorSet<StandardDescriptorPoolAlloc>>,
    building_texture_descriptor_set: Arc<PersistentDescriptorSet<StandardDescriptorPoolAlloc>>,
    hud_texture_descriptor_set: Arc<PersistentDescriptorSet<StandardDescriptorPoolAlloc>>,
    pub recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    device_local_tile_instance_buffer: Arc<DeviceLocalBuffer<[GpuStoredGameObject]>>,
    device_local_building_instance_buffer: Arc<DeviceLocalBuffer<[GpuStoredGameObject]>>,
    device_local_hud_instance_buffer: Arc<DeviceLocalBuffer<[GpuStoredHUDObject]>>,
    building_instance_count: u16,
    hud_instance_count: u8,
    //END OF VULKAN VARIABLES
    //END OF VULKAN VARIABLES
    pub input: Input,
    pub map: Map,
    pub camera: Camera,
}
