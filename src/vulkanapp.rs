use std::io::Cursor;
use std::string::FromUtf8Error;
use std::{ops::Deref, sync::Arc};

use crate::camera::Camera;
use crate::create_texture;
use crate::input::Input;
use crate::map::tile::{GpuStoredTile, Tile};
use crate::map::Map;
use bytemuck::Pod;
use vulkano::buffer::{BufferContents, BufferUsage, CpuAccessibleBuffer, DeviceLocalBuffer};
use vulkano::command_buffer::pool::standard::StandardCommandPoolAlloc;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferExecFuture, CopyBufferInfo, PrimaryAutoCommandBuffer,
    RenderPassBeginInfo,
};
use vulkano::descriptor_set::pool::standard::StandardDescriptorPoolAlloc;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::format::ClearValue;
use vulkano::image::ImageAccess;
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::viewport::Viewport;
use vulkano::pipeline::Pipeline;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};
use vulkano::sampler::{Sampler, SamplerCreateInfo};
use vulkano::swapchain::{PresentInfo, SwapchainAcquireFuture, SwapchainCreationError};
use vulkano::sync::{self, FenceSignalFuture, FlushError, GpuFuture, NowFuture};
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
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
    framebuffers: Vec<Arc<Framebuffer>>,
    draw_image_index: usize, //The index of the image the GPU is drawing on.
    viewport: Viewport,
    render_pass: Arc<RenderPass>,
    clear_values: [f32; 4],
    graphics_pipeline: Arc<GraphicsPipeline>,
    graphics_descriptor_set: Arc<PersistentDescriptorSet<StandardDescriptorPoolAlloc>>,
    pub recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    device_local_tile_instance_buffer: Arc<DeviceLocalBuffer<[[f32; 2]]>>,
    //END OF VULKAN VARIABLES
    //END OF VULKAN VARIABLES
    pub input: Input,
    map: Map,
    pub camera: Camera,
}

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
            Self::query_physical_device(instance, &device_extensions, &surface);

        let (device, graphics_queue) =
            Self::create_device(physical.clone(), device_extensions, queue_family_index);

        let (swapchain, swapchain_images) = Self::create_swapchain(device.clone(), surface.clone());

        let tile_vertex_shader = tile_vertex_shader::load(device.clone()).unwrap();
        let tile_fragment_shader = tile_fragment_shader::load(device.clone()).unwrap();

        let render_pass = Self::create_render_pass(device.clone(), swapchain.clone());

        let graphics_pipeline = Self::create_pipeline(
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

        let layout = graphics_pipeline.layout().set_layouts().get(0).unwrap();
        let graphics_descriptor_set = PersistentDescriptorSet::new(
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

        let framebuffers = Self::window_size_dependent_setup(
            &swapchain_images,
            &mut viewport,
            render_pass.clone(),
        );
        let recreate_swapchain = false;

        let map = Map::new(2000, 10).generate_automata(1.0);
        let instances = map.get_tile_coordinates();

        let camera = Camera::new(surface.window().inner_size().into());

        let (device_local_tile_instance_buffer, copy_future) =
            Self::create_device_local_buffer(device.clone(), graphics_queue.clone(), instances);

        let previous_frame_end = Some(texture_future.join(copy_future).boxed());

        (
            Self {
                surface,
                device,
                graphics_queue,
                render_pass,
                clear_values: [0.0, 0.68, 1.0, 1.0],
                swapchain,
                swapchain_images,
                recreate_swapchain,
                framebuffers,
                draw_image_index: 0,
                graphics_pipeline,
                graphics_descriptor_set,
                viewport,
                previous_frame_end,
                device_local_tile_instance_buffer,
                input: Input::init((800, 600)),
                map,
                camera,
            },
            event_loop,
        )
    }

    pub fn render(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.recreate_swapchain {
            self.recreate_swapchain();
        }

        let draw_image_future = match self.acquire_swapchain_image() {
            Some(future) => future,
            None => return,
        };

        let push_constants = tile_vertex_shader::ty::Camera {
            _dummy0: [0, 0, 0, 0],
            coordinates: self.camera.coordinates.into(),
            tile_size: self.camera.tile_size as u32,
            size: self.camera.camera_size.into(),
        };

        let mut cmd_buffer_builder = self.create_cmd_buffer_builder();

        cmd_buffer_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(self.clear_values.into())],
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[self.draw_image_index].clone(),
                    )
                },
                vulkano::command_buffer::SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()])
            .bind_pipeline_graphics(self.graphics_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                self.graphics_pipeline.layout().clone(),
                0,
                self.graphics_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.device_local_tile_instance_buffer.clone())
            .push_constants(self.graphics_pipeline.layout().clone(), 0, push_constants)
            .draw(4, self.map.num_of_vulkan_instances, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();

        let cmd_buffer = cmd_buffer_builder.build().unwrap();

        let render_future = self
            .previous_frame_end
            .take()
            .unwrap()
            .join(draw_image_future)
            .then_execute(self.graphics_queue.clone(), cmd_buffer)
            .unwrap()
            .then_swapchain_present(
                self.graphics_queue.clone(),
                PresentInfo {
                    index: self.draw_image_index,
                    ..PresentInfo::swapchain(self.swapchain.clone())
                },
            )
            .then_signal_fence_and_flush();

        match render_future {
            Ok(future) => {
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }

    pub fn refresh_game(&mut self) {
        self.camera.refresh_camera(&self.input);
        self.input.refresh_input();
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
                present_mode: vulkano::swapchain::PresentMode::Mailbox,
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
        let subpass = Subpass::from(render_pass, 0).unwrap();
        GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().instance::<GpuStoredTile>())
            .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
            .input_assembly_state(
                InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip),
            )
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
            .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
            .render_pass(subpass)
            .build(device.clone())
            .unwrap()
    }

    fn create_cmd_buffer_builder(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        AutoCommandBufferBuilder::primary(
            self.device.clone(),
            self.graphics_queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap()
    }

    fn create_device_local_buffer<T>(
        device: Arc<Device>,
        queue: Arc<Queue>,
        data: Vec<T>,
    ) -> (
        Arc<DeviceLocalBuffer<[T]>>,
        FenceSignalFuture<
            CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer<StandardCommandPoolAlloc>>,
        >,
    )
    where
        T: Pod + BufferContents,
    {
        let data_len = data.len() as u64;
        let cpu_accessible_buffer = CpuAccessibleBuffer::from_iter(
            device.clone(),
            BufferUsage {
                vertex_buffer: true,
                transfer_src: true,
                ..Default::default()
            },
            false,
            data,
        )
        .unwrap();

        let device_local_buffer = DeviceLocalBuffer::array(
            device.clone(),
            data_len,
            BufferUsage {
                vertex_buffer: true,
                transfer_dst: true,
                ..Default::default()
            },
            [queue.queue_family_index()],
        )
        .unwrap();

        let mut cmd_buffer_builder = AutoCommandBufferBuilder::primary(
            device.clone(),
            queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        cmd_buffer_builder
            .copy_buffer(CopyBufferInfo::buffers(
                cpu_accessible_buffer,
                device_local_buffer.clone(),
            ))
            .unwrap();

        let cmd_buffer = cmd_buffer_builder.build().unwrap();

        let copy_future = sync::now(device)
            .then_execute(queue.clone(), cmd_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        (device_local_buffer, copy_future)
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

    fn recreate_swapchain(&mut self) {
        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: self.surface.window().inner_size().into(),
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };
        self.swapchain = new_swapchain;
        self.framebuffers = Self::window_size_dependent_setup(
            &new_images,
            &mut self.viewport,
            self.render_pass.clone(),
        );
        self.recreate_swapchain = false;
    }

    fn acquire_swapchain_image(&mut self) -> Option<SwapchainAcquireFuture<Window>> {
        let (image_num, suboptimal, acquire_future) =
            match vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(vulkano::swapchain::AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return None;
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };
        if suboptimal {
            self.recreate_swapchain = true;
        }
        self.draw_image_index = image_num;
        Some(acquire_future)
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
        path: "shaders/tile_vertex_shader.vert",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        }
    }
}

mod tile_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/tile_fragment_shader.frag"
    }
}
