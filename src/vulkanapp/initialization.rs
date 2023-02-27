use std::io::Cursor;
use std::{ops::Deref, sync::Arc};
use vulkano::buffer::{BufferUsage, DeviceLocalBuffer};
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::instance::InstanceCreateInfo;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::sampler::{Sampler, SamplerCreateInfo};
use vulkano::sync::GpuFuture;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
    },
    format::Format,
    image::{view::ImageView, ImageDimensions, ImageUsage, ImmutableImage, SwapchainImage},
    instance::Instance,
    pipeline::{
        graphics::{
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        GraphicsPipeline, Pipeline,
    },
    render_pass::{RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};
use vulkano::{Version, VulkanLibrary};
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use crate::camera::Camera;
use crate::engine::vector2::{Vector2, Convert};
use crate::input::Input;
use crate::map::Map;

use super::gpustoredinstances::{GpuStoredGameObject, GpuStoredHUDObject};
use super::shaders;
use super::VulkanApp;
use crate::create_texture;

impl VulkanApp {
    pub fn init() -> (Self, EventLoop<()>) {
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
            query_physical_device(instance, &device_extensions, &surface);

        let (device, graphics_queue) =
            create_device(physical.clone(), device_extensions, queue_family_index);

        let (swapchain, swapchain_images) = create_swapchain(device.clone(), surface.clone());

        let tile_vertex_shader = shaders::tile_vertex_shader::load(device.clone()).unwrap();
        let general_fragment_shader =
            shaders::general_fragment_shader::load(device.clone()).unwrap();

        let render_pass = create_render_pass(device.clone(), swapchain.clone());

        let gameobject_pipeline = create_pipeline::<GpuStoredGameObject>(
            device.clone(),
            render_pass.clone(),
            tile_vertex_shader,
            general_fragment_shader.clone(),
        );

        let hud_vertex_shader = shaders::hud_vertex_shader::load(device.clone()).unwrap();

        let hud_pipeline = create_pipeline::<GpuStoredHUDObject>(
            device.clone(),
            render_pass.clone(),
            hud_vertex_shader,
            general_fragment_shader,
        );

        let (tile_textures, tile_texture_future) = create_texture!(
            graphics_queue.clone(),
            64, 64,
            "../../Assets/debug_tiles/0.png",           //0
            "../../Assets/debug_tiles/1_br.png",        //1
            "../../Assets/debug_tiles/1_bl.png",        //2
            "../../Assets/debug_tiles/2_bl_br.png",     //3
            "../../Assets/debug_tiles/1_tl.png",        //4
            "../../Assets/debug_tiles/2_tl_br.png",     //5
            "../../Assets/debug_tiles/2_tl_bl.png",     //6
            "../../Assets/debug_tiles/3_tl_bl_br.png",  //7
            "../../Assets/debug_tiles/1_tr.png",        //8
            "../../Assets/debug_tiles/2_br_tr.png",     //9
            "../../Assets/debug_tiles/2_bl_tr.png",     //10
            "../../Assets/debug_tiles/3_bl_br_tr.png",  //11
            "../../Assets/debug_tiles/2_tl_tr.png",     //12
            "../../Assets/debug_tiles/3_tl_br_tr.png",  //13
            "../../Assets/debug_tiles/3_tl_bl_tr.png",  //14
            "../../Assets/debug_tiles/4.png"           //15
        );
        let (building_textures, building_texture_future) = create_texture!(
            graphics_queue.clone(),
            64, 64,
            "../../Assets/debug_buildings/basic.png",
            "../../Assets/debug_buildings/basic.png"
        );
        let (troop_textures, troop_texture_future) = create_texture!(
            graphics_queue.clone(),
            64, 64,
            "../../Assets/debug_troops/basic.png",
            "../../Assets/debug_troops/basic.png"
        );
        let (hud_textures, hud_texture_future) = create_texture!(
            graphics_queue.clone(),
            20, 20,
            "../../Assets/hud/Background.png",
            "../../Assets/hud/Create.png",
            "../../Assets/hud/Destroy.png"
        );

        let sampler = Sampler::new(
            device.clone(),
            SamplerCreateInfo {
                mag_filter: vulkano::sampler::Filter::Nearest,
                min_filter: vulkano::sampler::Filter::Nearest,
                mipmap_mode: vulkano::sampler::SamplerMipmapMode::Nearest,
                ..SamplerCreateInfo::simple_repeat_linear()
            },
        )
        .unwrap();

        let gameobject_pipeline_descriptor_layout =
            gameobject_pipeline.layout().set_layouts().get(0).unwrap();
        let tile_texture_descriptor_set = PersistentDescriptorSet::new(
            gameobject_pipeline_descriptor_layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                tile_textures,
                sampler.clone(),
            )],
        )
        .unwrap();
        let building_texture_descriptor_set = PersistentDescriptorSet::new(
            gameobject_pipeline_descriptor_layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                building_textures,
                sampler.clone(),
            )],
        )
        .unwrap();
        let troop_texture_descriptor_set = PersistentDescriptorSet::new(
            gameobject_pipeline_descriptor_layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                troop_textures,
                sampler.clone(),
            )],
        )
        .unwrap();

        let hud_pipeline_descriptor_layout = hud_pipeline.layout().set_layouts().get(0).unwrap();
        let hud_texture_descriptor_set = PersistentDescriptorSet::new(
            hud_pipeline_descriptor_layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                hud_textures,
                sampler.clone(),
            )],
        )
        .unwrap();

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };

        let framebuffers = Self::window_size_dependent_setup(
            device.clone(),
            &swapchain_images,
            &mut viewport,
            render_pass.clone(),
        );

        let recreate_swapchain = false;

        let mapsize = 128;
        let mut map = Map::new(mapsize, 20);
        map.generate(None);
        //map.from_bmp("mask1.bmp");
        //map.generate_automata(0.7, 17);
        //println!("{}", map);

        let mut camera = Camera::new(surface.window().inner_size().into());
        camera.snap_to_tile(Vector2::new(mapsize / 2, mapsize / 2).convert());

        let (device_local_tile_instance_buffer, tile_copy_future) =
            Self::create_device_local_buffer(
                device.clone(),
                graphics_queue.clone(),
                map.get_tile_instance_coordinates(),
            );
        let device_local_building_instance_buffer =
            DeviceLocalBuffer::<[GpuStoredGameObject]>::array(
                device.clone(),
                1,
                BufferUsage {
                    vertex_buffer: true,
                    transfer_src: true,
                    ..Default::default()
                },
                [graphics_queue.queue_family_index()],
            )
            .unwrap();
        let device_local_troop_instance_buffer =
            DeviceLocalBuffer::<[GpuStoredGameObject]>::array(
                device.clone(),
                1,
                BufferUsage {
                    vertex_buffer: true,
                    transfer_src: true,
                    ..Default::default()
                },
                [graphics_queue.queue_family_index()],
            )
            .unwrap();
        let (device_local_hud_instance_buffer, hud_copy_future) = Self::create_device_local_buffer(
            device.clone(),
            graphics_queue.clone(),
            camera.get_hud_instance_coordinates(),
        );

        let previous_frame_end = Some(
            tile_texture_future
                .join(building_texture_future)
                .join(troop_texture_future)
                .join(hud_texture_future)
                .join(tile_copy_future)
                .join(hud_copy_future)
                .boxed(),
        );
        let vulkan_app = Self {
            surface,
            device,
            graphics_queue,
            render_pass,
            clear_values: vec![Some([0.0, 0.68, 1.0, 1.0].into()), Some(1f32.into())],
            swapchain,
            recreate_swapchain,
            framebuffers,
            draw_image_index: 0,
            gameobject_pipeline,
            hud_pipeline,
            tile_texture_descriptor_set,
            building_texture_descriptor_set,
            troop_texture_descriptor_set,
            hud_texture_descriptor_set,
            viewport,
            previous_frame_end,
            device_local_tile_instance_buffer,
            device_local_building_instance_buffer,
            device_local_troop_instance_buffer,
            device_local_hud_instance_buffer,
            building_instance_count: 0,
            troop_instance_count: 0,
            hud_instance_count: 1,
            map,
            input: Input::init(),
            camera,
        };
        (vulkan_app, event_loop)
    }
}

