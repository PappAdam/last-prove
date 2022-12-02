use std::{ops::Deref, sync::Arc};

use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
    },
    image::{ImageUsage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    shader::ShaderModule,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
    Version, VulkanLibrary, render_pass::{RenderPass, Subpass}, pipeline::{GraphicsPipeline, graphics::input_assembly::{InputAssemblyState, PrimitiveTopology}},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::map::Map;

pub struct VulkanApp {
    physical: Arc<PhysicalDevice>,
    device: Arc<Device>,
    surface: Arc<Surface<Window>>,
    pub event_loop: EventLoop<()>,
    graphics_queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
    tile_vertex_shader: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    map: Map,
}

impl VulkanApp {
    pub fn init() -> Self {
        let vulkan_library = VulkanLibrary::new().unwrap();
        let vulkano_win_extensions = vulkano_win::required_extensions(&vulkan_library);
        let instance = Instance::new(
            vulkan_library,
            InstanceCreateInfo {
                application_name: Some(String::from("")),
                application_version: Version {
                    major: 0,
                    minor: 0,
                    patch: 1,
                },
                enabled_extensions: vulkano_win_extensions,
                ..Default::default()
            },
        )
        .unwrap();

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let (physical, queue_family_index) =
            Self::query_physical_device(instance, &device_extensions, &surface);

        let (device, graphics_queue) =
            Self::create_device(physical.clone(), device_extensions, queue_family_index);

        let (swapchain, swapchain_images) = Self::create_swapchain(device.clone(), surface.clone());

        let tile_vertex_shader = tile_vertex_shader::load(device.clone()).unwrap();

        let render_pass = Self::create_render_pass(device.clone(), swapchain.clone());

        let pipeline = Self::create_pipeline(device.clone(), render_pass.clone());

        Self {
            device,
            physical,
            surface,
            event_loop,
            graphics_queue,
            swapchain,
            swapchain_images,
            tile_vertex_shader,
            render_pass,
            map: Map::new(200, 10, Some(10)).generate(),
        }
    }

    fn query_physical_device(
        instance: Arc<Instance>,
        device_extensions: &DeviceExtensions,
        surface: &Surface<Window>,
    ) -> (Arc<PhysicalDevice>, u32) {
        instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.graphics && p.surface_support(i as u32, surface).unwrap()
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap()
    }

    fn create_device(
        physical: Arc<PhysicalDevice>,
        device_extensions: DeviceExtensions,
        queue_family_index: u32,
    ) -> (Arc<Device>, Arc<Queue>) {
        let mut device = Device::new(
            physical.clone(),
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    queues: vec![0.5],
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        (device.0, device.1.next().unwrap())
    }

    fn create_swapchain(
        device: Arc<Device>,
        surface: Arc<Surface<Window>>,
    ) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
        let surface_capabilites = device
            .physical_device()
            .surface_capabilities(surface.deref(), Default::default())
            .unwrap();
        let image_format = Some(
            device
                .physical_device()
                .surface_formats(surface.deref(), Default::default())
                .unwrap()[0]
                .0,
        );

        Swapchain::new(
            device,
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilites.min_image_count,
                image_format,
                image_extent: surface.window().inner_size().into(),
                image_usage: ImageUsage {
                    color_attachment: true,
                    ..Default::default()
                },
                composite_alpha: surface_capabilites
                    .supported_composite_alpha
                    .iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        )
        .unwrap()
    }

    fn create_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain<Window>>) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap()
    }

    fn create_pipeline(device: Arc<Device>, render_pass: Arc<RenderPass>) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            //.vertex_input_state(BuffersDefinition::new().instance::<>()) TODO: Create Instance (Tile struct)
            .render_pass(Subpass::from(render_pass, 0).unwrap())
            .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip))
            .build(device.clone())
            .unwrap()
    }
}


mod tile_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/tile_vertex_shader.vert"
    }
}
