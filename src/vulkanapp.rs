use std::io::Cursor;
use std::{ops::Deref, sync::Arc};

use crate::camera::Camera;
use crate::create_texture;
use crate::engine::vector2::Vector2;
use crate::gpustoredinstances::{GpuStoredGameObject, GpuStoredHUDObject};
use crate::input::Input;
use crate::map::tile::TileFlag;
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
use vulkano::image::{AttachmentImage, ImageAccess};
use vulkano::pipeline::graphics::color_blend::ColorBlendState;
use vulkano::pipeline::graphics::vertex_input::Vertex;
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
    //END OF VULKAN VARIABLES
    //END OF VULKAN VARIABLES
    pub input: Input,
    pub map: Map,
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
        let general_fragment_shader = general_fragment_shader::load(device.clone()).unwrap();

        let render_pass = Self::create_render_pass(device.clone(), swapchain.clone());

        let gameobject_pipeline = Self::create_pipeline::<GpuStoredGameObject>(
            device.clone(),
            render_pass.clone(),
            tile_vertex_shader,
            general_fragment_shader.clone(),
        );

        let hud_vertex_shader = hud_vertex_shader::load(device.clone()).unwrap();

        let hud_pipeline = Self::create_pipeline::<GpuStoredHUDObject>(
            device.clone(),
            render_pass.clone(),
            hud_vertex_shader,
            general_fragment_shader,
        );

        let (tile_textures, tile_texture_future) = create_texture!(
            graphics_queue.clone(),
            "../Assets/debug_tiles/0.png",          //0
            "../Assets/debug_tiles/1_br.png",       //1
            "../Assets/debug_tiles/1_bl.png",       //2
            "../Assets/debug_tiles/2_bl_br.png",    //3
            "../Assets/debug_tiles/1_tl.png",       //4
            "../Assets/debug_tiles/2_tl_br.png",    //5
            "../Assets/debug_tiles/2_tl_bl.png",    //6
            "../Assets/debug_tiles/3_tl_bl_br.png", //7
            "../Assets/debug_tiles/1_tr.png",       //8
            "../Assets/debug_tiles/2_br_tr.png",    //9
            "../Assets/debug_tiles/2_bl_tr.png",    //10
            "../Assets/debug_tiles/3_bl_br_tr.png", //11
            "../Assets/debug_tiles/2_tl_tr.png",    //12
            "../Assets/debug_tiles/3_tl_br_tr.png", //13
            "../Assets/debug_tiles/3_tl_bl_tr.png", //14
            "../Assets/debug_tiles/4.png"           //15
        );
        let (building_textures, building_texture_future) = create_texture!(
            graphics_queue.clone(),
            "../Assets/debug_buildings/basic.png",
            "../Assets/debug_buildings/basic.png"
        );
        let (hud_textures, hud_texture_future) = create_texture!(
            graphics_queue.clone(),
            "../Assets/hud/Debug.png",
            "../Assets/hud/Debug.png"
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

        let mapsize = 800;
        let mut map = Map::new(mapsize, 8);
        map.generate(None);
        //map.generate_automata(0.7, 17);
        //println!("{}", map);

        let mut camera = Camera::new(surface.window().inner_size().into());
        camera.snap_to_tile(Vector2::new_usize(mapsize / 2, mapsize / 2));

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
            ).unwrap();
        let (device_local_hud_instance_buffer, hud_copy_future) = Self::create_device_local_buffer(
            device.clone(),
            graphics_queue.clone(),
            camera.get_hud_instance_coordinates(),
        );

        let previous_frame_end = Some(
            tile_texture_future
                .join(building_texture_future)
                .join(tile_copy_future)
                .join(hud_copy_future)
                .join(hud_texture_future)
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
            hud_texture_descriptor_set,
            viewport,
            previous_frame_end,
            device_local_tile_instance_buffer,
            device_local_building_instance_buffer,
            device_local_hud_instance_buffer,
            building_instance_count: 0,
            map,
            input: Input::init(),
            camera,
        };
        (vulkan_app, event_loop)
    }

    pub fn render(&mut self) {
        if self.recreate_swapchain {
            self.recreate_swapchain();
        }

        let draw_image_future = match self.acquire_swapchain_image() {
            Some(future) => future,
            None => return,
        };

        let push_constants = tile_vertex_shader::ty::Camera {
            coordinates: self.camera.coordinates.into(),
            tile_size: (2.0 / self.camera.tiles_fit).into(),
            size: self.camera.camera_size.into(),
        };

        let mut cmd_buffer_builder = self.create_cmd_buffer_builder();
        cmd_buffer_builder
            //General setup
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: self.clear_values.clone(),
                    ..RenderPassBeginInfo::framebuffer(
                        self.framebuffers[self.draw_image_index].clone(),
                    )
                },
                vulkano::command_buffer::SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.clone()])
            .push_constants(self.gameobject_pipeline.layout().clone(), 0, push_constants)
            //Tile rendering
            .bind_pipeline_graphics(self.gameobject_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                self.gameobject_pipeline.layout().clone(),
                0,
                self.tile_texture_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.device_local_tile_instance_buffer.clone())
            .draw(4, self.map.num_of_vulkan_instances, 0, 0)
            .unwrap();
        //Building rendering, pipeline is the same.
        if self.building_instance_count > 0 {
            cmd_buffer_builder
                .bind_vertex_buffers(0, self.device_local_building_instance_buffer.clone())
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    self.gameobject_pipeline.layout().clone(),
                    0,
                    self.building_texture_descriptor_set.clone(),
                )
                .draw(4, self.building_instance_count as u32, 0, 0)
                .unwrap();
        }

        //HUD rendering
        cmd_buffer_builder
            .bind_pipeline_graphics(self.hud_pipeline.clone())
            .bind_descriptor_sets(
                vulkano::pipeline::PipelineBindPoint::Graphics,
                self.hud_pipeline.layout().clone(),
                0,
                self.hud_texture_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.device_local_hud_instance_buffer.clone())
            .draw(4, 1, 0, 0)
            .unwrap();

        cmd_buffer_builder.end_render_pass().unwrap();

        let cmd_buffer = cmd_buffer_builder.build().unwrap();

        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

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

    pub fn refresh_game(&mut self, delta_time: f32) {
        self.process_input_commands();

        self.camera.refresh_camera(&self.input, delta_time);
        self.input.refresh_input();
    }

    fn process_input_commands(&mut self) {
        if self
            .input
            .get_mousebutton_pressed(winit::event::MouseButton::Left)
        {
            let mouse_position = self.input.get_mouse_position();
            let mouse_coordinates = self
                .camera
                .screen_position_to_tile_coordinates(mouse_position);
            if let Some(hud_object) = self
                .camera
                .get_hud_object_at_screen_position(mouse_position)
            {
                //do stuff with hud
            } else {
                if let Some(clicked_tile) =
                    self.map.get_shown_tile_at_coordinates(mouse_coordinates)
                {
                    //No building on top
                    if clicked_tile.flags & TileFlag::BuildingOnTop as u8
                        != TileFlag::BuildingOnTop as u8
                    {
                        self.map.build_building(clicked_tile.coordinates.into(), 0);
                        self.copy_into_building_buffer();
                    }
                    //Has building on top
                    else {
                        self.map.destroy_building(clicked_tile.coordinates.into());
                        self.copy_into_building_buffer();
                    }
                } else {
                }
            }
        }
        if self.input.get_key_pressed(winit::event::VirtualKeyCode::E) {
            self.camera.toggle_hud_visibility(0)
        }
    }

    fn copy_into_building_buffer(&mut self) {
        let gpu_stored_building_vector = self.map.get_building_instance_coordinates();
        self.building_instance_count = self.map.building_vector.len() as u16;
        if self.building_instance_count == 0 {
            return;
        }
        (self.device_local_building_instance_buffer, _) = Self::create_device_local_buffer(
            self.device.clone(),
            self.graphics_queue.clone(),
            gpu_stored_building_vector,
        );
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
                present_mode: vulkano::swapchain::PresentMode::Immediate,
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

    fn create_pipeline<InstanceType: Vertex>(
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
            .input_assembly_state(
                InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip),
            )
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
            //.depth_stencil_state(depth_stencil_state)
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
        device: Arc<Device>,
        images: &[Arc<SwapchainImage<Window>>],
        viewport: &mut Viewport,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        let depth_buffer = ImageView::new_default(
            AttachmentImage::transient(device.clone(), dimensions, Format::D16_UNORM).unwrap(),
        )
        .unwrap();

        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view, depth_buffer.clone()],
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
            self.device.clone(),
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
                    vulkano::image::MipmapsCount::One,
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

mod hud_vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/hud_vertex_shader.vert"
    }
}

mod general_fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/general_fragment_shader.frag"
    }
}
