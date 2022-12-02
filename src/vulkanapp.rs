use std::io::Cursor;
use std::{ops::Deref, sync::Arc};

use crate::create_texture;
use crate::map::tile::Tile;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::image::ImageAccess;
use vulkano::pipeline::Pipeline;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};
use vulkano::sampler::{Sampler, SamplerCreateInfo};
use vulkano::sync::GpuFuture;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
    },
    format::Format,
    image::{view::ImageView, ImageDimensions, ImageUsage, ImmutableImage, SwapchainImage},
    instance::{Instance, InstanceCreateInfo},
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        GraphicsPipeline,
    },
    render_pass::{RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
    Version, VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct VulkanApp {
    pub event_loop: EventLoop<()>,
    #[allow(dead_code)]
    surface: Arc<Surface<Window>>,
    //physical: Arc<PhysicalDevice>,
    //device: Arc<Device>,
    //graphics_queue: Arc<Queue>,
    //swapchain: Arc<Swapchain<Window>>,
    //swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
    //render_pass: Arc<RenderPass>,
    //map: Map,
    //NOTHING HAS TO BE STORED YET
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
        let tile_fragment_shader = tile_fragment_shader::load(device.clone()).unwrap();

        let render_pass = Self::create_render_pass(device.clone(), swapchain.clone());

        let pipeline = Self::create_pipeline(
            device.clone(),
            render_pass.clone(),
            tile_vertex_shader,
            tile_fragment_shader,
        );

        let (textures, texture_future) = create_texture!(
            graphics_queue.clone(),
            "../Assets/debug_tiles/0.png",
            "../Assets/debug_tiles/4.png"
        );

        let sampler =
            Sampler::new(device.clone(), SamplerCreateInfo::simple_repeat_linear()).unwrap();

        let layout = pipeline.layout().set_layouts().get(0).unwrap();
        let graphics_set = PersistentDescriptorSet::new(
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                textures.clone(),
                sampler.clone(),
            )],
        )
        .unwrap();

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        let mut framebuffers = Self::window_size_dependent_setup(&swapchain_images, &mut viewport, render_pass.clone());
        let mut recreate_swapchain = false;
        let mut previous_frame_end = Some(texture_future.boxed());

        Self {
            event_loop,
            surface, //map: Map::new(200, 10, Some(10)).generate(),
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

    fn create_render_pass(
        device: Arc<Device>,
        swapchain: Arc<Swapchain<Window>>,
    ) -> Arc<RenderPass> {
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
        )
        .unwrap()
    }

    fn create_pipeline(
        device: Arc<Device>,
        render_pass: Arc<RenderPass>,
        vertex_shader: Arc<ShaderModule>,
        fragment_shader: Arc<ShaderModule>,
    ) -> Arc<GraphicsPipeline> {
        GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().instance::<Tile>())
            .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
            .input_assembly_state(
                InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip),
            )
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
            .render_pass(Subpass::from(render_pass, 0).unwrap())
            .build(device.clone())
            .unwrap()
    }

    fn window_size_dependent_setup(
        images: &[Arc<SwapchainImage<Window>>],
        viewport: &mut Viewport,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];
    
        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! create_texture {
        ($queue:expr, $( $texture:expr ), +) => {
            {
                let mut layer_count = 0;
                $(
                   $texture;
                    layer_count += 1;
                )* 
                let image_array: Vec<_> = vec![
                    $(include_bytes!($texture).to_vec()),*
                ]
                .iter()
                .flat_map(|png_bytes| {
                    let cursor = Cursor::new(png_bytes);
                    let decoder = png::Decoder::new(cursor);
                    let mut reader = decoder.read_info().unwrap();
                    let info = reader.info();
                    let mut image_data = Vec::new();
                    image_data.resize((info.width * info.height * 4) as usize, 0);
                    reader.next_frame(&mut image_data).unwrap();
                    image_data
                })
                .collect();
                let dimensions = ImageDimensions::Dim2d {
                    width: 64,
                    height: 64,
                    array_layers: layer_count,
                };
                let (image, future) = ImmutableImage::from_iter(
                    image_array,
                    dimensions,
                    vulkano::image::MipmapsCount::Log2,
                    Format::R8G8B8A8_SRGB,
                    $queue,
                )
                .unwrap();
                (ImageView::new_default(image).unwrap(), future)

            }
        }
}

mod tile_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/tile_vertex_shader.vert"
    }
}

mod tile_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/tile_fragment_shader.frag"
    }
}