pub fn query_physical_device(
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

pub fn create_device(
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

pub fn create_swapchain(
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
            present_mode: vulkano::swapchain::PresentMode::Immediate,
            ..Default::default()
        },
    )
    .unwrap()
}

pub fn create_render_pass(
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
            },
            depth: {
                load: Clear,
                store: DontCare,
                format: Format::D16_UNORM,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {depth}
        }
    )
    .unwrap()
}

pub fn create_pipeline<InstanceType: Vertex>(
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
) -> Arc<GraphicsPipeline> {
    let subpass = Subpass::from(render_pass, 0).unwrap();

    //let mut depth_stencil_state = DepthStencilState::simple_depth_test();
    //depth_stencil_state.depth.unwrap().compare_op = StateMode::Fixed(CompareOp::Never);

    GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().instance::<InstanceType>())
        .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip))
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
        //.depth_stencil_state(depth_stencil_state)
        .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
        .render_pass(subpass)
        .build(device.clone())
        .unwrap()
}

#[macro_export]
macro_rules! create_texture {
        ($queue:expr, $width:expr, $height:expr, $( $texture:expr ), +) => {
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
                    width: $width,
                    height: $height,
                    array_layers: layer_count,
                };
                let (image, future) = ImmutableImage::from_iter(
                    image_array,
                    dimensions,
                    vulkano::image::MipmapsCount::One,
                    Format::R8G8B8A8_SRGB,
                    $queue,
                )
                .unwrap();
                (ImageView::new_default(image).unwrap(), future)

            }
        }
}
