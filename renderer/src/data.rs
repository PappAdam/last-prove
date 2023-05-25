use nalgebra::{Matrix4, Vector3};

use crate::{
    create_shader,
    resources::{
        self,
        buffer::{Buffer, DynamicUniformBuffer, UniformBuffer},
        desriptors::{
            create_descriptor_pool, create_descriptor_set_layout, create_descriptor_sets,
        },
        image::Image,
    },
    setup,
    utils::{
        buffer_data::{BufferObject, PushConst, WorldView},
        MAX_WORLD_OBJECTS,
    },
};
use ash::vk;

use super::{base::RenderBase, utils::MAX_FRAME_DRAWS};

pub struct RenderData {
    pub pipeline_layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
    pub pipeline: vk::Pipeline,
    pub viewport: vk::Viewport,
    pub scissor: vk::Rect2D,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub img_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub fences: Vec<vk::Fence>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub depth_img: Image,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub descriptor_set_layout: vk::DescriptorSetLayout,

    //Buffer content
    pub world_view: WorldView,
    pub push_const: PushConst,

    //Buffers
    pub uniform_buffer: UniformBuffer,
    pub dynamic_uniform_buffer: DynamicUniformBuffer,
}

impl RenderData {
    pub fn new(base: &mut RenderBase) -> Result<Self, String> {
        let vertex_shader_module = create_shader!("../.compiled_shaders/vert.spv", base.device);
        let fragment_shader_module = create_shader!("../.compiled_shaders/frag.spv", base.device);

        let descriptor_set_layout = create_descriptor_set_layout(&base.device)?;
        let pipeline_layout = setup::create_pipeline_layout(&base.device, descriptor_set_layout)?;

        let render_pass =
            setup::create_render_pass(&base.device, base.surface_format.format, base.depth_format)?;

        let pipeline = setup::create_pipelines(
            &base.device,
            vertex_shader_module,
            fragment_shader_module,
            pipeline_layout,
            render_pass,
        )?;

        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: base.surface_extent.width as f32,
            height: base.surface_extent.height as f32,
            min_depth: 0.0f32,
            max_depth: 1.0f32,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: base.surface_extent.width,
                height: base.surface_extent.height,
            },
        };

        let depth_img = Image::new(
            &base.device,
            base.surface_extent.into(),
            base.depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::ImageAspectFlags::DEPTH,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            base.physical_device_memory_properties,
        )?;

        let framebuffers = resources::create_framebuffers(
            &base.device,
            &base.swapchain_image_views,
            render_pass,
            base.surface_extent,
            depth_img.view,
        )?;

        let img_available_semaphores =
            resources::create_semaphore(&base.device, "img available semaphore")?;

        let render_finished_semaphores =
            resources::create_semaphore(&base.device, "rendering finished semaphore")?;

        let fences = resources::create_fences(&base.device)?;
        let command_pool = setup::create_command_pool(&base.device, base.queue_family)?;

        let command_buffers = {
            unsafe {
                let cb_info = vk::CommandBufferAllocateInfo::builder()
                    .command_pool(command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(MAX_FRAME_DRAWS as u32)
                    .build();
                base.device
                    .allocate_command_buffers(&cb_info)
                    .map_err(|err| format!("{}", err))?
            }
        };

        let descriptor_pool = create_descriptor_pool(&base.device)?;

        let world_view = WorldView::new();
        let uniform_buffer = Buffer::uniform_buffer::<WorldView>(
            &base.device,
            world_view.as_void_ptr(),
            base.physical_device_memory_properties,
            0,
        )?;

        let dynamic_uniform_buffer = Buffer::dynamic_uniform_buffer::<Matrix4<f32>>(
            &base.device,
            base.physical_device_memory_properties,
            base.physical_device_properties,
            MAX_WORLD_OBJECTS,
            1,
        )?;

        let descriptor_sets = create_descriptor_sets(
            &base.device,
            descriptor_pool,
            descriptor_set_layout,
            &dynamic_uniform_buffer,
            &uniform_buffer,
        )?;

        uniform_buffer.update(&base.device, world_view.as_void_ptr(), &descriptor_sets);

        Ok(Self {
            pipeline_layout,
            render_pass,
            pipeline,
            viewport,
            scissor,
            framebuffers,
            img_available_semaphores,
            render_finished_semaphores,
            fences,
            command_pool,
            command_buffers,
            depth_img,
            descriptor_pool,
            descriptor_set_layout,
            descriptor_sets,

            world_view,

            //Buffers
            uniform_buffer,
            dynamic_uniform_buffer,
            push_const: PushConst::default(),
        })
    }

    #[inline]
    pub fn resize(&mut self, base: &RenderBase) -> Result<(), String> {
        unsafe {
            for &framebuffer in &self.framebuffers {
                base.device.destroy_framebuffer(framebuffer, None);
            }
        }

        self.depth_img.free(&base.device);

        self.depth_img = Image::new(
            &base.device,
            base.surface_extent.into(),
            base.depth_format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::ImageAspectFlags::DEPTH,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            base.physical_device_memory_properties,
        )?;

        self.framebuffers = resources::create_framebuffers(
            &base.device,
            &base.swapchain_image_views,
            self.render_pass,
            base.surface_extent,
            self.depth_img.view,
        )?;

        Ok(())
    }

    pub fn clean_up(&self, device: &ash::Device) {
        unsafe {
            self.dynamic_uniform_buffer.free(device);
            self.uniform_buffer.free(device);
            self.depth_img.free(device);

            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);

            device.destroy_pipeline_layout(self.pipeline_layout, None);

            device.destroy_render_pass(self.render_pass, None);

            device.destroy_pipeline(self.pipeline, None);

            for &framebuffer in &self.framebuffers {
                device.destroy_framebuffer(framebuffer, None);
            }

            for i in 0..MAX_FRAME_DRAWS {
                device.destroy_semaphore(self.img_available_semaphores[i], None);
                device.destroy_semaphore(self.render_finished_semaphores[i], None);
                device.destroy_fence(self.fences[i], None);
            }
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
